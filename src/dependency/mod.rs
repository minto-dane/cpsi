use crate::util::version::Version;
use serde::{Deserialize, Serialize};

pub mod resolve;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub min_version: Option<Version>,
}
