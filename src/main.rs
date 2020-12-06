#![feature(crate_visibility_modifier)]

crate mod api {
    #[derive(Debug, thiserror::Error)]
    crate enum Error {
        #[error("API response {code}, {msg}")]
        Api { msg: String, code: u16 },
        #[error(transparent)]
        UnknownUreq(#[from] ureq::Error),
        #[error(transparent)]
        UnknownIo(#[from] std::io::Error),
    }

    #[derive(Debug, serde::Deserialize)]
    struct ErrorResponse {
        message: String,
        variant: String,
    }

    crate mod identities {
        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        crate struct Identity {
            crate peer_id: String,
            crate urn: String,
            crate shareable_entity_identifier: String,
            crate metadata: Metadata,
            crate avatar_fallback: AvatarFallback,
        }

        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        crate struct Metadata {
            crate handle: String,
        }

        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        crate struct AvatarFallback {
            crate emoji: String,
        }

        #[fehler::throws(crate::api::Error)]
        crate fn list(agent: &ureq::Agent) -> Vec<Identity> {
            let response = agent.get("http://localhost:17246/v1/identities/").call();
            if response.error() {
                if response.synthetic() {
                    fehler::throw!(response.into_synthetic_error().unwrap());
                }
                let code = response.status();
                let crate::api::ErrorResponse { message: msg, .. } =
                    response.into_json_deserialize()?;
                fehler::throw!(crate::api::Error::Api { msg, code });
            }
            response.into_json_deserialize()?
        }
    }
}

crate mod app {
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
        #[fehler::throws(anyhow::Error)]
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
        #[fehler::throws(anyhow::Error)]
        fn run(self, context: &Context) {
            match self {
                Self::Identities(identities) => identities.run(context)?,
            }
        }
    }

    crate mod identities {
        #[derive(Debug, clap::Clap)]
        /// Commands related to identities
        pub(super) struct App {
            #[clap(subcommand)]
            cmd: Cmd,
        }

        #[derive(Debug, clap::Clap)]
        pub(super) enum Cmd {
            /// List all known identities
            List,
        }

        impl App {
            #[fehler::throws(anyhow::Error)]
            pub(super) fn run(self, context: &crate::app::Context) {
                self.cmd.run(context)?
            }
        }

        impl Cmd {
            #[fehler::throws(anyhow::Error)]
            pub(super) fn run(self, context: &crate::app::Context) {
                match self {
                    Self::List => {
                        for identity in crate::api::identities::list(&context.agent)? {
                            println!(
                                "{} {}: {}",
                                identity.avatar_fallback.emoji,
                                identity.metadata.handle,
                                identity.urn
                            );
                        }
                    }
                }
            }
        }
    }
}

#[fehler::throws(anyhow::Error)]
fn main() {
    use clap::Clap;

    app::App::try_parse()?.run()?;
}
