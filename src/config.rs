#![allow(dead_code)] // TODO: remove when convienent

use log::{error, info};
use serde::Deserialize;
use std::fs;
use std::fs::File;
/// Loads the user's config, if there is one.
/// Otherwise, it creates a config with default options.

/// Represents the configuration file given by the user.
#[derive(Deserialize)]
pub struct Configuration {
    file_name: String,
    api_config: APIConfig,
    settings_config: SettingsConfig,
}

#[derive(Deserialize)]
struct APIConfig {
    bot_key: String,
    channels: Vec<String>,
}

#[derive(Deserialize)]
struct SettingsConfig {
    checking_rate: u32,
    check_rate_unit: String,
    url: String,
    retries: u32,
}

/// Represents time value for check_rate_unit.
pub enum Time {
    Second,
    Minute,
    Hour,
}

impl Configuration {
    /// Scans the config for bad options.
    fn check_config() {
        unimplemented!();
    }

    /// Returns the bot API key.
    fn get_bot_key(&self) -> String {
        self.api_config.bot_key.clone()
    }

    fn get_channels(&self) -> Vec<String> {
        self.api_config.channels.clone()
    }

    fn get_checking_rate(&self) -> u32 {
        self.settings_config.checking_rate
    }

    /// Returns the checking rate unit as a type.
    fn get_check_rate_unit(&self) -> Time {
        match self
            .settings_config
            .check_rate_unit
            .to_ascii_lowercase()
            .as_str()
        {
            "s" | "sec" | "second" => Time::Second,
            "m" | "min" | "minute" => Time::Minute,
            "h" | "hr" | "hour" => Time::Hour,
            other => {
                panic!("Invalid configuration option. The check_rate_unit option is supposed to be a second, minute, or hour, but was actually: {}.", other);
            }
        }
    }

    // TODO: get the other ones
}

pub fn load_config(file_name: Option<&str>, recursive: bool) -> Configuration {
    let config_content = match file_name {
        Some(given_file_name) => fs::read_to_string(given_file_name),
        None => {
            info!("Configuration file not given. Assuming it's located at the standard config location...");
            let config_directory = dirs::config_dir().expect("Error when attempting to create a configuration file: wasn't able to find a configuration dir for your platform... ");
            Ok(format!(
                "{}/text_on_server_death/config.toml",
                config_directory
                    .to_str()
                    .expect("Couldn't find your configuration directory!")
            ))
        }
    };

    match config_content {
        Ok(content) => toml::from_str::<Configuration>(&content).unwrap(),
        Err(error) => {
            error!(
                "Unable to find given configuration file. Attempting to create one for you... {}",
                error.to_string()
            );

            // Let's stop an infinite loop today :)
            if recursive {
                panic!("Couldn't create your configuration file. Sorry!");
            }

            match create_config_file() {
                Ok(_) => {
                    // Attempt to write the default configuration to the new config file!
                    info!("Sucessfully created new configuration file!");
                    load_config(None, true)
                }
                Err(file_error) => {
                    error!("We couldn't create a config file for you...");
                    panic!("{}", file_error.to_string());
                }
            }
        }
    }
}

fn create_config_file() -> Result<File, std::io::Error> {
    let config_directory = dirs::config_dir().expect("Error when attempting to create a configuration file: wasn't able to find a configuration dir for your platform... ");
    File::create(config_directory)
}
