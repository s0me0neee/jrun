use clap::Parser;
use owo_colors::OwoColorize;
use std::{path::PathBuf, process::exit};
use which::which_global;

use crate::{
    config::{Config, Setting},
    java::{Javac, Jvm},
    versions::{
        find_javac, find_jvm, find_one_javac, find_one_jvm, get_tool_info,
        java_major_version, list_available, Toolchain,
    },
};
mod config;
mod file;
mod java;
mod log;
mod pretty;
mod versions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Java source file to compile and run
    #[arg(required_unless_present_any = ["list", "set_default"])]
    file: Option<String>,

    /// Output directory for compiled .class files
    #[arg(short, long, value_name = "DIR")]
    output: Option<String>,

    /// JavaC version or path to use (e.g. "21", "21.0.2", "/usr/bin/javac")
    #[arg(long, value_name = "VERSION_OR_PATH")]
    javac: Option<String>,

    /// JVM version or path to use; defaults to --javac value if not set
    #[arg(long, value_name = "VERSION_OR_PATH")]
    jvm: Option<String>,

    /// List all detected Java/JavaC installations
    #[arg(short, long)]
    list: bool,

    /// Persist the selected toolchain as the new default
    #[arg(short = 'd', long)]
    set_default: bool,
}

#[tokio::main]
async fn main() {
    setup_log();
    let args = Args::parse();

    let config_path = setting_init().unwrap_or_else(|e| {
        eprintln!("{}", error!("{}", e));
        exit(1);
    });

    let config = config_init(&config_path).unwrap_or_else(|e| {
        eprintln!("{}", error!("{}", e));
        exit(1);
    });

    if args.list {
        if let Err(e) = list_available() {
            eprintln!("{}", error!("{}", e));
        }
        if args.file.is_none() && !args.set_default {
            return;
        }
    }

    let toolchain = build_toolchain(&args, &config).unwrap_or_else(|e| {
        eprintln!("{}", error!("{}", e));
        exit(1);
    });

    if args.set_default {
        let new_config = Config {
            jvm_path: PathBuf::from(toolchain.jvm.path.as_ref()),
            javac_path: PathBuf::from(toolchain.javac.path.as_ref()),
        };
        if let Err(e) = new_config.write(&config_path) {
            eprintln!("{}", error!("Failed to save default: {}", e));
            exit(1);
        }
        println!("{}", info!("Config", "saved default toolchain:"));
        println!("  JVM:   {} — {}", toolchain.jvm.version, toolchain.jvm.path);
        println!("  JavaC: {} — {}", toolchain.javac.version, toolchain.javac.path);
        if args.file.is_none() {
            return;
        }
    }

    let file = match args.file {
        Some(f) => f,
        None => return,
    };

    let target_path = file::validate_file(PathBuf::from(&file)).unwrap_or_else(|e| {
        eprintln!("{}", error!("{}: {}", e, file));
        exit(1);
    });

    let class_name = target_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| {
            eprintln!("{}", error!("Cannot determine class name from file path"));
            exit(1);
        })
        .to_string();

    let output_path = args.output.map(PathBuf::from);

    if let (Some(javac_v), Some(jvm_v)) = (
        java_major_version(&toolchain.javac.version),
        java_major_version(&toolchain.jvm.version),
    ) {
        if javac_v > jvm_v {
            eprintln!(
                "{}",
                warning!(
                    "JavaC {} compiles to class file version {} which JVM {} cannot run \
                     — upgrade the JVM to {} or higher to avoid UnsupportedClassVersionError",
                    toolchain.javac.version,
                    javac_v,
                    toolchain.jvm.version,
                    javac_v
                )
            );
        }
    }

    let compile_start = std::time::Instant::now();
    let (out_msg, out_dir) =
        java::compile(&toolchain, target_path, output_path).unwrap_or_else(|e| {
            eprintln!("{}", error!("Compile failed:"));
            eprintln!("{}", pretty::colorize_compile_error(e.trim_end()));
            exit(1);
        });
    let compile_elapsed = compile_start.elapsed();

    if !out_msg.is_empty() {
        print!("{}", out_msg);
    }
    println!(
        "{}",
        info!("Compile", "success → {} ({})", out_dir.display(), fmt_duration(compile_elapsed))
    );

    match java::run(&toolchain.jvm, &out_dir, &class_name).await {
        Ok(elapsed) => {
            println!("{}", info!("Run", "finished in {}", fmt_duration(elapsed)));
        }
        Err(e) => {
            eprintln!("{}", error!("Run failed: {}", e));
            exit(1);
        }
    }
}

