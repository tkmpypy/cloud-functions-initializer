use std::process::exit;

use clap::Parser;
use cloud_functions_initializer::{
    command::{Args, FuncType, LangType},
    deploy::{
        golang::GolangDeployer, node::NodeDeployer, php::PhpDeployer, python::PythonDeployer,
        ruby::RubyDeployer, DeployError, Deployer,
    },
};

fn deploy(args: &Args) -> Result<(), DeployError> {
    match args.lang() {
        LangType::Go => {
            let dep = GolangDeployer::new(args.output());
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
            let dep = NodeDeployer::new(args.output());
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
            let dep = PhpDeployer::new(args.output());
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
            let dep = RubyDeployer::new(args.output());
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
            let dep = PythonDeployer::new(args.output());
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
