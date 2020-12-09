use crate::api::{identities::Identity, Error, Nothing};
use std::collections::HashSet;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
crate struct Session {
    crate identity: Identity,
    crate settings: Settings,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
crate struct Settings {
    crate appearance: Appearance,
    crate coco: Coco,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
crate struct Appearance {
    crate theme: String,
    crate hints: Hints,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
crate struct Hints {
    crate show_remote_helper: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
crate struct Coco {
    crate seeds: HashSet<String>,
}

crate struct Api<'a> {
    agent: &'a crate::api::Agent,
}

impl<'a> Api<'a> {
    pub(super) fn new(agent: &'a crate::api::Agent) -> Self {
        Self { agent }
    }

    #[fehler::throws]
    crate fn get(&self) -> Session {
        self.agent.get(["v1", "session"])?
    }

    #[fehler::throws]
    crate fn update_settings(&self, settings: Settings) {
        let Nothing = self.agent.post(["v1", "session", "settings"], settings)?;
    }
}
