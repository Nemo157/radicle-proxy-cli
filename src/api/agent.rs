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
        f.debug_struct("Agent").field("base", &self.base.to_string()).finish()?;
    }
}

impl Agent {
    #[fehler::throws(anyhow::Error)]
    #[tracing::instrument(fields(%base))]
    pub(super) fn new(base: Url) -> Self {
        let agent = ureq::agent();
        anyhow::ensure!(
            !base.cannot_be_a_base(),
            "Invalid base url, must be able to append components"
        );
        let agent = Secret::new(UreqAgent(agent));
        Self { base, agent }
    }

    #[fehler::throws(crate::api::Error)]
    #[tracing::instrument]
    pub(super) fn login(&self, passphrase: Secret<String>) {
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

        let crate::api::Nothing =
            self.post(["v1", "keystore", "unseal"], LoginData { passphrase })?;
        // The web server resets itself after login...
        std::thread::sleep(std::time::Duration::from_millis(100));
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
        let value = response.check_error()?.into_json_deserialize()?;
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
        let value = if response.status() == 404 {
            None
        } else {
            Some(response.check_error()?.into_json_deserialize()?)
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
        let value = response.check_error()?.into_json_deserialize()?;
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
    fn check_error(self) -> Self;
}

impl ApiResponseExt for ureq::Response {
    #[fehler::throws(crate::api::Error)]
    fn check_error(self) -> Self {
        if self.error() {
            if self.synthetic() {
                fehler::throw!(self.into_synthetic_error().unwrap());
            }
            let code = self.status();
            let ErrorResponse { message: msg, .. } = self.into_json_deserialize()?;
            fehler::throw!(crate::api::Error::Api { msg, code });
        }
        self
    }
}
