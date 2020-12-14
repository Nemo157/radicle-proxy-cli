use crate::app::Context;
use anyhow::Error;

#[derive(Debug, clap::Clap)]
/// Commands related to the current session
pub(super) struct App {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Clap)]
pub(super) enum Cmd {
    /// Get the current session details
    Get,
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
            Self::Get => {
                let session = context.api.session().get()?;
                println!("{:#?}", session);
            }
        }
    }
}

impl std::fmt::Display for App {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        write!(f, "session {}", self.cmd)?;
    }
}

impl std::fmt::Display for Cmd {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        match self {
            Self::Get => write!(f, "get")?,
        }
    }
}
