use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
pub enum Engine {
    Local(String),
    Api(Api),
}

#[derive(Debug, Deserialize)]
pub struct Api {
    endpoint: String,
    key: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub engine: String,
    pub local_model_path: Option<String>,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let config_contents = fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&config_contents)?;
        if config.engine == "api" {
            let api_key = env::var("MELANGE_API_KEY")
                .map_err(|_| anyhow::anyhow!("MELANGE_API_KEY environment variable not set"))?;
            config.api_key = Some(api_key);
        }
        Ok(config)
    }
    pub fn engine(&self) -> Engine {
        if self.engine == "api" {
            Engine::Api(Api {
                endpoint: self.api_endpoint.clone().unwrap(),
                key: self.api_key.clone().unwrap(),
            })
        } else {
            Engine::Local(self.local_model_path.clone().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = Config::from_file("tests/fixtures/melange-config.toml").unwrap();
        assert_eq!(config.engine, "local");
        assert!(config.local_model_path.is_some());
    }
}
