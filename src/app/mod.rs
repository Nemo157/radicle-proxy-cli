use crate::api::Api;
use anyhow::Error;
use secrecy::Secret;

mod identities;

#[derive(Debug, clap::Clap)]
#[clap(name = "rad", about, version)]
#[clap(global_setting(clap::AppSettings::ColoredHelp))]
#[clap(global_setting(clap::AppSettings::DisableHelpSubcommand))]
#[clap(global_setting(clap::AppSettings::GlobalVersion))]
crate struct App {
    #[clap(long, default_value = "http://localhost:17246")]
    base_url: url::Url,
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Clap)]
enum Cmd {
    Identities(identities::App),
}

#[derive(Debug)]
struct Context {
    api: Api,
}

impl App {
    #[fehler::throws]
    crate fn run(self) {
        tracing::trace!(?self, "running app");
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
            Self::Identities(identities) => identities.run(context)?,
        }
    }
}
