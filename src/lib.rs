pub mod actuator;
pub mod cli;
pub mod utils;

use std::str::FromStr;

pub use cli::*;

pub trait Actuator {
    fn execute(self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub enum Base64Charset {
    Standard,
    StandardNoPad,
    UrlSaff,
    UrlSafeNoPad,
    None,
}

#[derive(Debug, Clone)]
pub enum DataFormat {
    Json,
    Yaml,
    Toml,
    Text,
}

impl FromStr for DataFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "json" => Ok(DataFormat::Json),
            "yaml" => Ok(DataFormat::Yaml),
            "toml" => Ok(DataFormat::Toml),
            "text" => Ok(DataFormat::Text),
            _ => Err(anyhow::anyhow!("Invalid data format: {}", s)),
        }
    }
}

impl From<DataFormat> for &str {
    fn from(df: DataFormat) -> Self {
        match df {
            DataFormat::Json => "json",
            DataFormat::Yaml => "yaml",
            DataFormat::Toml => "toml",
            DataFormat::Text => "text",
        }
    }
}

impl FromStr for Base64Charset {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "standard" => Ok(Base64Charset::Standard),
            "standard-nopad" => Ok(Base64Charset::StandardNoPad),
            "urlsafe" => Ok(Base64Charset::UrlSaff),
            "urlsafe-nopad" => Ok(Base64Charset::UrlSafeNoPad),
            "none" => Ok(Base64Charset::None),
            _ => Err(anyhow::anyhow!("Invalid base64 charset: {}", s)),
        }
    }
}

impl From<Base64Charset> for &str {
    fn from(cs: Base64Charset) -> Self {
        match cs {
            Base64Charset::Standard => "standard",
            Base64Charset::StandardNoPad => "standard-nopad",
            Base64Charset::UrlSaff => "urlsafe",
            Base64Charset::UrlSafeNoPad => "urlsafe-nopad",
            Base64Charset::None => "none",
        }
    }
}
