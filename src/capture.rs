use pnet::datalink::{self, Channel::Ethernet};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

async fn capture_with_bpf(interface: &str, filters: Arc<Mutex<Vec<String>>>, sender: mpsc::Sender<Vec<u8>>) {
    let interface = datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == *interface)
        .expect("No se encontró la interfaz");

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        _ => panic!("Error al abrir la interfaz"),
    };

    while let Ok(packet) = rx.next() {
        let active_filters = filters.lock().unwrap();
        if active_filters.iter().any(|f| apply_bpf_filter(packet, f)) {
            sender.send(packet.to_vec()).await.expect("Error al enviar datos");
        }
    }
}
