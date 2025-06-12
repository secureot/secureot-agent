# SecureOT Insight Agent - Módulo de Utilidades (`utils.rs`)

## 📌 Introducción
El módulo `utils.rs` contiene **funciones auxiliares** que optimizan el rendimiento y la gestión de datos dentro del SecureOT Insight Agent. Estas funciones mejoran la **eficiencia en transmisión, liberación de memoria y manejo concurrente**.

✅ **Optimización de memoria para tráfico OT.**  
✅ **Manejo de contadores en procesos concurrentes.**  
✅ **Conversión de tamaños de datos en formatos legibles (`KB`, `MB`).**  
✅ **Temporizador para medir tiempos de procesamiento y rendimiento.**  

## 🚀 **1️⃣ Temporizador para análisis de rendimiento**
Esta estructura `Timer` permite **medir el tiempo de ejecución** de procesos en el agente.

### 📌 **Código en Rust**
```rust
use std::time::{Duration, Instant};

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
```

✅ **Mide el tiempo que tarda en ejecutarse un proceso.**  
✅ **Útil para analizar la eficiencia de captura y transmisión de tráfico.**  

### 📌 **Ejemplo de uso**
```rust
let timer = Timer::new();
// Ejecutar alguna operación...
println!("Tiempo transcurrido: {:?}", timer.elapsed());
```

## 🧠 **2️⃣ Optimización de memoria**
La función `optimize_memory()` filtra bytes innecesarios en paquetes de red, **reduciendo el uso de memoria**.

### 📌 **Código en Rust**
```rust
pub fn optimize_memory(packet: Vec<u8>) -> Vec<u8> {
    packet.into_iter().filter(|&byte| byte != 0x00).collect()
}
```

✅ **Elimina bytes nulos (`0x00`) que no aportan información útil.**  
✅ **Optimiza el almacenamiento y transmisión de datos.**  

### 📌 **Ejemplo de uso**
```rust
let raw_packet = vec![0x00, 0x45, 0x00, 0x32, 0x00];
let optimized_packet = optimize_memory(raw_packet);
println!("{:?}", optimized_packet); // Salida: [0x45, 0x00, 0x32]
```

## 🔢 **3️⃣ Manejo seguro de contadores concurrentes**
La estructura `Counter` permite **incrementar valores en múltiples hilos de ejecución sin riesgo de colisiones**.

### 📌 **Código en Rust**
```rust
use std::sync::{Arc, Mutex};

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
```

✅ **Seguro en entornos de multiprocesamiento.**  
✅ **Evita condiciones de carrera entre múltiples hilos.**  

### 📌 **Ejemplo de uso**
```rust
let counter = Counter::new();
counter.increment();
println!("Valor del contador: {}", counter.get()); // Salida: 1
```

## 🔍 **4️⃣ Conversión de tamaño de datos**
La función `format_bytes()` convierte un número de bytes en un **formato legible para humanos** (`KB`, `MB`).

### 📌 **Código en Rust**
```rust
pub fn format_bytes(size: usize) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1_048_576 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else {
        format!("{:.2} MB", size as f64 / 1_048_576.0)
    }
}
```

✅ **Permite visualizar tamaños de transmisión de forma clara.**  
✅ **Ideal para logs y análisis de rendimiento.**  

### 📌 **Ejemplo de uso**
```rust
println!("{}", format_bytes(2048)); // Salida: "2.00 KB"
println!("{}", format_bytes(5000000)); // Salida: "4.77 MB"
```

