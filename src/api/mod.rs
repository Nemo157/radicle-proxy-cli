use secrecy::Secret;
use url::Url;

mod agent;
crate mod identities;
crate mod projects;
crate mod session;

use agent::Agent;

#[derive(Debug, thiserror::Error)]
crate enum Error {
    #[error("API response {code}, {msg}")]
    Api { msg: String, code: u16 },

    #[error("Internal error constructing url")]
    Url(#[from] url::ParseError),

    #[error("Serializing data to send failed")]
    SerializationFailure(#[from] serde_json::Error),

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
    crate fn new(base: Url) -> Self {
        Self {
            agent: Agent::new(base)?,
        }
    }

    #[fehler::throws(anyhow::Error)]
    /// Sets the current auth token, then returns whether it's valid
    crate fn set_token(&self, auth_token: Secret<String>) -> bool {
        self.agent.set_token(auth_token)?
    }

    #[fehler::throws(anyhow::Error)]
    /// Logs in, then returns the new auth token
    crate fn login(&self, passphrase: Secret<String>) -> Secret<String> {
        self.agent.login(passphrase)?
    }

    crate fn identities(&self) -> identities::Api<'_> {
        identities::Api::new(&self.agent)
    }

    crate fn session(&self) -> session::Api<'_> {
        session::Api::new(&self.agent)
    }

    crate fn projects(&self) -> projects::Api<'_> {
        projects::Api::new(&self.agent)
    }
}

/// For API requests that return no response data
#[derive(Debug)]
struct Nothing;

impl<'de> serde::Deserialize<'de> for Nothing {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Nothing)
    }
}
