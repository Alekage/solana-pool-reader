use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            api: ApiConfig {
                timeout: Duration::from_secs(10),
            },
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut config = Config::default();
        
        if let Ok(host) = std::env::var("HOST") {
            config.server.host = host;
        }
        
        if let Ok(port) = std::env::var("PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.server.port = port_num;
            }
        }
        
        if let Ok(timeout) = std::env::var("API_TIMEOUT_SECS") {
            if let Ok(timeout_num) = timeout.parse::<u64>() {
                config.api.timeout = Duration::from_secs(timeout_num);
            }
        }
        
        config
    }
} 