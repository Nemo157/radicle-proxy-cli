use crate::api::identities::Identity;
use crate::app::Context;
use anyhow::Error;

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

impl App {
    #[fehler::throws]
    pub(super) fn run(self, context: &Context) {
        self.cmd.run(context)?
    }
}

impl Cmd {
    fn print_identities_list(&self, identities: &[Identity]) {
        for identity in identities {
            println!(
                "{} {}: {}",
                identity.avatar_fallback.emoji, identity.metadata.handle, identity.peer_id
            );
        }
    }

    #[fehler::throws]
    #[tracing::instrument]
    fn find_matching_identities(&self, context: &Context, id: &str) -> Vec<Identity> {
        context
            .api
            .identities()
            .list()?
            .into_iter()
            .filter(|identity| {
                identity.urn == id || identity.metadata.handle == id || identity.peer_id == id
            })
            .collect()
    }

    #[fehler::throws]
    pub(super) fn run(self, context: &Context) {
        match &self {
            Self::List => {
                self.print_identities_list(&context.api.identities().list()?);
            }

            Self::Get { id } => match self.find_matching_identities(context, id)?.as_slice() {
                [] => {
                    println!("no identity matching '{}' found", id);
                }
                [identity] => {
                    println!("{:#?}", identity);
                }
                identities => {
                    println!(
                        "\
                            multiple identities matched '{}', \
                            please use a urn/peer_id to guarantee uniqueness:
                        ",
                        id
                    );
                    self.print_identities_list(identities);
                }
            },

            Self::This => {
                let identity = context.api.session().get()?.identity;
                println!("{:#?}", identity);
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
