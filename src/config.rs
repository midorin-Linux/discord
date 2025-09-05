use anyhow::Result;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "DATABASE_URL")]
    pub database_url: String,

    #[serde(rename = "DISCORD_TOKEN")]
    pub discord_token: String,

    #[serde(rename = "GUILD_ID")]
    pub guild_id: String,

    #[serde(default = "default_timeout")]
    pub request_timeout_secs: u64,
}

fn default_timeout() -> u64 { 10 }

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let _ = dotenvy::dotenv();

        let config = ConfigBuilder::builder()
            .add_source(
                File::with_name(".env")
                    .format(config::FileFormat::Ini)
                    .required(false)
            )
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.discord_token.is_empty() {
            return Err("Discord token cannot be empty".to_string());
        }

        if self.guild_id.parse::<u64>().is_err() {
            return Err("Guild ID must be a valid number".to_string());
        }

        Ok(())
    }
}