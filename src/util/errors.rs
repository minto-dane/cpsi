use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpsiError {
    #[error("Not Found Available Repositories")]
    NoRepositories,
}
