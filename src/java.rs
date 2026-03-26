use duct::cmd;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Jvm {
    pub(crate) version: String,
    pub(crate) path: String,
}

#[derive(Debug, PartialEq)]
pub struct Javac {
    pub(crate) version: String,
    pub(crate) path: String,
}

#[derive(Debug)]
pub struct Toolchain {
    pub jvm: Jvm,
    pub javac: Javac,
}

fn find_tool(tool: &str) -> Vec<(String, String)> {
    let Ok(paths) = which::which_all_global(tool) else {
        return Vec::new();
    };
    paths
        .filter_map(|p| {
            let version_output = cmd!(&p, "--version").read().ok()?;
            let version = version_output
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))?
                .to_string();
            let path = p.into_os_string().into_string().ok()?;
            Some((version, path))
        })
        .collect()
}

pub fn find_jvm() -> Result<Vec<Jvm>, String> {
    find_tool("java")
        .into_iter()
        .map(|(version, path)| Ok(Jvm { version, path }))
        .collect()
}

pub fn find_javac() -> Result<Vec<Javac>, String> {
    find_tool("javac")
        .into_iter()
        .map(|(version, path)| Ok(Javac { version, path }))
        .collect()
}

pub fn list_available() -> Result<(), String> {
    let jvms = find_jvm()?;
    let javacs = find_javac()?;

    println!("JVM versions:");
    print_grouped(
        &jvms
            .iter()
            .map(|j| (&j.version, &j.path))
            .collect::<Vec<_>>(),
    );

    println!("\nJavaC versions:");
    print_grouped(
        &javacs
            .iter()
            .map(|j| (&j.version, &j.path))
            .collect::<Vec<_>>(),
    );
    std::process::exit(0);
}

fn print_grouped(entries: &[(&String, &String)]) {
    let mut grouped: std::collections::BTreeMap<&String, Vec<&String>> =
        std::collections::BTreeMap::new();
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
    get_v: impl Fn(&'a T) -> &'a str,
    get_p: impl Fn(&'a T) -> &'a str,
) -> Option<&'a T> {
    entries
        .iter()
        .find(|e| get_p(e) == query)
        .or_else(|| entries.iter().find(|e| get_v(e) == query))
}

pub fn set_default(
    jvms: &[Jvm],
    javacs: &[Javac],
    jvm_query: &str,
    javac_query: Option<&str>,
) -> Result<Toolchain, String> {
    let javac_query = javac_query.unwrap_or(jvm_query);

    let jvm = resolve(jvms, jvm_query, |j| &j.version, |j| &j.path)
        .ok_or_else(|| format!("No JVM found matching '{}'", jvm_query))?;

    let javac = resolve(javacs, javac_query, |j| &j.version, |j| &j.path)
        .ok_or_else(|| format!("No JavaC found matching '{}'", javac_query))?;

    Ok(Toolchain {
        jvm: Jvm {
            version: jvm.version.clone(),
            path: jvm.path.clone(),
        },
        javac: Javac {
            version: javac.version.clone(),
            path: javac.path.clone(),
        },
    })
}
fn compile(compiler: Javac) -> Result<String, String> {
    Ok("".to_string())
}

// pub fn run(path: PathBuf) -> Result<String, String> {}
//

#[cfg(test)]
mod test {
    use crate::warning;

    use super::*;
    use owo_colors::OwoColorize;
    #[test]
    fn test() {
        println!("{}", warning!("{}", "hello"));
    }
}
