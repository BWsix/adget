use clap::Parser;
use serde::{Deserialize, Serialize};
use std::os::unix::process::CommandExt;
use std::process::exit;
use std::{process, thread, time};

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    apikey: String,
}

#[derive(Debug, Parser)]
struct Cli {
    magnet: String,
}

enum Res<T> {
    Data(T),
    Error(Error),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Response<T> {
    status: String,
    data: Option<T>,
    error: Option<Error>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Error {
    code: String,
    message: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UserData {
    username: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct User {
    user: UserData,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MagnetUpload {
    magnets: Vec<Magnet>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Magnet {
    magnet: String,
    error: Option<Error>,
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UnlockLink {
    link: String,
    host: String,
    filename: String,
    // streaming: Vec<?>,
    paws: bool,
    filesize: u64,
    id: String,
}

fn all_debrid_get<T>(url: &str) -> Result<Res<T>, Box<dyn std::error::Error>>
where
    T: for<'a> Deserialize<'a>,
{
    let body = reqwest::blocking::get(url)
        .expect("Failed to make GET request")
        .text()
        .expect("Failed to parse response from GET request");
    let response: Response<T> = serde_json::from_str(&body).expect("Failed to load json");

    if response.status == "success" {
        return Ok(Res::Data(
            response
                .data
                .expect("Loaded json, but failed to parse data"),
        ));
    } else {
        return Ok(Res::Error(
            response
                .error
                .expect("Loaded json, but failed to parse error"),
        ));
    }
}

fn magnet_upload(apikey: &str, magnet: &str) -> Res<MagnetUpload> {
    let url = format!(
        "https://api.alldebrid.com/v4/magnet/upload?agent=cli&apikey={apikey}&magnets[]={magnet}"
    );
    return all_debrid_get::<MagnetUpload>(&url)
        .expect("Unexpected error happend while uploading magnet");
}

fn magnet_status(apikey: &str, id: u64) -> Res<MagnetStatus> {
    let url =
        format!("https://api.alldebrid.com/v4/magnet/status?agent=cli&apikey={apikey}&id={id}");
    return all_debrid_get::<MagnetStatus>(&url)
        .expect("Unexpected error happend while retriving magnet status");
}

fn unlock_link(apikey: &str, link: &str) -> Res<UnlockLink> {
    let url =
        format!("https://api.alldebrid.com/v4/link/unlock?agent=cli&apikey={apikey}&link={link}");
    return all_debrid_get::<UnlockLink>(&url)
        .expect("Unexpected error happend while unlocking link");
}

fn main() {
    let args = Cli::parse();
    let mut config: Config = confy::load("adget", None).expect("Failed to load config file");

    if &config.apikey == "" {
        println!("AllDebrid apikey not found");
        println!("Please visit https://alldebrid.com/apikeys/ and generate one.");
        println!("paste your apikey here:");

        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line from stdin");
        config.apikey = line.trim().to_string();
        confy::store("adget", None, &config).expect("Failed to save config");

        let config_path =
            confy::get_configuration_file_path("adget", None).expect("Failed to load config path");
        println!("apikey saved to {}", config_path.display());
    } else {
        let url = format!(
            "https://api.alldebrid.com/v4/user?agent=cli&apikey={}",
            &config.apikey
        );
        let res =
            all_debrid_get::<User>(&url).expect("Unexpected error happend while loading user info");
        match res {
            Res::Error(_) => {
                println!("Invalid AllDebrid apikey found: deleting apikey from config...");
                config.apikey = "".to_string();
                confy::store("adget", None, &config).expect("Failed to remove apikey from config");
                process::exit(1);
            }
            Res::Data(data) => {
                println!("Logged in as {}", data.user.username);
            }
        }
    }

    match magnet_upload(&config.apikey, &args.magnet) {
        Res::Error(error) => println!("Error: {}", error.message),
        Res::Data(data) => {
            if let Some(error) = &data.magnets[0].error {
                println!("Error: {}", error.message);
                exit(1);
            }

            let id = data.magnets[0].id.expect("Failed to upload magnet");
            loop {
                match magnet_status(&config.apikey, id) {
                    Res::Error(error) => {
                        println!("Error: {}", error.message);
                        process::exit(1);
                    }
                    Res::Data(data) => {
                        if data.magnets.status == "Ready" {
                            println!("Ready.");
                            // TODO: support list of links
                            if data.magnets.links.len() > 1 {
                                dbg!(&data.magnets.links);
                                println!("Folders are not supported right now ._.");
                                process::exit(1);
                            }
                            match unlock_link(&config.apikey, &data.magnets.links[0].link) {
                                Res::Error(error) => println!("Error: {}", error.message),
                                Res::Data(data) => {
                                    process::Command::new("wget").arg(data.link).exec();
                                }
                            }
                        } else if data.magnets.status == "Downloading" {
                            println!(
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
                            println!(
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
                            println!("{}", data.magnets.status)
                        }
                    }
                }
            }
        }
    }
}
