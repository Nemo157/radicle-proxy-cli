#![feature(crate_visibility_modifier)]

use anyhow::Error;
use clap::Clap;

mod api;
mod app;

#[fehler::throws]
fn main() {
    app::App::try_parse()?.run()?;
}
