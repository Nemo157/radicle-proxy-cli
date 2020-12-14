use crate::app::Context;
use anyhow::Error;

#[derive(Debug, clap::Clap)]
/// Commands related to the seed list
pub(super) struct App {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Clap)]
pub(super) enum Cmd {
    /// Get the list of configured seeds
    List,
    /// Add a new seed
    Add { seed: String },
    /// Remove an existing seed
    Remove { seed: String },
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
                for seed in context.api.session().get()?.settings.coco.seeds {
                    println!("{}", seed);
                }
            }
            Self::Add { seed } => {
                let mut settings = context.api.session().get()?.settings;
                settings.coco.seeds.insert(seed);
                context.api.session().update_settings(settings)?;
            }
            Self::Remove { seed } => {
                let mut settings = context.api.session().get()?.settings;
                settings.coco.seeds.remove(&seed);
                context.api.session().update_settings(settings)?;
            }
        }
    }
}

impl std::fmt::Display for App {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        write!(f, "seeds {}", self.cmd)?;
    }
}

impl std::fmt::Display for Cmd {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        match self {
            Self::List => write!(f, "list")?,
            Self::Add { seed } => write!(f, "add {:?}", seed)?,
            Self::Remove { seed } => write!(f, "remove {:?}", seed)?,
        }
    }
}
