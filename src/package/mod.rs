use serde::{Deserialize, Serialize};

use crate::{
    dependency::Dependency,
    util::{architecture::Architecture, version::Version},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    name: String,
    version: Version,

    release: u32,
    arch: Vec<Architecture>,

    dependencies: Vec<Dependency>,
    description: String,

    provides: Vec<String>,

    repository: String,
}
