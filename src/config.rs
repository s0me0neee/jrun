use super::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub jvm_path: PathBuf,
    pub javac_path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Setting {
    pub(crate) config_path: PathBuf,
}

impl Config {
    pub fn read(path: &PathBuf) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), String> {
        let content = serde_json::to_string(&self).map_err(|e| e.to_string())?;
        std::fs::write(path, content).map_err(|e| e.to_string())
    }
}

impl Setting {
    pub fn read() -> Result<Self, String> {
        let path = default_setting_path!();
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    pub fn write(setting: Setting) -> Result<(), String> {
        let path = default_setting_path!();
        let content = serde_json::to_string(&setting).map_err(|e| e.to_string())?;
        std::fs::write(path, content).map_err(|e| e.to_string())
    }
}
