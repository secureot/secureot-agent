mod transmit;
mod api;
mod metrics;

use crate::transmit::start_transmission;
use crate::api::start_server;
use crate::metrics::init_metrics;
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let filters = Arc::new(Mutex::new(vec!["tcp port 502".to_string(), "udp port 161".to_string()]));

    let app_state = Arc::new(init_metrics());

    let (_tx, mut rx) = mpsc::channel::<Vec<u8>>(100);

    tokio::spawn(async move {
        start_server(Arc::clone(&app_state), filters.clone()).await;
    });

    tokio::spawn(async move {
        start_transmission("192.168.1.100", 443, &[0x45, 0x00, 0x32], true);
    });

    while let Some(packet) = rx.recv().await {
        println!("📡 Paquete procesado: {:?}", packet);
    }
}
