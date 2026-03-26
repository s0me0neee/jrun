use std::path::{self, PathBuf};

#[macro_export]
macro_rules! defult_setting_path {
    () => {{
        let mut config_path = dirs::config_dir().ok_or_else(|| {
            "Can't get config path for your system".to_string()
        })?;
        config_path.push("setting.json");
        config_path
    }};
}

#[macro_export]
macro_rules! defult_config_path {
    () => {{
        let mut config_path = dirs::config_dir().ok_or_else(|| {
            "Can't get config path for your system".to_string()
        })?;
        config_path.push("config.json");
        config_path
    }};
}

pub fn validate_file(path: PathBuf) -> Result<PathBuf, String> {
    let path = path::absolute(path).map_err(|e| e.to_string())?;

    if !path.exists() {
        return Err("Provided path dose not exist".to_string());
    }

    if path.is_dir() {
        return Err("Provided path is a directory".to_string());
    }

    match path.extension() {
        Some(s) => {
            if !(s.eq("java")) {
                return Err("File extension is not .java".to_string());
            }
        }
        None => {
            return Err("File dose not have a extension".to_string());
        }
    }

    Ok(path)
}

pub fn validate_path(path: PathBuf) -> Result<PathBuf, String> {
    let path = path::absolute(path).map_err(|e| e.to_string())?;

    if !path.exists() {
        return Err("Provided path dose not exist".to_string());
    }

    if path.is_dir() {
        return Err("Provided path is a directory".to_string());
    }

    match path.extension() {
        Some(s) => {
            if !(s.eq("java")) {
                return Err("File extension is not .java".to_string());
            }
        }
        None => {
            return Err("File dose not have a extension".to_string());
        }
    }

    Ok(path)
}
