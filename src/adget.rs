mod utils;

use clap::{arg, ArgAction, Command};
use std::{os::unix::process::CommandExt, process, thread, time};

fn main() {
    let matches = Command::new("adget")
        .about("All Debrid Downloader")
        .arg(
            arg!(<link>)
                .required(true)
                .help("Magnet or Premium Link To Download"),
        )
        .arg(
            arg!(nodl: --"nodl")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("No Download: only prints out the all debrid link"),
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
    let nodl = matches.get_flag("nodl");
    let link = matches.get_one::<String>("link").expect("required");
    let wget_args = matches
        .get_many::<String>("wget_args")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default();

    if link.starts_with("magnet") {
        match utils::magnet_upload(&config.apikey, &link) {
            utils::Res::Error(error) => eprintln!("Error: {}", error.message),
            utils::Res::Data(data) => {
                if let Some(error) = &data.magnets[0].error {
                    eprintln!("Error: {}", error.message);
                    process::exit(1);
                }
                let id = data.magnets[0].id.expect("Failed to upload magnet");
                loop {
                    match utils::magnet_status(&config.apikey, id) {
                        utils::Res::Error(error) => {
                            eprintln!("Error: {}", error.message);
                            process::exit(1);
                        }
                        utils::Res::Data(data) => {
                            if data.magnets.status == "Ready" {
                                eprintln!("Ready.");
                                // TODO: support list of links
                                if data.magnets.links.len() > 1 {
                                    dbg!(&data.magnets.links);
                                    eprintln!("Folders are unsupported right now ._.");
                                    process::exit(1);
                                }
                                match utils::link_unlock(
                                    &config.apikey,
                                    &data.magnets.links[0].link,
                                ) {
                                    utils::Res::Error(error) => {
                                        eprintln!("Error: {}", error.message)
                                    }
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
                            } else if data.magnets.status == "Downloading" {
                                eprintln!(
                                    "Downloading - {: >9}/{: >9} ({: >9}/s, {: >2} seeders)",
                                    humansize::format_size(
                                        data.magnets.downloaded,
                                        humansize::DECIMAL
                                    ),
                                    humansize::format_size(data.magnets.size, humansize::DECIMAL),
                                    humansize::format_size(
                                        data.magnets.download_speed,
                                        humansize::DECIMAL
                                    ),
                                    data.magnets.seeders
                                );
                                thread::sleep(time::Duration::from_secs(1));
                            } else if data.magnets.status == "Uploading" {
                                eprintln!(
                                    "Uploading - {: >9}/{: >9} ({: >9}/s)",
                                    humansize::format_size(
                                        data.magnets.uploaded,
                                        humansize::DECIMAL
                                    ),
                                    humansize::format_size(data.magnets.size, humansize::DECIMAL),
                                    humansize::format_size(
                                        data.magnets.upload_speed,
                                        humansize::DECIMAL
                                    )
                                );
                                thread::sleep(time::Duration::from_secs(1));
                            } else {
                                eprintln!("{}", data.magnets.status)
                            }
                        }
                    }
                }
            }
        }
    }
    if link.starts_with("http") {
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
}
