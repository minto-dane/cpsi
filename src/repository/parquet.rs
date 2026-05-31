use std::{
    collections::HashMap,
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
    packages: HashMap<String, Package>,
}

impl Repository {
    pub fn load() -> Result<Self, CpsiError> {
        let parquet_files = find_all_parquet(constants::REPOSITORIES_DIRECTORY);

        if parquet_files.is_empty() {
            return Err(CpsiError::NoRepositories);
        }

        let mut packages = HashMap::new();

        for file in parquet_files {
            let loaded = load_packages(&file);

            for pkg in loaded {
                if packages.contains_key(&pkg.name) {
                    return Err(CpsiError::DuplicatePackage(pkg.name.clone()));
                }
                packages.insert(pkg.name.clone(), pkg);
            }
        }

        Ok(Self { packages })
    }

    pub fn find_package<T: AsRef<str>>(&self, package: T) -> Option<&Package> {
        self.packages.get(package.as_ref())
    }

    pub fn packages(&self) -> impl Iterator<Item = &Package> {
        self.packages.values()
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
        let mut loaded: Vec<Package> =
            serde_arrow::from_record_batch(&batch.unwrap_or_display()).unwrap_or_display();
        packages.append(&mut loaded);
    }

    packages
}
