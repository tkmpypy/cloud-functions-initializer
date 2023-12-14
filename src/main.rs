use std::{fs, path::Path, process::exit};

use clap::Parser;
use cloud_functions_initializer::{
    command::{Args, FuncType, LangType},
    deploy::{
        golang::GolangDeployer, node::NodeDeployer, php::PhpDeployer, python::PythonDeployer,
        ruby::RubyDeployer, DeployError, Deployer,
    },
};

fn deploy(args: &Args) -> Result<(), DeployError> {
    let p = Path::new(args.output());
    let path = p
        .to_str()
        .ok_or_else(|| DeployError::new(&format!("not valid path {}", args.output())))?;

    if args.parents() {
        fs::create_dir_all(p)
            .map_err(|e| DeployError::new(&format!("cannot create directory: {}", e)))?;
    }

    match args.lang() {
        LangType::Go => {
            let dep = GolangDeployer::new(path);
            match args.func() {
                FuncType::Http => {
                    dep.deploy_http()?;
                }
                FuncType::Event => {
                    dep.deploy_event()?;
                }
            };

            dep.add_dependency()?
        }
        LangType::Java => unimplemented!(),
        LangType::Node => {
            let dep = NodeDeployer::new(path);
            match args.func() {
                FuncType::Http => {
                    dep.deploy_http()?;
                }
                FuncType::Event => {
                    dep.deploy_event()?;
                }
            };

            dep.add_dependency()?
        }
        LangType::CSharp => unimplemented!(),
        LangType::Php => {
            let dep = PhpDeployer::new(path);
            match args.func() {
                FuncType::Http => {
                    dep.deploy_http()?;
                }
                FuncType::Event => {
                    dep.deploy_event()?;
                }
            };

            dep.add_dependency()?
        }
        LangType::Ruby => {
            let dep = RubyDeployer::new(path);
            match args.func() {
                FuncType::Http => {
                    dep.deploy_http()?;
                }
                FuncType::Event => {
                    dep.deploy_event()?;
                }
            };

            dep.add_dependency()?
        }
        LangType::Python => {
            let dep = PythonDeployer::new(path);
            match args.func() {
                FuncType::Http => {
                    dep.deploy_http()?;
                }
                FuncType::Event => {
                    dep.deploy_event()?;
                }
            };

            dep.add_dependency()?
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    match deploy(&args) {
        Ok(_) => {
            println!("Initial setup of function has been completed.");
            exit(0)
        }
        Err(e) => panic!("{}", e),
    }
}
