use prometheus::{IntCounter, Gauge};

pub struct Metrics {
    pub packets_processed: IntCounter,
    pub cpu_usage: Gauge,
    pub memory_usage: Gauge,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            packets_processed: IntCounter::new("packets_processed", "Cantidad de paquetes procesados").unwrap(),
            cpu_usage: Gauge::new("cpu_usage", "Uso de CPU").unwrap(),
            memory_usage: Gauge::new("memory_usage", "Uso de memoria").unwrap(),
        }
    }
}

pub fn init_metrics() -> Metrics {
    Metrics::new()
}
