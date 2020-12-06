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
    #[clap(long)]
    base_url: url::Url,
    #[clap(long)]
    auth_token: Secret<String>,
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Clap)]
enum Cmd {
    Identities(identities::App),
}

struct Context {
    api: Api,
}

impl App {
    #[fehler::throws]
    crate fn run(self) {
        let context = Context {
            api: Api::new(self.base_url, self.auth_token)?,
        };
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
