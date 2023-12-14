use std::process::Command;

use super::{write_from_asset, DeployError, Deployer};

pub struct PhpDeployer {
    output: String,
}

impl PhpDeployer {
    pub fn new(output: &str) -> Self {
        Self {
            output: output.to_string(),
        }
    }
}

impl Deployer for PhpDeployer {
    fn deploy_http(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "index.php", "php/http.php")?;
        Ok(())
    }

    fn deploy_event(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "index.php", "php/event.php")?;
        Ok(())
    }

    fn add_dependency(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "composer.json", "php/composer.json")?;

        let status = Command::new("composer")
            .current_dir(&self.output)
            .args(["require", "google/cloud-functions-framework"])
            .status()
            .map_err(|e| {
                DeployError::new(&format!(
                    "execute `composer require google/cloud-functions-framework` error: {}",
                    e
                ))
            })?;

        if !status.success() {
            return Err(DeployError::new(
                "could not terminate successfly: composer require google/cloud-functions-framework",
            ));
        }

        Ok(())
    }
}
