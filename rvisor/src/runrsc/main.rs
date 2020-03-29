
#[macro_use]
extern crate clap;
extern crate log;
extern crate env_logger;

mod cli;

use sentry;

fn main () {
    env_logger::init();
    cli::run_cli();
}