fn fmt_duration(d: std::time::Duration) -> String {
    let ms = d.as_secs_f64() * 1000.0;
    if ms < 1000.0 {
        format!("{:.1}ms", ms)
    } else {
        format!("{:.3}s", d.as_secs_f64())
    }
}

/// Resolves the toolchain from CLI args, falling back to config where not specified.
///
/// - `--javac X --jvm Y`: use X for javac, Y for jvm
/// - `--javac X` only:    use X for both javac and jvm
/// - `--jvm Y` only:      use Y for jvm, config default for javac
/// - neither:             use config defaults for both
fn build_toolchain(args: &Args, config: &Config) -> Result<Toolchain, String> {
    if args.javac.is_none() && args.jvm.is_none() {
        return toolchain_from_config(config);
    }

    let jvms = find_jvm()?;
    let javacs = find_javac()?;

    let javac = match &args.javac {
        Some(q) => find_one_javac(&javacs, q)?,
        None => {
            // --jvm set but not --javac: keep the config javac unchanged
            get_tool_info(config.javac_path.clone())
                .map(|(ver, path)| Javac { version: ver.into(), path: path.into() })
                .ok_or_else(|| format!("Cannot read JavaC at {:?}", config.javac_path))?
        }
    };

    let jvm = match &args.jvm {
        Some(q) => find_one_jvm(&jvms, q)?,
        None => {
            // --javac set but not --jvm: use the same query for jvm
            find_one_jvm(&jvms, args.javac.as_deref().unwrap())?
        }
    };

    Ok(Toolchain { jvm, javac })
}

fn toolchain_from_config(config: &Config) -> Result<Toolchain, String> {
    let (jvm_ver, jvm_path) = get_tool_info(config.jvm_path.clone())
        .ok_or_else(|| format!("Cannot read JVM at {:?}", config.jvm_path))?;
    let (javac_ver, javac_path) = get_tool_info(config.javac_path.clone())
        .ok_or_else(|| format!("Cannot read JavaC at {:?}", config.javac_path))?;
    Ok(Toolchain {
        jvm: Jvm { version: jvm_ver.into(), path: jvm_path.into() },
        javac: Javac { version: javac_ver.into(), path: javac_path.into() },
    })
}

fn setup_log() {
    // .env is optional; silently ignore if missing
    let _ = dotenvy::dotenv();
    env_logger::init();
}

fn setting_init() -> Result<PathBuf, String> {
    let setting_path = default_setting_path!();

    if setting_path.exists() {
        let setting =
            Setting::read().map_err(|e| format!("Failed to read setting: {}", e))?;
        if !setting.config_path.exists()
            && let Some(parent) = setting.config_path.parent()
        {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        return Ok(setting.config_path);
    }

    let def_config_path = default_config_path!();
    if let Some(parent) = setting_path.parent()
        && !parent.exists()
    {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    Setting::write(Setting { config_path: def_config_path.clone() })
        .map_err(|e| e.to_string())?;
    Ok(def_config_path)
}

fn config_init(config_path: &PathBuf) -> Result<Config, String> {
    if config_path.exists() {
        return Config::read(config_path);
    }

    let javac_path = which_global("javac")
        .map_err(|_| "Could not find 'javac' in PATH".to_string())?;
    let jvm_path = which_global("java")
        .map_err(|_| "Could not find 'java' in PATH".to_string())?;

    let config = Config { jvm_path, javac_path };
    config.write(config_path)?;
    Ok(config)
}
