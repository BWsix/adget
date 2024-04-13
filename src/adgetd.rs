mod utils;

use clap::Parser;
use std::{os::unix::process::CommandExt, process};

#[derive(Debug, Parser)]
struct Cli {
    link: String,
}

fn main() {
    let args = Cli::parse();
    let config = utils::load_config();

    match utils::link_unlock(&config.apikey, &args.link) {
        utils::Res::Error(error) => println!("Error: {}", error.message),
        utils::Res::Data(data) => {
            process::Command::new("wget").arg(data.link).exec();
        }
    }
}
