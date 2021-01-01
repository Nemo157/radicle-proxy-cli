use crate::api::identities::Identity;
use crate::app::WithContext;
use anyhow::Error;
use std::io::Write;

#[derive(Debug, clap::Clap)]
/// Commands related to identities
pub(super) struct App {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Clap)]
pub(super) enum Cmd {
    /// List all known identities
    List,

    /// Get details for an identity
    Get {
        /// URN, handle or peer-id for the identity
        id: String,
    },

    /// Get own identity details
    #[clap(name = "self")]
    This,
}

impl WithContext<App> {
    #[fehler::throws]
    pub(super) fn run(self) {
        self.map(|app| app.cmd).run()?
    }
}

impl WithContext<Cmd> {
    #[fehler::throws]
    fn print_identities_list(&self, identities: &[Identity]) {
        for identity in identities {
            writeln!(
                self.output(),
                "{} {}: {}",
                identity.avatar_fallback.emoji,
                identity.metadata.handle,
                identity.peer_id
            )?;
        }
    }

    #[fehler::throws]
    #[tracing::instrument]
    fn find_matching_identities(&self, id: &str) -> Vec<Identity> {
        self.api()
            .identities()
            .list()?
            .into_iter()
            .filter(|identity| {
                identity.urn == id || identity.metadata.handle == id || identity.peer_id == id
            })
            .collect()
    }

    #[fehler::throws]
    pub(super) fn run(self) {
        match self.as_ref() {
            Cmd::List => {
                self.print_identities_list(&self.api().identities().list()?)?;
            }

            Cmd::Get { id } => match self.find_matching_identities(id)?.as_slice() {
                [] => {
                    writeln!(self.output(), "no identity matching '{}' found", id)?;
                }
                [identity] => {
                    writeln!(self.output(), "{:#?}", identity)?;
                }
                identities => {
                    writeln!(
                        self.output(),
                        "\
                            multiple identities matched '{}', \
                            please use a urn/peer_id to guarantee uniqueness:
                        ",
                        id
                    )?;
                    self.print_identities_list(identities)?;
                }
            },

            Cmd::This => {
                let identity = self.api().session().get()?.identity;
                writeln!(self.output(), "{:#?}", identity)?;
            }
        }
    }
}

impl std::fmt::Display for App {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        write!(f, "identities {}", self.cmd)?;
    }
}

impl std::fmt::Display for Cmd {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        match self {
            Self::List => write!(f, "list")?,
            Self::Get { id } => write!(f, "get {:?}", id)?,
            Self::This => write!(f, "self")?,
        }
    }
}
