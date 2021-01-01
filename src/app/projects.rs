use crate::app::WithContext;
use anyhow::Error;
use std::io::Write;

#[derive(Debug, clap::Clap)]
/// Commands related to projects
pub(super) struct App {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Clap)]
pub(super) enum Cmd {
    /// Get the list of tracked projects
    Tracked,

    /// Get the list of contributed projects
    Contributed,

    /// Get the list of requested projects
    Requested,

    /// Get a projects details
    Get {
        /// URN for the project
        urn: String,
    },

    /// Get the tracked peers for a project
    Peers {
        /// URN for the project
        urn: String,
    },
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
            Cmd::Tracked => {
                for project in self.api().projects().tracked()? {
                    writeln!(self.output(), "{}: {}", project.metadata.name, project.urn)?;
                }
            }

            Cmd::Contributed => {
                for project in self.api().projects().contributed()? {
                    writeln!(self.output(), "{}: {}", project.metadata.name, project.urn)?;
                }
            }

            Cmd::Requested => {
                for project in self.api().projects().requested()? {
                    writeln!(self.output(), "{}: {:?}", project.urn, project.state)?;
                }
            }

            Cmd::Get { urn } => {
                if let Some(project) = self.api().projects().get(&urn)? {
                    writeln!(self.output(), "{:#?}", project)?;
                } else {
                    writeln!(self.output(), "Project {} not found", urn)?;
                }
            }

            Cmd::Peers { urn } => {
                for peer in self.api().projects().peers(&urn)? {
                    writeln!(
                        self.output(),
                        "{} ({}): {}",
                        peer.status.user.metadata.handle,
                        peer.peer_id,
                        peer.status.role
                    )?;
                }
            }
        }
    }
}

impl std::fmt::Display for App {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        write!(f, "projects {}", self.cmd)?;
    }
}

impl std::fmt::Display for Cmd {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        match self {
            Self::Tracked => write!(f, "tracked")?,
            Self::Contributed => write!(f, "contributed")?,
            Self::Requested => write!(f, "requested")?,
            Self::Get { urn } => write!(f, "get {:?}", urn)?,
            Self::Peers { urn } => write!(f, "peers {:?}", urn)?,
        }
    }
}
