use crate::app::WithContext;
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
            Cmd::List => {
                for seed in self.api().session().get()?.settings.coco.seeds {
                    println!("{}", seed);
                }
            }
            Cmd::Add { seed } => {
                let mut settings = self.api().session().get()?.settings;
                settings.coco.seeds.insert(seed.clone());
                self.api().session().update_settings(settings)?;
            }
            Cmd::Remove { seed } => {
                let mut settings = self.api().session().get()?.settings;
                settings.coco.seeds.remove(seed);
                self.api().session().update_settings(settings)?;
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
