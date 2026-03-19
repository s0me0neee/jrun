use std::path::{self, PathBuf};

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
