use clap::Parser;
use owo_colors::OwoColorize;
use std::{
    path::{Path, PathBuf},
    process::exit,
};
use which::{which, which_global};

use crate::{
    config::{Config, Setting},
    versions::{
        Toolchain, find_javac, find_jvm, get_tool_info, list_available, query,
    },
};
mod config;
mod file;
mod java;
mod log;
mod versions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "Output path")]
    output: Option<String>,

    #[arg(short, long, value_names = ["Compiler", "Jvm"], num_args = 2)]
    toolchain: Option<Vec<String>>,

    #[arg(required_unless_present_any = ["list", "set_default"])]
    file: Option<String>,

    #[arg(short, long, help = "List all available java versions")]
    list: bool,

    #[arg(short, long, value_names = ["Compiler", "Jvm"], num_args = 2, help = "Set the default version (e.g. --set-default javac 21)")]
    set_default: Option<Vec<String>>,
}

fn main() {
    setup_log();
    let args = Args::parse();

    let config_path = match setting_init() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", error!("{}", e));
            exit(1);
        }
    };

    let config = match config_init(config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", error!("{}", e));
            exit(1);
        }
    };

    if args.list {
        match list_available() {
            Ok(_) => {
                return;
            }
            Err(e) => {
                println!("{}", error!(e))
            }
        }
    }

    let input_file = match args.file {
        Some(f) => f,
        None => {
            if args.list
                || args.set_default.is_some()
                || args.toolchain.is_some()
            {
                return;
            }
            eprintln!("{}", error!("No file specified"));
            std::process::exit(1);
        }
    };

    let target_path = match file::validate_file(PathBuf::from(&input_file)) {
        Ok(p) => p,
        Err(e) => {
            let epath = &input_file;
            eprintln!("{}", error!("{} for target: {}", e, epath));
            std::process::exit(1);
        }
    };

    let jvms = find_jvm().unwrap_or_else(|e| {
        eprintln!("{}", error!(e));
        exit(1);
    });
    let javacs = find_javac().unwrap_or_else(|e| {
        eprintln!("{}", error!(e));
        exit(1);
    });

    //TODO: logic not completed yet.
    if let Some(ref input_version) = args.set_default {
        if input_version.len() != 2 {
            eprintln!(
                "{}",
                error!("Set default must be 2 values (compiler, jvm)")
            );
            std::process::exit(1);
        }
        let toolchain =
            query(&jvms, &javacs, &input_version[0], Some(&input_version[1]))
                .unwrap_or_else(|e| {
                    eprintln!("{}", error!(e));
                    exit(1);
                });
        println!("{}", crate::info!("Toolchain", "set default to:"));
        println!(
            "  JVM:   {} — {}",
            toolchain.jvm.version, toolchain.jvm.path
        );
        println!(
            "  JavaC: {} — {}",
            toolchain.javac.version, toolchain.javac.path
        );
        return;
    }

    let current_toolchain = if let Some(ref input_version) = args.toolchain {
        if input_version.len() != 2 {
            eprintln!(
                "{}",
                error!("Toolchain must be 2 values (compiler, jvm)")
            );
            std::process::exit(1);
        }
        query(&jvms, &javacs, &input_version[0], Some(&input_version[1]))
            .unwrap_or_else(|e| {
                eprintln!("{}", error!(e));
                exit(1);
            })
    } else {
        dbg!(&config);
        let (jvm_version, jvm_path) = get_tool_info(config.jvm_path.clone())
            .unwrap_or_else(|| {
                eprintln!(
                    "{}",
                    error!(
                        "Could not get JVM info from config path: {:?}",
                        config.jvm_path
                    )
                );
                exit(1);
            });
        let (javac_version, javac_path) =
            get_tool_info(config.javac_path.clone()).unwrap_or_else(|| {
                eprintln!(
                    "{}",
                    error!(
                        "Could not get JavaC info from config path: {:?}",
                        config.javac_path
                    )
                );
                exit(1);
            });

        Toolchain {
            jvm: crate::java::Jvm {
                version: jvm_version.into(),
                path: jvm_path.into(),
            },
            javac: crate::java::Javac {
                version: javac_version.into(),
                path: javac_path.into(),
            },
        }
    };

    if args.list {
        println!("{}", crate::info!("Toolchain", "using:"));
        println!(
            "  JVM:   {} — {}",
            current_toolchain.jvm.version, current_toolchain.jvm.path
        );
        println!(
            "  JavaC: {} — {}",
            current_toolchain.javac.version, current_toolchain.javac.path
        );
    }

    let output_path = args.output.map(PathBuf::from);
    match java::compile(current_toolchain, target_path, output_path) {
        Ok(i) => {
            println!("{}", info!("Compile", "Success {}", i.0));
            println!("{}", info!("Compile", "Out: {}", i.1.display()))
        }
        Err(e) => {
            eprintln!("{}", error!("Compile failed: {}", e));
            std::process::exit(1);
        }
    }
}

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

fn setting_init() -> Result<PathBuf, String> {
    let setting_path = default_setting_path!();

    if setting_path.exists() {
        let setting = Setting::read()
            .map_err(|e| format!("Failed to read existing setting: {}", e))?;
        if !setting.config_path.exists()
            && let Some(parent) = setting.config_path.parent()
        {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        return Ok(setting.config_path);
    }

    let def_config_path = default_config_path!();
    let def_setting = Setting {
        config_path: def_config_path.clone(),
    };

    if let Some(parent) = setting_path.parent()
        && !parent.exists()
    {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    Setting::write(def_setting).map_err(|e| e.to_string())?;
    Ok(def_config_path)
}

fn config_init(config_path: PathBuf) -> Result<Config, String> {
    if config_path.exists() {
        dbg!(&config_path);
        return Config::read(&config_path);
    }

    let def_c = which::which_global("javac")
        .map_err(|_| "Could not find 'javac' in PATH".to_string())?;
    let def_jvm = which::which_global("java")
        .map_err(|_| "Could not find 'java' in PATH".to_string())?;

    let def_config = Config {
        jvm_path: def_jvm,
        javac_path: def_c,
    };

    def_config.write(&config_path)?;
    Ok(def_config)
}
