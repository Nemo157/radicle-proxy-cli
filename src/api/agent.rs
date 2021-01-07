use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use std::fmt::Debug;
use url::Url;

#[derive(Debug, serde::Deserialize)]
struct ErrorResponse {
    message: String,
    variant: String,
}

pub(super) struct Agent {
    base: Url,
    agent: Secret<UreqAgent>,
}

struct UreqAgent(ureq::Agent);

impl core::ops::Deref for UreqAgent {
    type Target = ureq::Agent;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl secrecy::Zeroize for UreqAgent {
    fn zeroize(&mut self) {
        // Not possible
    }
}

impl std::fmt::Debug for Agent {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        f.debug_struct("Agent")
            .field("base", &self.base.to_string())
            .finish()?;
    }
}

impl Agent {
    #[fehler::throws(anyhow::Error)]
    #[tracing::instrument(fields(%base))]
    pub(super) fn with_token(base: Url, auth_token: Secret<String>) -> Option<Self> {
        anyhow::ensure!(
            !base.cannot_be_a_base(),
            "Invalid base url, must be able to append components"
        );
        let domain = base
            .domain()
            .context("Invalid base url, must contain a domain to attach cookie to")?
            .to_owned();
        let mut cookies = cookie_store::CookieStore::load_json(std::io::Cursor::new("")).unwrap();
        cookies.insert_raw(
            &cookie::Cookie::build("auth-token", auth_token.expose_secret().to_owned())
                .domain(domain)
                .path("/")
                .finish(),
            &base,
        )?;
        let agent = Self {
            base,
            agent: Secret::new(UreqAgent(ureq::builder().cookie_store(cookies).build())),
        };
        match agent.get(["v1", "identities"]) {
            Ok(t) => {
                let _: ureq::SerdeValue = t;
                Some(agent)
            }
            Err(crate::api::Error::Api { code, .. }) if code == 403 => None,
            Err(err) => fehler::throw!(err),
        }
    }

    #[fehler::throws(anyhow::Error)]
    #[tracing::instrument(fields(%base))]
    pub(super) fn with_login(base: Url, passphrase: Secret<String>) -> (Self, Secret<String>) {
        #[derive(Debug, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct LoginData {
            #[serde(serialize_with = "serialize_passphrase")]
            passphrase: Secret<String>,
        }

        fn serialize_passphrase<S>(
            passphrase: &Secret<String>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::Serialize;
            passphrase.expose_secret().serialize(serializer)
        }

        anyhow::ensure!(
            !base.cannot_be_a_base(),
            "Invalid base url, must be able to append components"
        );

        let domain = base
            .domain()
            .context("Invalid base url, must contain a domain to attach cookie to")?
            .to_owned();

        let agent = Self {
            base,
            agent: Secret::new(UreqAgent(ureq::agent())),
        };

        let crate::api::Nothing =
            agent.post(["v1", "keystore", "unseal"], LoginData { passphrase })?;

        let auth_token = Secret::new(
            agent
                .agent
                .expose_secret()
                .cookie_store()
                .get(&domain, "/", "auth-token")
                .context("Missing auth token after login")?
                .value()
                .to_owned(),
        );

        // The web server resets itself after login...
        std::thread::sleep(std::time::Duration::from_millis(100));

        (agent, auth_token)
    }

    #[fehler::throws(crate::api::Error)]
    #[tracing::instrument]
    pub(super) fn get<T: serde::de::DeserializeOwned + Debug>(
        &self,
        path: impl UrlComponents + Debug,
    ) -> T {
        let url = path.append_to(self.base.clone());
        let response = self.agent.expose_secret().get(&url.to_string()).call();
        tracing::debug!(%url, ?response);
        let value = response.check_error()?.into_json()?;
        tracing::trace!(?value);
        value
    }

    #[fehler::throws(crate::api::Error)]
    #[tracing::instrument]
    pub(super) fn get_opt<T: serde::de::DeserializeOwned + Debug>(
        &self,
        path: impl UrlComponents + Debug,
    ) -> Option<T> {
        let url = path.append_to(self.base.clone());
        let response = self.agent.expose_secret().get(&url.to_string()).call();
        tracing::debug!(%url, ?response);
        let value = if let Err(ureq::Error::Status(404, _)) = response {
            None
        } else {
            Some(response.check_error()?.into_json()?)
        };
        tracing::trace!(?value);
        value
    }

    #[fehler::throws(crate::api::Error)]
    #[tracing::instrument]
    pub(super) fn post<T: serde::de::DeserializeOwned + Debug>(
        &self,
        path: impl UrlComponents + Debug,
        data: impl serde::Serialize + Debug,
    ) -> T {
        let url = path.append_to(self.base.clone());
        let response = self
            .agent
            .expose_secret()
            .post(&url.to_string())
            .send_json(ureq::serde_to_value(data)?);
        tracing::debug!(%url, ?response);
        let value = response.check_error()?.into_json()?;
        tracing::trace!(?value);
        value
    }
}

pub(super) trait UrlComponents {
    fn append_to(self, url: Url) -> Url;
}

impl UrlComponents for &str {
    fn append_to(self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("cannot_be_a_base checked in constructor")
            .push(self);
        url
    }
}

impl UrlComponents for String {
    fn append_to(self, url: Url) -> Url {
        self.as_str().append_to(url)
    }
}

impl<T> UrlComponents for &[T]
where
    for<'a> &'a T: UrlComponents,
{
    fn append_to(self, url: Url) -> Url {
        self.iter()
            .fold(url, |url, component| component.append_to(url))
    }
}

impl<T: UrlComponents, const N: usize> UrlComponents for [T; N] {
    fn append_to(self, url: Url) -> Url {
        std::array::IntoIter::new(self).fold(url, |url, component| component.append_to(url))
    }
}

impl<T: UrlComponents, U: UrlComponents> UrlComponents for (T, U) {
    fn append_to(self, url: Url) -> Url {
        self.1.append_to(self.0.append_to(url))
    }
}

trait ApiResponseExt: Sized {
    #[fehler::throws(crate::api::Error)]
    fn check_error(self) -> ureq::Response;
}

impl ApiResponseExt for Result<ureq::Response, ureq::Error> {
    #[fehler::throws(crate::api::Error)]
    fn check_error(self) -> ureq::Response {
        match self {
            Ok(response) => response,
            Err(ureq::Error::Status(code, response)) => {
                let ErrorResponse { message: msg, .. } = response.into_json()?;
                fehler::throw!(crate::api::Error::Api { msg, code });
            }
            Err(ureq::Error::Transport(transport)) => {
                fehler::throw!(Box::new(ureq::Error::Transport(transport)))
            }
        }
    }
}
