use crate::api::{Error, ErrorResponse};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
crate struct Identity {
    crate peer_id: String,
    crate urn: String,
    crate shareable_entity_identifier: String,
    crate metadata: Metadata,
    crate avatar_fallback: AvatarFallback,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
crate struct Metadata {
    crate handle: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
crate struct AvatarFallback {
    crate emoji: String,
}

#[fehler::throws]
crate fn list(agent: &ureq::Agent) -> Vec<Identity> {
    let response = agent.get("http://localhost:17246/v1/identities/").call();
    if response.error() {
        if response.synthetic() {
            fehler::throw!(response.into_synthetic_error().unwrap());
        }
        let code = response.status();
        let ErrorResponse { message: msg, .. } = response.into_json_deserialize()?;
        fehler::throw!(Error::Api { msg, code });
    }
    response.into_json_deserialize()?
}
