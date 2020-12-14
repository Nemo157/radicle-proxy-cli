use crate::api::Error;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
crate struct Project {
    crate urn: String,
    crate shareable_entity_identifier: String,
    crate metadata: Metadata,
    crate stats: Stats,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
crate struct Metadata {
    crate name: String,
    crate description: String,
    crate default_branch: String,
    crate maintainers: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
crate struct Stats {
    crate commits: u64,
    crate branches: u64,
    crate contributors: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
crate struct Peer {
    crate peer_id: String,
    crate status: PeerStatus,
    crate type_: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
crate struct PeerStatus {
    crate role: String,
    crate type_: String,
    crate user: crate::api::identities::Identity,
}

crate struct Api<'a> {
    agent: &'a crate::api::Agent,
}

impl<'a> Api<'a> {
    pub(super) fn new(agent: &'a crate::api::Agent) -> Self {
        Self { agent }
    }

    #[fehler::throws]
    #[tracing::instrument(skip(self))]
    crate fn tracked(&self) -> Vec<Project> {
        self.agent.get(["v1", "projects", "tracked"])?
    }

    #[fehler::throws]
    #[tracing::instrument(skip(self))]
    crate fn contributed(&self) -> Vec<Project> {
        self.agent.get(["v1", "projects", "contributed"])?
    }

    #[fehler::throws]
    #[tracing::instrument(skip(self))]
    crate fn get(&self, urn: &str) -> Option<Project> {
        self.agent.get_opt(["v1", "projects", urn])?
    }

    #[fehler::throws]
    #[tracing::instrument(skip(self))]
    crate fn peers(&self, urn: &str) -> Vec<Peer> {
        self.agent.get(["v1", "projects", urn, "peers"])?
    }
}
