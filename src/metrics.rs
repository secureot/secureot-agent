use prometheus::{IntCounter, Opts, Registry};
use std::sync::{Arc, Mutex};

pub struct Metrics {
    pub packets_captured: IntCounter,
    pub packets_filtered: IntCounter,
    pub bytes_transmitted: IntCounter,
}

pub fn init_metrics() -> Arc<Metrics> {
    let registry = Registry::new();

    let packets_captured = IntCounter::new("packets_captured", "Total de paquetes capturados").unwrap();
    let packets_filtered = IntCounter::new("packets_filtered", "Total de paquetes filtrados por BPF").unwrap();
    let bytes_transmitted = IntCounter::new("bytes_transmitted", "Total de datos transmitidos en bytes").unwrap();

    registry.register(Box::new(packets_captured.clone())).unwrap();
    registry.register(Box::new(packets_filtered.clone())).unwrap();
    registry.register(Box::new(bytes_transmitted.clone())).unwrap();

    Arc::new(Metrics {
        packets_captured,
        packets_filtered,
        bytes_transmitted,
    })
}

pub fn increment_captured(metrics: Arc<Metrics>) {
    metrics.packets_captured.inc();
}

pub fn increment_filtered(metrics: Arc<Metrics>) {
    metrics.packets_filtered.inc();
}

pub fn increment_transmitted(metrics: Arc<Metrics>, bytes: usize) {
    metrics.bytes_transmitted.inc_by(bytes as u64);
}
