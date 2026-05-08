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
use std::path::{Path, PathBuf};

pub fn compile(
    toolchain: &Toolchain,
    path: PathBuf,
    outpath: Option<PathBuf>,
) -> Result<(String, PathBuf), String> {
    let javac = &toolchain.javac;
    println!("{}", info!("Compile", "using javac {}", &javac.version));

    let output_path = outpath.unwrap_or_else(|| {
        path.parent()
            .map(|p| p.join("build"))
            .unwrap_or_else(|| PathBuf::from(".out"))
    });

    if !output_path.exists() {
        std::fs::create_dir_all(&output_path).map_err(|e| e.to_string())?;
    }

    let output = cmd!(javac.path.as_ref(), "-d", &output_path, &path)
        .stderr_to_stdout()
        .stdout_capture()
        .unchecked()
        .run()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if output.status.success() {
        Ok((stdout, output_path))
    } else {
        Err(stdout)
    }
}

pub fn run(jvm: &Jvm, class_dir: &Path, class_name: &str) -> Result<String, String> {
    println!("{}", info!("Run", "using jvm {}", &jvm.version));

    let output = cmd!(jvm.path.as_ref(), "-cp", class_dir, class_name)
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
