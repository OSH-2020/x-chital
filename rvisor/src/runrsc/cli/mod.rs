

// we are using clap for runrs's subcommand
use clap::App;

// Clap Usage:
// flag : matches.is_present("flag")
// optional : matches.value_of("config").unwrap_or("default.conf")
// input : if let Some(matches) = matches.subcommand_matches("test") {
//              matches.value_of("INPUT").unwrap();
//         }


mod run;
mod boot;

use super::sentry;

pub fn run_cli() {
    use log::info;

    info!("running client.");
    // cli.yml contains the defination of the subcommand
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let input = matches.value_of("INNER_PROGRAM").unwrap();

        let tty = matches.is_present("tty");
        run::run_commmand(tty, input);
    }

    if let Some(matches) = matches.subcommand_matches("boot") {
        let input = matches.value_of("INNER_PROGRAM").unwrap();

        let tty = matches.is_present("tty");
        boot::boot_commmand(tty, input);
    }
}
