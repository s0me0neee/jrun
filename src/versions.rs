use duct::cmd;
use owo_colors::OwoColorize;
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::java::{Javac, Jvm};

#[derive(Debug)]
pub struct Toolchain {
    pub jvm: Jvm,
    pub javac: Javac,
}

pub fn get_tool_info(path: PathBuf) -> Option<(String, String)> {
    let version_output = cmd!(&path, "--version").read().ok()?;
    let first_line = version_output.lines().next()?;
    let tokens: Vec<&str> = first_line.split_whitespace().collect();
    // Java 9+: "java 21.0.2"           → tokens[1]
    // Java 8:  "java version \"1.8.0\"" → tokens[2], strip surrounding quotes
    let raw = match tokens.get(1)? {
        &"version" => tokens.get(2)?.trim_matches('"'),
        v => v,
    };
    let path_str = path.into_os_string().into_string().ok()?;
    Some((raw.to_string(), path_str))
}

fn find_tool(tool: &str) -> Vec<(String, String)> {
    let Ok(paths) = which::which_all_global(tool) else {
        return Vec::new();
    };
    paths.filter_map(get_tool_info).collect()
}

pub fn find_jvm() -> Result<Vec<Jvm>, String> {
    Ok(find_tool("java")
        .into_iter()
        .map(|(version, path)| Jvm { version: version.into(), path: path.into() })
        .collect())
}

pub fn find_javac() -> Result<Vec<Javac>, String> {
    Ok(find_tool("javac")
        .into_iter()
        .map(|(version, path)| Javac { version: version.into(), path: path.into() })
        .collect())
}

pub fn find_one_jvm(jvms: &[Jvm], query: &str) -> Result<Jvm, String> {
    resolve(jvms, query, |j| &j.version, |j| &j.path)
        .cloned()
        .ok_or_else(|| format!("No JVM matching '{}' (is it in PATH?)", query))
}

pub fn find_one_javac(javacs: &[Javac], query: &str) -> Result<Javac, String> {
    resolve(javacs, query, |j| &j.version, |j| &j.path)
        .cloned()
        .ok_or_else(|| format!("No JavaC matching '{}' (is it in PATH?)", query))
}

pub fn list_available() -> Result<(), String> {
    let jvms = find_jvm()?;
    let javacs = find_javac()?;

    println!("{}", crate::info!("Version", "JVM versions:"));
    print_grouped(
        &jvms.iter().map(|j| (j.version.as_ref(), j.path.as_ref())).collect::<Vec<_>>(),
    );
    println!("\n{}", crate::info!("Version", "JavaC versions:"));
    print_grouped(
        &javacs.iter().map(|j| (j.version.as_ref(), j.path.as_ref())).collect::<Vec<_>>(),
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

/// Extracts the Java major version number from a version string.
/// Handles both modern format ("21.0.2" → 21) and legacy Java 8 format ("1.8.0_301" → 8).
pub fn java_major_version(version: &str) -> Option<u32> {
    let first: u32 = version.split('.').next()?.parse().ok()?;
    if first == 1 {
        // Legacy "1.x.y" format used by Java 8 and below
        version.split('.').nth(1)?.parse().ok()
    } else {
        Some(first)
    }
}

fn resolve<'a, T>(
    entries: &'a [T],
    query: &str,
    get_v: impl Fn(&'a T) -> &str,
    get_p: impl Fn(&'a T) -> &str,
) -> Option<&'a T> {
    // Exact path match first, then version prefix match (e.g. "21" matches "21.0.2")
    entries.iter().find(|e| get_p(e) == query).or_else(|| {
        entries.iter().find(|e| {
            let v = get_v(e);
            v == query
                || (v.starts_with(query) && v.as_bytes().get(query.len()) == Some(&b'.'))
        })
    })
}
