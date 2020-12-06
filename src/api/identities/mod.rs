use crate::api::Error;

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

crate struct Api<'a> {
    agent: &'a crate::api::Agent,
}

impl<'a> Api<'a> {
    pub(super) fn new(agent: &'a crate::api::Agent) -> Self {
        Self { agent }
    }

    #[fehler::throws]
    crate fn list(&self) -> Vec<Identity> {
        self.agent.get("/v1/identities/")?
    }
}
