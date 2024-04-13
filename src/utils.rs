use serde::{Deserialize, Serialize};
use std::process;

pub const API: &str = "https://api.alldebrid.com/v4";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub apikey: String,
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
pub struct Error {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Response<T> {
    status: String,
    data: Option<T>,
    error: Option<Error>,
}

pub enum Res<T> {
    Data(T),
    Error(Error),
}

pub fn load_config() -> Config {
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
                println!("Rerun the command to enter a new apikey!");
                config.apikey = "".to_string();
                confy::store("adget", None, &config).expect("Failed to remove apikey from config");
                process::exit(1);
            }
            Res::Data(data) => {
                println!("Logged in as {}", data.user.username);
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

pub fn link_unlock(apikey: &str, link: &str) -> Res<UnlockLink> {
    let url = format!("{API}/link/unlock?agent=cli&apikey={apikey}&link={link}");
    return all_debrid_get::<UnlockLink>(&url)
        .expect("Unexpected error happend while unlocking link");
}

