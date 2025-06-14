use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_enabled: bool,
    pub api_port: u16,
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub log_level: String,
}

pub fn read_config() -> Config {
    Config {
        api_enabled: true,
        api_port: 8080,
        metrics_enabled: true,
        metrics_port: 9090,
        log_level: "info".to_string(),
    }
}

pub fn display_config(config: &Config) {
    println!("🔧 Configuración Cargada:");
    println!("API Habilitada: {}", config.api_enabled);
    println!("Puerto API: {}", config.api_port);
    println!("Métricas Habilitadas: {}", config.metrics_enabled);
    println!("Puerto Métricas: {}", config.metrics_port);
    println!("Nivel de Log: {}", config.log_level);
}
