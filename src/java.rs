use duct::cmd;
use std::path::PathBuf;

pub struct Jdk {
    version: String,
    javac: PathBuf,
    jvm: PathBuf,
}

pub fn find_version() {
    let out = cmd!("which java").read();
    dbg!(out);
}

pub fn compile(
    path: PathBuf,
    compiler: PathBuf,
) -> Result<String, String> {
    Ok("".to_string())
}

// pub fn run(path: PathBuf) -> Result<String, String> {}
//

#[cfg(test)]
mod test {
    use crate::java::find_version;

    #[test]
    fn test() {
        find_version();
    }
}
