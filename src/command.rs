use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum FuncType {
    Http,
    Event,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum LangType {
    Go,
    Java,
    Node,
    CSharp,
    Php,
    Ruby,
    Python,
}

#[derive(Parser, Clone)]
pub struct Args {
    #[arg(short, long)]
    #[clap(value_enum, default_value_t=LangType::Node)]
    lang: LangType,

    #[arg(short, long)]
    #[clap(value_enum, default_value_t=FuncType::Http)]
    func: FuncType,

    #[arg(short, long, default_value = ".")]
    output: String,
}

impl Args {
    pub fn lang(&self) -> &LangType {
        &self.lang
    }

    pub fn func(&self) -> &FuncType {
        &self.func
    }

    pub fn output(&self) -> &str {
        self.output.as_ref()
    }
}
