use std::path::{self, PathBuf};

#[macro_export]
macro_rules! default_setting_path {
    () => {{
        let mut p = dirs::config_dir().expect("Can't get config path for your system");
        p.push("jrun2");
        p.push("setting.json");
        p
    }};
}

#[macro_export]
macro_rules! default_config_path {
    () => {{
        let mut p = dirs::config_dir().expect("Can't get config path for your system");
        p.push("jrun2");
        p.push("config.json");
        p
    }};
}

pub fn validate_file(path: PathBuf) -> Result<PathBuf, String> {
    let path = path::absolute(path).map_err(|e| e.to_string())?;
    if !path.exists() {
        return Err("Path does not exist".to_string());
    }
    if path.is_dir() {
        return Err("Path is a directory, expected a .java file".to_string());
    }
    match path.extension() {
        Some(ext) if ext == "java" => Ok(path),
        Some(_) => Err("File does not have a .java extension".to_string()),
        None => Err("File does not have an extension".to_string()),
    }
}
