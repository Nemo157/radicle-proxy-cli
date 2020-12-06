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
}

impl App {
    #[fehler::throws]
    pub(super) fn run(self, context: &Context) {
        self.cmd.run(context)?
    }
}

impl Cmd {
    #[fehler::throws]
    pub(super) fn run(self, context: &Context) {
        match self {
            Self::List => {
                for identity in context.api.identities().list()? {
                    println!(
                        "{} {}: {}",
                        identity.avatar_fallback.emoji, identity.metadata.handle, identity.urn
                    );
                }
            }
        }
    }
}
