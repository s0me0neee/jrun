use clap::{ArgAction, Parser};
use owo_colors::OwoColorize;
use std::path::PathBuf;

use crate::{config::Setting, java::list_available};
mod color;
mod config;
mod file;
mod java;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "Output path", conflicts_with_all = ["list", "set_default"])]
    output: Option<String>,

    #[arg(short, long, value_names = ["Compiler", "Jvm"], num_args = 2, conflicts_with_all = ["list", "set_default"])]
    toolchain: Option<Vec<String>>,

    #[arg(conflicts_with_all = ["list", "set_default"])]
    file: Option<String>,

    #[arg(long, conflicts_with_all = ["file", "output", "toolchain", "set_default"], help = "List all available java versions")]
    list: bool,

    #[arg(long, value_names = ["Compiler", "Jvm"], num_args = 2, conflicts_with_all = ["file", "output", "toolchain", "list"], help = "Set the default version (e.g. --set-default javac 21)")]
    set_default: Option<Vec<String>>,
}

fn main() {
    setup_log();
    let args = Args::parse();

    if args.list
        && let Err(e) = list_available()
    {
        println!("{}", error!(e));
        std::process::exit(1);
    }
    let input_file = args.file.unwrap();

    let target_path = match file::validate_file(PathBuf::from(&input_file)) {
        Ok(p) => p,
        Err(e) => {
            let epath = &input_file;
            eprintln!("{}", error!("{} for target: {}", e, epath));
            std::process::exit(1);
        }
    };

    if let Err(e) = setting_init() {
        eprintln!("{}", error!("{}", e));
    };
}

#[inline(always)]
fn setup_log() -> String {
    let result = match dotenvy::dotenv() {
        Ok(o) => {
            format!(
                "{}\n",
                info!(
                    "Env",
                    "loaded env: {}",
                    std::path::absolute(o)
                        .expect("Env path dosen't exist")
                        .display()
                )
            )
        }
        Err(e) => error!(e).to_string(),
    };
    env_logger::init();
    result
}

fn setting_init() -> Result<(), String> {
    if !defult_setting_path!().exists() {
        let def_setting = Setting {
            config_path: defult_config_path!(),
        };
        Setting::write(def_setting)?
    }
    Ok(())
}
