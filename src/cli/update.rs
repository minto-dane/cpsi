use crate::repository::sync;
use cps_common::errors::CpsiError;

pub async fn update() -> Result<(), CpsiError> {
    sync::sync().await?;

    Ok(())
}
