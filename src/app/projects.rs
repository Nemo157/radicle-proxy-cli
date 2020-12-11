use crate::app::Context;
use anyhow::Error;

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
            Self::Tracked => {
                for project in context.api.projects().tracked()? {
                    println!("{}: {}", project.metadata.name, project.urn);
                }
            }

            Self::Contributed => {
                for project in context.api.projects().contributed()? {
                    println!("{}: {}", project.metadata.name, project.urn);
                }
            }

            Self::Get { urn } => {
                if let Some(project) = context.api.projects().get(&urn)? {
                    println!("{:#?}", project);
                } else {
                    println!("Project {} not found", urn);
                }
            }

            Self::Peers { urn } => {
                for peer in context.api.projects().peers(&urn)? {
                    println!(
                        "{} ({}): {}",
                        peer.status.user.metadata.handle, peer.peer_id, peer.status.role
                    );
                }
            }
        }
    }
}
