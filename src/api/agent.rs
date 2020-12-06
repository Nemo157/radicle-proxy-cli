use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use url::Url;

#[derive(Debug, serde::Deserialize)]
struct ErrorResponse {
    message: String,
    variant: String,
}

#[derive(Debug)]
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

impl secrecy::DebugSecret for UreqAgent {
    fn debug_secret(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED {}]", std::any::type_name::<ureq::Agent>())
    }
}

impl Agent {
    #[fehler::throws(anyhow::Error)]
    pub(super) fn new(base: Url, auth_token: Secret<String>) -> Self {
        let agent = ureq::agent();
        anyhow::ensure!(
            !base.cannot_be_a_base(),
            "Invalid base url, must be able to append components"
        );
        let domain = base
            .domain()
            .context("Invalid base url, must contain a domain to attach cookie to")?
            .to_owned();
        agent.set_cookie(
            ureq::Cookie::build("auth-token", auth_token.expose_secret().to_owned())
                .domain(domain)
                .path("/")
                .finish(),
        );
        let agent = Secret::new(UreqAgent(agent));
        Self { base, agent }
    }

    #[fehler::throws(crate::api::Error)]
    pub(super) fn get<T: serde::de::DeserializeOwned>(&self, path: impl UrlComponents) -> T {
        let url = path.append_to(self.base.clone());
        let response = self.agent.expose_secret().get(&url.to_string()).call();
        response.check_error()?.into_json_deserialize()?
    }

    #[fehler::throws(crate::api::Error)]
    pub(super) fn get_opt<T: serde::de::DeserializeOwned>(
        &self,
        path: impl UrlComponents,
    ) -> Option<T> {
        let url = path.append_to(self.base.clone());
        let response = self.agent.expose_secret().get(&url.to_string()).call();
        if response.status() == 404 {
            None
        } else {
            Some(response.check_error()?.into_json_deserialize()?)
        }
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
