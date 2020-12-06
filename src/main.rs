#![feature(crate_visibility_modifier)]
#![feature(min_const_generics)]
#![feature(array_value_iter)]

use anyhow::Error;
use clap::Clap;

mod api;
mod app;

#[fehler::throws]
fn main() {
    app::App::try_parse()?.run()?;
}
