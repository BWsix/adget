mod utils;
use utils::API;

use clap::{arg, ArgAction, Command};
use serde::Deserialize;
use std::{os::unix::process::CommandExt, process, thread, time};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MagnetUpload {
    magnets: Vec<Magnet>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Magnet {
    magnet: String,
    error: Option<utils::Error>,
    hash: Option<String>,
    name: Option<String>,
    size: Option<u64>,
    ready: Option<bool>,
    id: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MagnetStatus {
    magnets: DetailedMagnet,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct DetailedMagnet {
    id: u64,
    filename: String,
    size: u64,
    hash: String,
    status: String,
    status_code: u64,
    downloaded: u64,
    uploaded: u64,
    seeders: u64,
    download_speed: u64,
    processing_perc: u64,
    upload_speed: u64,
    upload_date: u64,
    completion_date: u64,
    links: Vec<MagnetLink>,
    r#type: String,
    notified: bool,
    version: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MagnetLink {
    filename: String,
    size: u64,
    files: Vec<MagnetFile>,
    link: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MagnetFile {
    n: String,
    e: Option<Box<Vec<MagnetFile>>>,
    s: Option<u64>,
}

fn magnet_upload(apikey: &str, magnet: &str) -> utils::Res<MagnetUpload> {
    let url = format!("{API}/magnet/upload?agent=cli&apikey={apikey}&magnets[]={magnet}");
    return utils::all_debrid_get::<MagnetUpload>(&url)
        .expect("Unexpected error happend while uploading magnet");
}

fn magnet_status(apikey: &str, id: u64) -> utils::Res<MagnetStatus> {
    let url = format!("{API}/magnet/status?agent=cli&apikey={apikey}&id={id}");
    return utils::all_debrid_get::<MagnetStatus>(&url)
        .expect("Unexpected error happend while retriving magnet status");
}

fn main() {
    let matches = Command::new("adgetm")
        .about("Magnet downloader")
        .arg(
            arg!(<magnet>)
                .required(true)
                .help("The magnet to download"),
        )
        .arg(
            arg!(nodl: --"nodl")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not download with wget; only prints out the link"),
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
    let magnet = matches.get_one::<String>("magnet").expect("required");
    let wget_args = matches
        .get_many::<String>("wget_args")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default();

    match magnet_upload(&config.apikey, &magnet) {
        utils::Res::Error(error) => eprintln!("Error: {}", error.message),
        utils::Res::Data(data) => {
            if let Some(error) = &data.magnets[0].error {
                eprintln!("Error: {}", error.message);
                process::exit(1);
            }

            let id = data.magnets[0].id.expect("Failed to upload magnet");
            loop {
                match magnet_status(&config.apikey, id) {
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
                            match utils::link_unlock(&config.apikey, &data.magnets.links[0].link) {
                                utils::Res::Error(error) => eprintln!("Error: {}", error.message),
                                utils::Res::Data(data) => {
                                    if nodl {
                                        println!("{}", data.link);
                                        process::exit(0);
                                    }else{
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
                                humansize::format_size(data.magnets.downloaded, humansize::DECIMAL),
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
                                humansize::format_size(data.magnets.uploaded, humansize::DECIMAL),
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
