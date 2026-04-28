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
        if path.exists() {
            let content =
                std::fs::read_to_string(path).map_err(|e| e.to_string())?;
            let config: Config =
                serde_json::from_str(&content).map_err(|e| e.to_string())?;
            Ok(config)
        } else {
            Err("Config file does not exist".to_string())
        }
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), String> {
        let content =
            serde_json::to_string(&self).map_err(|e| e.to_string())?;
        std::fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl Setting {
    pub fn read() -> Result<Self, String> {
        let config_path = default_setting_path!();
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)
                .map_err(|e| e.to_string())?;
            let setting: Setting =
                serde_json::from_str(&content).map_err(|e| e.to_string())?;
            Ok(setting)
        } else {
            Err("Config path dose not exists".to_string())
        }
    }

    pub fn write(setting: Setting) -> Result<(), String> {
        let setting_path = default_setting_path!();
        let content =
            serde_json::to_string(&setting).map_err(|e| e.to_string())?;
        std::fs::write(setting_path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}
