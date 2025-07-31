use crate::{
    cli::print_info,
    version::{parse_version_string, version_cmp},
};

pub fn get_status_list(version_: &str) -> Result<FeatureStatusList, String> {
    let mut status_list = FeatureStatusList::default();
    let version = parse_version_string(version_)?;

    let current_cmp = version_cmp(version, env!("CARGO_PKG_VERSION"));
    if current_cmp > 0 {
        return Err(
            format!(
                "Version {} is newer than the current version {}. Maybe update your compiler? https://github.com/jaspwr/LiA",
                version_,
                env!("CARGO_PKG_VERSION")
            )
        );
    }

    version_cmp(version, "0.1.0");0;
    if version_cmp(version, "0.2.0") >= 0 {
        status_list.equation_statement_internal_syntax = ImplementationStatus::Implemented;
        status_list.enumerated_lists = ImplementationStatus::Implemented;
    }

    if current_cmp < 0 {
        print_info(format!(
            "Document is being compiled for version {version_}."
        ));
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
