use std::process::Command;

use crate::deploy::write_from_asset;

use super::{DeployError, Deployer};

pub struct PythonDeployer {
    output: String,
}

impl PythonDeployer {
    pub fn new(output: &str) -> Self {
        Self {
            output: output.to_string(),
        }
    }
}

impl Deployer for PythonDeployer {
    fn deploy_http(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "main.py", "python/http.py")?;
        Ok(())
    }

    fn deploy_event(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "main.py", "python/event.py")?;
        Ok(())
    }

    fn add_dependency(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "requirements.txt", "python/requirements.txt")?;

        let status = Command::new("pip")
            .current_dir(&self.output)
            .args(["install", "-r", "requirements.txt"])
            .status()
            .map_err(|e| {
                DeployError::new(&format!(
                    "execute `pip install -r requirements.txt` error: {}",
                    e
                ))
            })?;

        if !status.success() {
            return Err(DeployError::new(
                "could not terminate successfly: pip install -r requirements.txt",
            ));
        }

        Ok(())
    }
}
