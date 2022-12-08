use std::{num::ParseIntError, error::Error};

use owo_colors::OwoColorize;

use crate::utils::{load_utf8_file, write_utf8_file};

pub fn parse_version_string(version: &str) -> Result<(u8, u8, u8), String> {
    let mut version_spl = version.split(".");
    if version_spl.clone().count() != 3 {
        return Err(format!{"Invalid version string \"{}\".", version});
    }
    match try_ver_number_num_casts(&mut version_spl) {
        Ok(v) =>  Ok(v),
        Err(_) => Err(format!{"Invalid version string \"{}\".", version})
    }
}

fn try_ver_number_num_casts(version_spl: &mut std::str::Split<&str>) -> Result<(u8, u8, u8), ParseIntError> {
    let major = version_spl.next().unwrap().parse::<u8>()?;
    let minor = version_spl.next().unwrap().parse::<u8>()?;
    let patch = version_spl.next().unwrap().parse::<u8>()?;
    Ok((major, minor, patch))
}

pub fn version_cmp(version1: (u8, u8, u8), version2str: &str) -> i8 {
    let (major1, minor1, patch1) = version1;
    let (major2, minor2, patch2) = parse_version_string(version2str).unwrap();
    if major1 > major2 {
        return 1;
    } else if major1 < major2 {
        return -1;
    } else {
        if minor1 > minor2 {
            return 1;
        } else if minor1 < minor2 {
            return -1;
        } else {
            if patch1 > patch2 {
                return 1;
            } else if patch1 < patch2 {
                return -1;
            } else {
                return 0;
            }
        }
    }
}


// This file may be used to store other things in the future, but for now 
// it's just needed for a timestamp of last ping and version.
static CACHE_FILE: &'static str = ".liacache";
static CARGO_TOML_URL: &'static str = "https://raw.githubusercontent.com/jaspwr/LiA/main/Cargo.toml";

pub fn check_for_new_version() -> Result<(), Box<dyn Error>> {
    let path = home::home_dir().unwrap().join(CACHE_FILE).to_str().unwrap().to_string();
    let f = match load_utf8_file(path.clone()) {
        Ok(f) => f,
        Err(_) => "0\n0.0.0".to_string() 
    };
    let mut lines = f.lines();
    let last_ping = lines.next().unwrap().parse::<u64>()?;
    let last_version = lines.next().unwrap();
    let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
    if current_time - last_ping < 86400 { // One day
        return Ok(());
    }
    let latest_version = fetch_latest_version_string()?;
    if last_version != latest_version {
        let current_version = parse_version_string(env!("CARGO_PKG_VERSION"))?;
        if version_cmp(current_version, latest_version.as_str()) < 0 {
            println!("[{}] There is a new version of LiA available at https://github.com/jaspwr/LiA. You are running {} and the latest is {}", "INFO".yellow(), env!("CARGO_PKG_VERSION"), latest_version);
        }
    }
    let _ = write_utf8_file(path, format!("{}\n{}", current_time, latest_version));
    Ok(())
}

fn fetch_latest_version_string() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get(CARGO_TOML_URL)?.text()?;
    Ok(resp.split("version = \"").nth(1).unwrap().split("\"").next().unwrap().to_string())
}