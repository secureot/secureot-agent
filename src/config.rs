use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub interface: String,
    pub host: String,
    pub port: u16,
    pub ssl_enabled: bool,
    pub mode: String,
    pub api_enabled: bool,
    pub api_port: u16,
    pub filters: Vec<String>,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub log_level: String,
}

pub fn read_config() -> Config {
    let config_str = fs::read_to_string("config/agent.toml").expect("Error al leer configuración");
    toml::from_str(&config_str).expect("Error al parsear configuración")
}

pub fn display_config(config: &Config) {
    println!("🔹 Configuración del Agente:");
    println!("  - Interfaz de captura: {}", config.interface);
    println!("  - Destino: {}:{}", config.host, config.port);
    println!("  - SSL habilitado: {}", config.ssl_enabled);
    println!("  - Modo de operación: {}", config.mode);
    println!("  - API REST habilitada: {} en puerto {}", config.api_enabled, config.api_port);
    println!("  - Filtros BPF activos: {:?}", config.filters);
    println!("  - Certificado SSL: {:?}", config.cert_file);
    println!("  - Clave privada SSL: {:?}", config.key_file);
    println!("  - Métricas activadas: {} en puerto {}", config.metrics_enabled, config.metrics_port);
    println!("  - Nivel de logs: {}", config.log_level);
}
// Actualizacion de codigo 2025-06-10
// Actualizacion de codigo 2025-06-03
// Actualizacion de codigo 2025-05-28
// Actualizacion de codigo 2025-05-26
// Actualizacion de codigo 2025-05-24
// Actualizacion de codigo 2025-05-23
// Actualizacion de codigo 2025-05-19
// Actualizacion de codigo 2025-05-17
// Actualizacion de codigo 2025-05-14
// Actualizacion de codigo 2025-05-13
// Actualizacion de codigo 2025-05-23
