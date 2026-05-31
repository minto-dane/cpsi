use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    package::Package,
    util::{constants, errors::CpsiError, result::ResultExt},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    packages: Vec<Package>,
}

impl Repository {
    pub fn new() -> Result<Self, CpsiError> {
        let parquet_files = find_all_parquet(constants::REPOSITORIES_DIRECTORY);

        if parquet_files.is_empty() {
            return Err(CpsiError::NoRepositories);
        }

        let mut packages = Vec::new();

        for file in parquet_files {
            let mut loaded = load_packages(&file);

            packages.append(&mut loaded);
        }

        Ok(Self { packages })
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

fn load_packages<P: AsRef<Path>>(file: P) -> Vec<Package> {
    let file = fs::File::open(file).unwrap_or_display();
    let mut packages: Vec<Package> = Vec::new();

    let reader = parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(file)
        .unwrap_or_display()
        .build()
        .unwrap_or_display();

    for batch in reader {
        packages = serde_arrow::from_record_batch(&batch.unwrap_or_display()).unwrap_or_display()
    }

    packages
}
