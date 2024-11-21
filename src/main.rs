#![allow(non_snake_case)]

mod command_chain;

mod app;
mod banner;
mod commander;
mod vim_mode;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(app::App);
}
