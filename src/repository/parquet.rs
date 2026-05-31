use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    package::Package,
    util::{constants, result::ResultExt},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    packages: Vec<Package>,
    name: String,
}

impl Repository {
    pub fn new() -> Repository {
        let parquet_files = find_all_parquet(constants::REPOSITORIES_DIRECTORY);

        if parquet_files.is_empty() {
            eprintln!("Please update and retry later");
            panic!();
        }
    }
}

fn find_all_parquet(dir: &str) -> Vec<PathBuf> {
    let mut list = Vec::new();

    for entry in fs::read_dir(dir).unwrap_or_display() {
        let entry = entry.unwrap_or_display();
        if entry.file_name().to_str().unwrap().ends_with(".parquet") {
            list.push(entry.path());
        }
    }

    return list;
}
