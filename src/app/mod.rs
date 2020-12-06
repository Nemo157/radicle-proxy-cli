use crate::api::Api;
use anyhow::Error;

mod identities;

#[derive(Debug, clap::Clap)]
crate struct App {
    #[clap(long)]
    base_url: url::Url,
    #[clap(long)]
    auth_token: String,
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
