use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub donetick_url: String,
    pub donetick_token: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let donetick_url = env::var("DONETICK_URL")
            .context("DONETICK_URL environment variable not set")?;
        let donetick_token = env::var("DONETICK_TOKEN")
            .context("DONETICK_TOKEN environment variable not set")?;

        let config = Self {
            donetick_url,
            donetick_token,
        };

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.donetick_url.is_empty() {
            anyhow::bail!("DONETICK_URL cannot be empty");
        }
        if self.donetick_token.is_empty() {
            anyhow::bail!("DONETICK_TOKEN cannot be empty");
        }
        if !self.donetick_url.starts_with("http://") && !self.donetick_url.starts_with("https://") {
            anyhow::bail!("DONETICK_URL must start with http:// or https://");
        }
        Ok(())
    }
}
