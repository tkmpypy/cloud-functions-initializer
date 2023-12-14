use std::process::Command;

use super::{write_from_asset, DeployError, Deployer};

pub struct RubyDeployer {
    output: String,
}

impl RubyDeployer {
    pub fn new(output: &str) -> Self {
        Self {
            output: output.to_string(),
        }
    }
}

impl Deployer for RubyDeployer {
    fn deploy_http(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "app.rb", "ruby/http.rb")?;
        Ok(())
    }

    fn deploy_event(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "app.rb", "ruby/event.rb")?;
        Ok(())
    }

    fn add_dependency(&self) -> Result<(), DeployError> {
        let status = Command::new("bundle")
            .current_dir(&self.output)
            .args(["init"])
            .status()
            .map_err(|e| DeployError::new(&format!("execute `bundle init` error: {}", e)))?;

        if !status.success() {
            return Err(DeployError::new(
                "could not terminate successfly: bundle init",
            ));
        }

        let status = Command::new("bundle")
            .current_dir(&self.output)
            .args(["add", "functions_framework"])
            .status()
            .map_err(|e| {
                DeployError::new(&format!(
                    "execute `bundle add functions_framework` error: {}",
                    e
                ))
            })?;

        if !status.success() {
            return Err(DeployError::new(
                "could not terminate successfly: bundle add functions_framework",
            ));
        }

        Ok(())
    }
}
