use crate::api::Api;
use anyhow::Error;
use secrecy::Secret;

mod identities;
mod projects;
mod seeds;
mod session;

const APP_NAME: &str = env!("CARGO_BIN_NAME");
const DEFAULT_BASE_URL: &str = "http://localhost:17246";

#[derive(Debug, clap::Clap)]
#[clap(name = APP_NAME, about, version)]
#[clap(global_setting(clap::AppSettings::ColoredHelp))]
#[clap(global_setting(clap::AppSettings::DisableHelpSubcommand))]
#[clap(global_setting(clap::AppSettings::GlobalVersion))]
crate struct App {
    #[clap(long, default_value = DEFAULT_BASE_URL)]
    base_url: url::Url,
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Clap)]
enum Cmd {
    Identities(identities::App),
    Session(session::App),
    Seeds(seeds::App),
    Projects(projects::App),
}

#[derive(Debug)]
struct Context {
    api: Api,
}

impl App {
    #[fehler::throws]
    #[tracing::instrument(fields(%self))]
    crate fn run(self) {
        tracing::debug!("requesting passphrase");
        let passphrase = Secret::new(rpassword::read_password_from_tty(Some(
            "Please enter radicle passphrase: ",
        ))?);
        let context = Context {
            api: Api::new(self.base_url)?,
        };
        context.api.login(passphrase)?;
        self.cmd.run(&context)?
    }
}

impl Cmd {
    #[fehler::throws]
    fn run(self, context: &Context) {
        match self {
            Self::Identities(app) => app.run(context)?,
            Self::Session(app) => app.run(context)?,
            Self::Seeds(app) => app.run(context)?,
            Self::Projects(app) => app.run(context)?,
        }
    }
}

impl std::fmt::Display for App {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        write!(f, "{}", APP_NAME)?;
        if self.base_url != DEFAULT_BASE_URL.parse().unwrap() {
            write!(f, " --base-url={}", self.base_url)?;
        }
        write!(f, " {}", self.cmd)?;
    }
}

impl std::fmt::Display for Cmd {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        match self {
            Self::Identities(app) => write!(f, "{}", app)?,
            Self::Session(app) => write!(f, "{}", app)?,
            Self::Seeds(app) => write!(f, "{}", app)?,
            Self::Projects(app) => write!(f, "{}", app)?,
        }
    }
}
