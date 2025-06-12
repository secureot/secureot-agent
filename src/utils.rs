use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

// 🔹 Estructura para medir tiempos de ejecución
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer { start: Instant::now() }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

// 🔹 Función para liberar memoria de paquetes innecesarios
pub fn optimize_memory(packet: Vec<u8>) -> Vec<u8> {
    packet.into_iter().filter(|&byte| byte != 0x00).collect()
}

// 🔹 Función para convertir bytes en formato legible
pub fn format_bytes(size: usize) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1_048_576 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else {
        format!("{:.2} MB", size as f64 / 1_048_576.0)
    }
}

// 🔹 Manejo seguro de contadores concurrentes
pub struct Counter {
    value: Arc<Mutex<u64>>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: Arc::new(Mutex::new(0)),
        }
    }

    pub fn increment(&self) {
        let mut val = self.value.lock().unwrap();
        *val += 1;
    }

    pub fn get(&self) -> u64 {
        *self.value.lock().unwrap()
    }
}
