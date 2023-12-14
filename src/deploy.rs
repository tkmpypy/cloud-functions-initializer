pub mod golang;
pub mod node;
pub mod php;
pub mod python;
pub mod ruby;

use std::{fs, path::Path};

use crate::asset::Asset;

pub struct DeployError {
    pub msg: String,
}

impl DeployError {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for DeployError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub trait Deployer {
    fn deploy_http(&self) -> Result<(), DeployError>;
    fn deploy_event(&self) -> Result<(), DeployError>;
    fn add_dependency(&self) -> Result<(), DeployError>;
}

pub fn to_path(output: &str) -> Result<&Path, DeployError> {
    let path = Path::new(output);
    if !path.exists() {
        return Err(DeployError::new(&format!("not found: {}", output)));
    } else if !path.is_dir() {
        return Err(DeployError::new(&format!("not directory: {}", output)));
    }

    Ok(path)
}

pub fn write_from_asset(
    output_dir: &str,
    fname: &str,
    asset_path: &str,
) -> Result<(), DeployError> {
    let path = to_path(output_dir)?;
    let p = path.join(fname);

    let src = Asset::get(asset_path)
        .ok_or_else(|| DeployError::new(&format!("cannot open {} template file", asset_path)))?;

    fs::write(p.as_path(), src.data)
        .map_err(|e| DeployError::new(&format!("failed create file: {:?}", e)))?;
    Ok(())
}
