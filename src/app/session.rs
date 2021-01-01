use crate::app::WithContext;
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

impl WithContext<App> {
    #[fehler::throws]
    pub(super) fn run(self) {
        self.map(|app| app.cmd).run()?
    }
}

impl WithContext<Cmd> {
    #[fehler::throws]
    pub(super) fn run(self) {
        match self.as_ref() {
            Cmd::Get => {
                let session = self.api().session().get()?;
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
