use std::num::ParseIntError;

use owo_colors::OwoColorize;

pub fn get_status_list(version_: &str) -> Result<FeatureStatusList, String> {
    let mut status_list = FeatureStatusList::default();
    let version = parse_version_string(version_)?;

    let current_cmp = version_cmp(version, env!("CARGO_PKG_VERSION"));
    if current_cmp > 0 {
        return Err(format!{"Version {} is newer than the current version {}. Maybe update your compiler? https://github.com/jaspwr/LiA", version_, env!("CARGO_PKG_VERSION")});
    }

    if version_cmp(version, "0.1.0") >= 0 {
    }
    if version_cmp(version, "0.2.0") >= 0 {
        status_list.equation_statement_internal_syntax = ImplementationStatus::Implemented;
        status_list.enumerated_lists = ImplementationStatus::Implemented;
    }

    if current_cmp < 0 {
        println!("[{}] Document is being compiled for version {}.", "INFO".yellow(), version_);
    }
    Ok(status_list)
}

#[derive(Default)]
pub enum ImplementationStatus {
    #[default]
    NotImplemented,
    Implemented,
}

impl ImplementationStatus {
    pub fn is_supported(&self) -> bool {
        match self {
            ImplementationStatus::NotImplemented => false,
            _ => true,
        }
    }
}

#[derive(Default)]
pub struct FeatureStatusList {
    pub enumerated_lists: ImplementationStatus,
    pub equation_statement_internal_syntax: ImplementationStatus,
}

fn parse_version_string(version: &str) -> Result<(u8, u8, u8), String> {
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

fn version_cmp(version1: (u8, u8, u8), version2str: &str) -> i8 {
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