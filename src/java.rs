use std::sync::Arc;

#[derive(Debug, PartialEq, Clone)]
pub struct Jvm {
    pub(crate) version: Arc<str>,
    pub(crate) path: Arc<str>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Javac {
    pub(crate) version: Arc<str>,
    pub(crate) path: Arc<str>,
}

use crate::{info, versions::Toolchain};
use duct::cmd;
use owo_colors::OwoColorize;
use std::path::PathBuf;

pub fn compile(
    compiler: Toolchain,
    path: PathBuf,
    outpath: Option<PathBuf>,
) -> Result<String, String> {
    let compiler = compiler.javac;
    println!(
        "{}",
        info!("Compile", "using javac version: {}", &compiler.version)
    );

    let output_path = if let Some(out_path) = outpath {
        out_path
    } else {
        path.parent()
            .map(|p| p.join("build"))
            .unwrap_or_else(|| PathBuf::from(".out"))
    };

    if !output_path.exists() {
        std::fs::create_dir_all(&output_path).map_err(|e| e.to_string())?;
    }

    let output = cmd!(compiler.path.to_string(), "-d", &output_path, path)
        .stderr_to_stdout()
        .stdout_capture()
        .unchecked()
        .run()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(stdout)
    }
}

// pub fn run(path: PathBuf) -> Result<String, String> {}
//
