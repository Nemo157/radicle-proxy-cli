#![feature(array_value_iter)]
#![feature(crate_visibility_modifier)]
#![feature(debug_non_exhaustive)]
#![feature(min_const_generics)]

use anyhow::Error;
use clap::Clap;
use tracing_subscriber::EnvFilter;

mod api;
mod app;

#[fehler::throws]
fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("RAD_LOG"))
        .with_writer(std::io::stderr)
        .pretty()
        .init();
    app::App::parse().run()?;
}
