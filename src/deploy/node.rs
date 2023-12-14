use std::process::Command;

use super::{write_from_asset, DeployError, Deployer};

pub struct NodeDeployer {
    output: String,
}

impl NodeDeployer {
    pub fn new(output: &str) -> Self {
        Self {
            output: output.to_string(),
        }
    }
}

impl Deployer for NodeDeployer {
    fn deploy_http(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "index.js", "node/http.js")?;
        Ok(())
    }

    fn deploy_event(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "index.js", "node/event.js")?;
        Ok(())
    }

    fn add_dependency(&self) -> Result<(), DeployError> {
        let status = Command::new("npm")
            .current_dir(&self.output)
            .args(["init", "-y"])
            .status()
            .map_err(|e| DeployError::new(&format!("execute `npm init -y` error: {}", e)))?;

        if !status.success() {
            return Err(DeployError::new(
                "could not terminate successfly: npm init -y",
            ));
        }

        let status = Command::new("npm")
            .current_dir(&self.output)
            .args(["install", "@google-cloud/functions-framework"])
            .status()
            .map_err(|e| DeployError::new(&format!("execute `npm install` error: {}", e)))?;

        if !status.success() {
            return Err(DeployError::new(
                "could not terminate successfly: npm install",
            ));
        }

        Ok(())
    }
}
