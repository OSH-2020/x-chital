// runrsc is a container manager runtime like gVisor runsc
// this crate use clap

#[macro_use]
extern crate clap;
extern crate log;
extern crate env_logger;
extern crate sentry;

mod cli;

fn main () {
    env_logger::init();
    cli::run_cli();
}
