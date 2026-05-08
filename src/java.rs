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
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

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

/// Runs the compiled class, captures output, and pretty-prints it.
/// Returns the wall-clock duration of the process on success.
pub async fn run(jvm: &Jvm, class_dir: &Path, class_name: &str) -> Result<Duration, String> {
    use crate::pretty;
    use std::process::Stdio;

    println!("{}", info!("Run", "using jvm {}", &jvm.version));

    let start = tokio::time::Instant::now();

    let output = tokio::process::Command::new(jvm.path.as_ref())
        .args(["-cp", &class_dir.to_string_lossy(), class_name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let elapsed = start.elapsed();

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.is_empty() {
        print!("{}", stdout);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        eprintln!("{}", pretty::colorize_runtime_stderr(stderr.trim_end()));
    }

    if output.status.success() {
        Ok(elapsed)
    } else {
        Err(format!("process exited with {}", output.status))
    }
}
