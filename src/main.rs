use clap::{Parser, error};
use owo_colors::OwoColorize;
use std::{
    path::{self, PathBuf},
    process::exit,
};
mod color;
mod file;
mod java;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "Output path")]
    output: Option<PathBuf>,

    #[arg()]
    #[clap(value_parser)]
    file: String,
}

fn main() {
    match dotenvy::dotenv() {
        Ok(o) => {
            println!(
                "{}",
                info!(
                    "Env",
                    "loaded env: {}",
                    std::path::absolute(o)
                        .expect("Env path dosen't exist")
                        .display()
                )
            );
        }
        Err(e) => {
            eprintln!("{}", error!(e));
        }
    }
    env_logger::init();

    let args = Args::parse();
    let path = match file::validate_file(PathBuf::from(&args.file)) {
        Ok(p) => p,
        Err(e) => {
            let epath = &args.file;
            eprintln!("{}", error!("{} for target: {}", e, epath));
            exit(1);
        }
    };
}
