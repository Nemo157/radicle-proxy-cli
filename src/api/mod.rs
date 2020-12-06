use secrecy::Secret;
use url::Url;

mod agent;
crate mod identities;

use agent::Agent;

#[derive(Debug, thiserror::Error)]
crate enum Error {
    #[error("API response {code}, {msg}")]
    Api { msg: String, code: u16 },

    #[error("Internal error constructing url")]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    UnknownUreq(#[from] ureq::Error),

    #[error(transparent)]
    UnknownIo(#[from] std::io::Error),
}

#[derive(Debug)]
crate struct Api {
    agent: Agent,
}

impl Api {
    #[fehler::throws(anyhow::Error)]
    crate fn new(base: Url, auth_token: Secret<String>) -> Self {
        Self {
            agent: Agent::new(base, auth_token)?,
        }
    }

    crate fn identities(&self) -> identities::Api<'_> {
        identities::Api::new(&self.agent)
    }
}
