use serde::{Deserialize, Serialize};
use std::process;

const API: &str = "https://api.alldebrid.com/v4";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub apikey: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UserData {
    pub username: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub user: UserData,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Error {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Response<T> {
    pub status: String,
    pub data: Option<T>,
    pub error: Option<Error>,
}

pub enum Res<T> {
    Data(T),
    Error(Error),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UnlockLink {
    pub link: String,
    pub host: String,
    pub filename: String,
    // streaming: Vec<?>,
    pub paws: bool,
    pub filesize: u64,
    pub id: String,
    pub delayed: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MagnetUpload {
    pub magnets: Vec<Magnet>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Magnet {
    pub magnet: String,
    pub error: Option<Error>,
    pub hash: Option<String>,
    pub name: Option<String>,
    pub size: Option<u64>,
    pub ready: Option<bool>,
    pub id: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DetailedMagnet {
    pub id: u64,
    pub filename: String,
    pub size: u64,
    pub hash: String,
    pub status: String,
    pub status_code: u64,
    pub downloaded: u64,
    pub uploaded: u64,
    pub seeders: u64,
    pub download_speed: u64,
    pub processing_perc: u64,
    pub upload_speed: u64,
    pub upload_date: u64,
    pub completion_date: u64,
    pub links: Vec<MagnetLink>,
    pub r#type: String,
    pub notified: bool,
    pub version: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MagnetStatus {
    pub magnets: DetailedMagnet,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MagnetLink {
    pub filename: String,
    pub size: u64,
    pub files: Vec<MagnetFile>,
    pub link: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MagnetFile {
    pub n: String,
    pub e: Option<Box<Vec<MagnetFile>>>,
    pub s: Option<u64>,
}

pub fn load_config() -> Config {
    let mut config: Config = confy::load("adget", None).expect("Failed to load config file");

    if &config.apikey == "" {
        eprintln!("AllDebrid apikey not found");
        eprintln!("Please visit https://alldebrid.com/apikeys/ and generate one.");
        eprintln!("paste your apikey here:");

        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line from stdin");
        config.apikey = line.trim().to_string();
        confy::store("adget", None, &config).expect("Failed to save config");

        let config_path =
            confy::get_configuration_file_path("adget", None).expect("Failed to load config path");
        eprintln!("apikey saved to {}", config_path.display());
    } else {
        let url = format!(
            "https://api.alldebrid.com/v4/user?agent=cli&apikey={}",
            &config.apikey
        );
        let res =
            all_debrid_get::<User>(&url).expect("Unexpected error happend while loading user info");
        match res {
            Res::Error(_) => {
                eprintln!("Invalid AllDebrid apikey found: deleting apikey from config...");
                eprintln!("Rerun the command to enter a new apikey!");
                config.apikey = "".to_string();
                confy::store("adget", None, &config).expect("Failed to remove apikey from config");
                process::exit(1);
            }
            Res::Data(data) => {
                eprintln!("Logged in as {}", data.user.username);
            }
        }
    }

    return config;
}

pub fn all_debrid_get<T>(url: &str) -> Result<Res<T>, Box<dyn std::error::Error>>
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

pub fn magnet_upload(apikey: &str, magnet: &str) -> Res<MagnetUpload> {
    let url = format!("{API}/magnet/upload?agent=cli&apikey={apikey}&magnets[]={magnet}");
    return all_debrid_get::<MagnetUpload>(&url)
        .expect("Unexpected error happend while uploading magnet");
}

pub fn magnet_status(apikey: &str, id: u64) -> Res<MagnetStatus> {
    let url = format!("{API}/magnet/status?agent=cli&apikey={apikey}&id={id}");
    return all_debrid_get::<MagnetStatus>(&url)
        .expect("Unexpected error happend while retriving magnet status");
}

pub fn link_unlock(apikey: &str, link: &str) -> Res<UnlockLink> {
    let url = format!("{API}/link/unlock?agent=cli&apikey={apikey}&link={link}");
    return all_debrid_get::<UnlockLink>(&url)
        .expect("Unexpected error happend while unlocking link");
}
