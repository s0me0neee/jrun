use duct::cmd;
use owo_colors::OwoColorize;
use std::collections::BTreeMap;
use std::process;

use crate::java::{Javac, Jvm};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Toolchain {
    pub jvm: Jvm,
    pub javac: Javac,
}

pub fn get_tool_info(path: PathBuf) -> Option<(String, String)> {
    let version_output = cmd!(&path, "--version").read().ok()?;
    let version = version_output
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))?
        .to_string();
    let path_str = path.into_os_string().into_string().ok()?;
    Some((version, path_str))
}

fn find_tool(tool: &str) -> Vec<(String, String)> {
    let Ok(paths) = which::which_all_global(tool) else {
        return Vec::new();
    };
    paths.filter_map(get_tool_info).collect()
}

pub fn find_jvm() -> Result<Vec<Jvm>, String> {
    find_tool("java")
        .into_iter()
        .map(|(version, path)| {
            Ok(Jvm {
                version: version.into(),
                path: path.into(),
            })
        })
        .collect()
}

pub fn find_javac() -> Result<Vec<Javac>, String> {
    find_tool("javac")
        .into_iter()
        .map(|(version, path)| {
            Ok(Javac {
                version: version.into(),
                path: path.into(),
            })
        })
        .collect()
}

pub fn list_available() -> Result<(), String> {
    let jvms = find_jvm()?;
    let javacs = find_javac()?;

    println!("{}", crate::info!("Version", "JVM versions:"));
    print_grouped(
        &jvms
            .iter()
            .map(|j| (j.version.as_ref(), j.path.as_ref()))
            .collect::<Vec<_>>(),
    );

    println!("\n{}", crate::info!("Version", "JavaC versions:"));
    print_grouped(
        &javacs
            .iter()
            .map(|j| (j.version.as_ref(), j.path.as_ref()))
            .collect::<Vec<_>>(),
    );
    Ok(())
}

fn print_grouped(entries: &[(&str, &str)]) {
    let mut grouped: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (version, path) in entries {
        grouped.entry(version).or_default().push(path);
    }
    for (version, paths) in &grouped {
        if paths.len() == 1 {
            println!("  {} — {}", version, paths[0]);
        } else {
            println!("  {}:", version);
            for path in paths {
                println!("    - {}", path);
            }
        }
    }
}

fn resolve<'a, T>(
    entries: &'a [T],
    query: &str,
    get_v: impl Fn(&'a T) -> &str,
    get_p: impl Fn(&'a T) -> &str,
) -> Option<&'a T> {
    entries.iter().find(|e| get_p(e) == query).or_else(|| {
        entries.iter().find(|e| {
            let v = get_v(e);
            v == query
                || (v.starts_with(query)
                    && v.as_bytes().get(query.len()) == Some(&b'.'))
        })
    })
}

pub fn query(
    jvms: &[Jvm],
    javacs: &[Javac],
    javac_query: &str,
    jvm_query: Option<&str>,
) -> Result<Toolchain, String> {
    let jvm_query = jvm_query.unwrap_or(javac_query);

    let jvm = resolve(jvms, jvm_query, |j| &j.version, |j| &j.path)
        .ok_or_else(|| {
            format!(
                "No JVM found matching '{}' is the executable in PATH?",
                jvm_query
            )
        })?;

    let javac = resolve(javacs, javac_query, |j| &j.version, |j| &j.path)
        .ok_or_else(|| {
            format!(
                "No JavaC found matching '{}' is the executable in PATH?",
                javac_query
            )
        })?;

    Ok(Toolchain {
        jvm: jvm.clone(),
        javac: javac.clone(),
    })
}
