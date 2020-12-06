#![feature(array_value_iter)]
#![feature(crate_visibility_modifier)]
#![feature(debug_non_exhaustive)]
#![feature(min_const_generics)]

use anyhow::Error;
use clap::Clap;

mod api;
mod app;

#[fehler::throws]
fn main() {
    app::App::parse().run()?;
}
