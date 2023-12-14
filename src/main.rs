use std::process::exit;

use clap::Parser;
use cloud_functions_initializer::{
    command::{Args, FuncType, LangType},
    deploy::{
        golang::GolangDeployer, node::NodeDeployer, php::PhpDeployer, python::PythonDeployer,
        ruby::RubyDeployer, DeployError, Deployer,
    },
};

fn deploy(lang: &LangType, ty: &FuncType, output: &str) -> Result<(), DeployError> {
    match lang {
        LangType::Go => {
            let dep = GolangDeployer::new(output);
            match ty {
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
            let dep = NodeDeployer::new(output);
            match ty {
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
            let dep = PhpDeployer::new(output);
            match ty {
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
            let dep = RubyDeployer::new(output);
            match ty {
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
            let dep = PythonDeployer::new(output);
            match ty {
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
    match deploy(args.lang(), args.func(), args.output()) {
        Ok(_) => {
            println!("Initial setup of function has been completed.");
            exit(0)
        }
        Err(e) => panic!("{}", e),
    }
}
