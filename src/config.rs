use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub api_key: String,
    pub model: String,
}

pub fn get_config_file() -> PathBuf {
    let home = env::var("HOME").expect("HOME directory not found");
    let folder = Path::new(&home).join(".zasto");
    let config_file_path = folder.join("config.toml");

    config_file_path
}

pub fn check_config() -> bool {
    let home = env::var("HOME").expect("HOME directory not found");
    let folder = Path::new(&home).join(".zasto");

    if !folder.exists() {
        fs::create_dir_all(&folder).expect("Failed to create Zasto directory");

        let default_config = Config {
            api_key: String::from("API_KEY_HERE"),
            model: String::from("google/gemma-4-26b-a4b-it"),
        };

        let toml_string =
            toml::to_string_pretty(&default_config).expect("Failed to serialize config");

        fs::write(get_config_file(), toml_string).expect("Failed to create config file");

        false
    } else {
        true
    }
}

pub fn get_toml_values() -> Result<Config, Box<dyn Error>> {
    let toml_str = fs::read_to_string(get_config_file())?;

    let config: Config = toml::from_str(&toml_str)?;

    Ok(config)
}

pub fn change_toml_value(entry: String, value: String) {
    let toml_str = fs::read_to_string(get_config_file()).expect("Failed to read config");
    let mut config: toml::Value = toml::from_str(&toml_str).expect("Failed to parse config");

    config[entry] = toml::Value::String(value);

    let updated_toml = toml::to_string_pretty(&config).expect("Failed to serialize config");

    fs::write(get_config_file(), updated_toml).expect("Failed to write config");
}
