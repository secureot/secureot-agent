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
use clap::{Command, Arg};

fn parse_args() -> Command {
    Command::new("SecureOT Insight Agent")
        .about("Captura y retransmisión de tráfico OT segura")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .help("Ruta del archivo de configuración"))
        .arg(Arg::new("log_level")
            .short('l')
            .long("log-level")
            .help("Define el nivel de logs (info, debug, error)"))
}

#[tokio::main]
async fn main() {
    let matches = parse_args().get_matches();

    if matches.contains_id("help") {
        println!("{}", parse_args().render_usage());
        return;
    }

    let config = read_config();
    display_config(&config); // Se usa la función `display_config`
    
    let filters = Arc::new(Mutex::new(vec!["tcp port 502".to_string(), "udp port 161".to_string()]));
    let app_state = Arc::new(init_metrics());

    let (_tx, mut rx) = mpsc::channel::<Vec<u8>>(100);

    tokio::spawn(start_server(app_state.clone(), filters.clone()));

    tokio::spawn(async move {
        start_transmission(&config.host, config.port, &[0x45, 0x00, 0x32], config.ssl_enabled)
            .await.unwrap();
    });

    while let Some(packet) = rx.recv().await {
        println!("📡 Paquete procesado: {:?}", packet);
    }
}
