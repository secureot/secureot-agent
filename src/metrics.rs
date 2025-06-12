use prometheus::IntCounter;

pub struct AppState {
    pub packets_captured: IntCounter,
    pub packets_filtered: IntCounter,
}

pub fn init_metrics() -> AppState {
    AppState {
        packets_captured: IntCounter::new("packets_captured", "Total de paquetes capturados").unwrap(),
        packets_filtered: IntCounter::new("packets_filtered", "Total de paquetes filtrados").unwrap(),
    }
}
