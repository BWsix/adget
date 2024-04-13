mod utils;

use clap::{arg, ArgAction, Command};
use std::{os::unix::process::CommandExt, process};

fn main() {
    let matches = Command::new("adgetd")
        .about("Premium link downloader")
        .arg(
            arg!(<link>)
                .required(true)
                .help("The premium link to unlock"),
        )
        .arg(
            arg!(nodl: --"nodl")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not download with wget; only prints out the unlocked link"),
        )
        .arg(
            arg!(wget_args: ["wget-args"])
                .required(false)
                .num_args(1..)
                .last(true)
                .help("Args for wget"),
        )
        .get_matches();

    let config = utils::load_config();
    let link = matches.get_one::<String>("link").expect("required");
    let nodl = matches.get_flag("nodl");
    let wget_args = matches
        .get_many::<String>("wget_args")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default();

    match utils::link_unlock(&config.apikey, &link) {
        utils::Res::Error(error) => println!("Error: {}", error.message),
        utils::Res::Data(data) => {
            if nodl {
                println!("{}", data.link);
                process::exit(0);
            } else {
                process::Command::new("wget")
                    .arg(data.link)
                    .args(&wget_args)
                    .exec();
            }
        }
    }
}
