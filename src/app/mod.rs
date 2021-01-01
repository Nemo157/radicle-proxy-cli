use self::context::{Context, With, WithContext};
use crate::api::Api;
use anyhow::Error;
use secrecy::Secret;

mod context;
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

trait ResultExt<T> {
    fn ok_or_debug(self) -> Option<T>;
}

impl<T, E: std::fmt::Debug> ResultExt<T> for Result<T, E> {
    fn ok_or_debug(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                tracing::debug!("{:?}", err);
                None
            }
        }
    }
}

#[cfg(target_os = "linux")]
mod auth_token {
    use anyhow::Error;
    use keyutils::{keytypes::user::User, Keyring, SpecialKeyring};
    use secrecy::{ExposeSecret, Secret};

    const AUTH_TOKEN_KEY: &str = "radicle-proxy-cli:auth_token";
    const AUTH_TOKEN_EXPIRY: std::time::Duration = std::time::Duration::from_secs(15 * 60);

    #[fehler::throws]
    #[tracing::instrument]
    pub(super) fn load() -> Secret<String> {
        use keyutils::{keytypes::user::User, Keyring, SpecialKeyring};
        // SAFETY: Not actually unsafe: https://github.com/mathstuf/rust-keyutils/issues/56
        let session_keyring = unsafe { Keyring::new(SpecialKeyring::UserSession.serial()) };
        let mut key = session_keyring
            .search_for_key::<User, _, _>(AUTH_TOKEN_KEY, SpecialKeyring::Process)?;
        let auth_token = String::from_utf8(key.read()?)?;
        key.set_timeout(AUTH_TOKEN_EXPIRY)?;
        Secret::new(auth_token)
    }

    #[fehler::throws]
    #[tracing::instrument]
    pub(super) fn store(auth_token: Secret<String>) {
        // SAFETY: Not actually unsafe: https://github.com/mathstuf/rust-keyutils/issues/56
        let mut session_keyring = unsafe { Keyring::new(SpecialKeyring::UserSession.serial()) };
        let mut key = session_keyring
            .add_key::<User, _, _>(AUTH_TOKEN_KEY, auth_token.expose_secret().as_bytes())?;
        key.set_timeout(AUTH_TOKEN_EXPIRY)?;
    }
}

#[cfg(not(target_os = "linux"))]
mod auth_token {
    use crate::api::Api;
    use anyhow::Error;
    use secrecy::Secret;

    #[fehler::throws]
    #[tracing::instrument]
    fn load() -> Secret<String> {
        None
    }

    #[fehler::throws]
    #[tracing::instrument]
    fn store() {}
}

#[fehler::throws]
#[tracing::instrument]
fn get_passphrase() -> Secret<String> {
    Secret::new(rpassword::read_password_from_tty(Some(
        "Please enter radicle passphrase: ",
    ))?)
}

impl App {
    #[fehler::throws]
    #[tracing::instrument(fields(%self))]
    crate fn run(self) {
        let api = Api::new(self.base_url)?;

        let authed = if let Some(auth_token) = auth_token::load().ok_or_debug() {
            api.set_token(auth_token)?
        } else {
            false
        };

        if !authed {
            auth_token::store(api.login(get_passphrase()?)?).ok_or_debug();
        }

        self.cmd.with(Context::new(api)).run()?;
    }
}

impl WithContext<Cmd> {
    #[fehler::throws]
    fn run(self) {
        self.and_then(|cmd, context| match cmd {
            Cmd::Identities(app) => app.with(context).run(),
            Cmd::Session(app) => app.with(context).run(),
            Cmd::Seeds(app) => app.with(context).run(),
            Cmd::Projects(app) => app.with(context).run(),
        })?;
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
