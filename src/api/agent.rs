use anyhow::Context;
use url::Url;

#[derive(Debug, serde::Deserialize)]
struct ErrorResponse {
    message: String,
    variant: String,
}

#[derive(Debug)]
pub(super) struct Agent {
    base: Url,
    agent: ureq::Agent,
}

impl Agent {
    #[fehler::throws(anyhow::Error)]
    pub(super) fn new(base: Url, auth_token: String) -> Self {
        let agent = ureq::agent();
        let domain = base
            .domain()
            .context("Invalid base url, must contain a domain to attach cookie to")?
            .to_owned();
        agent.set_cookie(
            ureq::Cookie::build("auth-token", auth_token)
                .domain(domain)
                .path("/")
                .finish(),
        );
        Self { base, agent }
    }

    #[fehler::throws(crate::api::Error)]
    pub(super) fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> T {
        let url = self.base.join(path)?.to_string();
        let response = self.agent.get(&url).call();
        response.check_error()?.into_json_deserialize()?
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
