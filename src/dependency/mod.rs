use serde::{Deserialize, Serialize};

use crate::util::version::Version;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub min_version: Option<Version>,
}
