mod transmit;
mod api;
mod metrics;
mod config;

use crate::transmit::start_transmission;
use crate::api::start_server;
use crate::metrics::init_metrics;
use crate::config::{read_config, display_config};
use std::sync::Mutex;
use tokio::sync::mpsc;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = read_config();
    display_config(&config); // Se usa `display_config` para mostrar configuración inicial

    let mode = if config.api_enabled { "tls" } else { "tcp" }; // Convertir `bool` a `&str`

    let filters = Arc::new(Mutex::new(vec!["tcp port 502".to_string(), "udp port 161".to_string()]));
    let app_state = Arc::new(init_metrics());

    let (_tx, mut rx) = mpsc::channel::<Vec<u8>>(100);

    tokio::spawn(start_server(app_state.clone(), filters.clone()));

    tokio::spawn(async move {
        start_transmission(&config.api_port.to_string(), config.api_port, &[0x45, 0x00, 0x32], mode)
            .await.unwrap();
    });

    while let Some(packet) = rx.recv().await {
        println!("📡 Paquete procesado: {:?}", packet);
    }
}
