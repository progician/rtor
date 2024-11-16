use clap::Parser;
use config::{Config, File, FileFormat};
use crate::LoggableError;
use serde::{Deserialize, Deserializer};
use std::path::{PathBuf};

pub struct DeserializablePath(PathBuf);

impl<'a> Deserialize<'a> for DeserializablePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        // Deserialize the string and convert it to PathBuf
        let path_str: String = String::deserialize(deserializer)?;
        Ok(DeserializablePath(PathBuf::from(path_str)))
    }
}

fn default_incoming_address() -> String {
    "0.0.0.0".to_string()
}

fn default_incoming_port() -> u16 {
    16384
}

fn default_torrent_info_storage() -> DeserializablePath {
    DeserializablePath(PathBuf::from("data"))
}

#[derive(Deserialize)]
pub struct Configuration {
    #[serde(default = "default_incoming_address")]
    pub incoming_address: String,

    #[serde(default = "default_incoming_port")]
    pub incoming_port: u16,

    #[serde(default = "default_torrent_info_storage")]
    pub torrent_info_storage: DeserializablePath,
}

#[derive(Parser)]
struct CommandLineConfig {
    #[arg(short, long, value_name="FILE", default_value="rtor.toml")]
    config: String,

    #[arg(long, value_name="ADDRESS")]
    incoming_address: Option<String>,

    #[arg(long, value_name="PORT_NUMBER")]
    incoming_port: Option<u16>,
}

pub fn load_config() -> Result<Configuration, LoggableError> {
    let args = CommandLineConfig::parse();

    let settings = Config::builder()
        .add_source(File::new(&args.config, FileFormat::Toml))
        .build()
        .map_err(|e| LoggableError::new("config", e.to_string()))?;

    let mut final_configuration: Configuration = settings
        .try_deserialize()
        .map_err(|e| LoggableError::new("config", e.to_string()))?;

    if let Some(arg_incoming_address) = args.incoming_address {
        final_configuration.incoming_address = arg_incoming_address;
    }
    if let Some(arg_incoming_port) = args.incoming_port {
        final_configuration.incoming_port = arg_incoming_port;
    }

    Ok(final_configuration)
}
