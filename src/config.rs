use super::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    jvm: PathBuf,
    javac: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Setting {
    pub(crate) config_path: PathBuf,
}

impl Config {
    fn read() -> Result<Self, String> {
        let config_path = Setting::read()?.config_path;
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)
                .map_err(|e| e.to_string())?;
            serde_json::from_str(&content).map_err(|e| e.to_string())?
        } else {
            Err("Config path dose not exists".to_string())
        }
    }

    fn write(setting: Setting) -> Result<(), String> {
        let config_path = Setting::read()?.config_path;
        let content =
            serde_json::to_string(&setting).map_err(|e| e.to_string())?;
        std::fs::write(config_path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl Setting {
    pub fn read() -> Result<Self, String> {
        let config_path = defult_setting_path!();
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)
                .map_err(|e| e.to_string())?;
            serde_json::from_str(&content).map_err(|e| e.to_string())?
        } else {
            Err("Config path dose not exists".to_string())
        }
    }

    pub fn write(setting: Setting) -> Result<(), String> {
        let config_path = defult_setting_path!();
        let content =
            serde_json::to_string(&setting).map_err(|e| e.to_string())?;
        std::fs::write(config_path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}
