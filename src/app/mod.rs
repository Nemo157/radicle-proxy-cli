use anyhow::Error;

mod identities;

#[derive(Debug, clap::Clap)]
crate struct App {
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
    agent: ureq::Agent,
}

impl App {
    #[fehler::throws]
    crate fn run(self) {
        let agent = ureq::agent();
        agent.set_cookie(
            ureq::Cookie::build("auth-token", self.auth_token)
                .domain("localhost")
                .path("/")
                .finish(),
        );
        let context = Context { agent };
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
