use std::process::Command;

use crate::deploy::DeployError;

use super::{write_from_asset, Deployer};

pub struct GolangDeployer {
    output: String,
}

impl GolangDeployer {
    pub fn new(output: &str) -> Self {
        Self {
            output: output.to_string(),
        }
    }
}

impl Deployer for GolangDeployer {
    fn deploy_http(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "function.go", "go/http.go")?;
        Ok(())
    }

    fn deploy_event(&self) -> Result<(), DeployError> {
        write_from_asset(&self.output, "function.go", "go/event.go")?;
        Ok(())
    }

    fn add_dependency(&self) -> Result<(), DeployError> {
        let status = Command::new("go")
            .current_dir(&self.output)
            .args(["mod", "init", "example.com/myfunction"])
            .status()
            .map_err(|e| DeployError::new(&format!("execute `go mod init` error: {}", e)))?;

        if !status.success() {
            return Err(DeployError::new(
                "could not terminate successfly: go mod init",
            ));
        }

        let status = Command::new("go")
            .current_dir(&self.output)
            .args(["mod", "tidy"])
            .status()
            .map_err(|e| DeployError::new(&format!("execute `go mod tidy` error: {}", e)))?;

        if !status.success() {
            return Err(DeployError::new(
                "could not terminate successfly: go mod tidy",
            ));
        }

        Ok(())
    }
}
