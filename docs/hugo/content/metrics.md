# SecureOT Insight Agent - Configuración (`config.rs`)

## 📌 Introducción
El módulo `config.rs` permite que el agente cargue y gestione la configuración desde el archivo `agent.toml`, facilitando ajustes **sin necesidad de modificar el código fuente**.

✅ **Permite configuración dinámica sin recompilación.**  
✅ **Soporta múltiples parámetros para control remoto, seguridad y monitoreo.**  
✅ **Facilita cambios rápidos en filtros BPF y opciones de transmisión.**  

## 🚀 **1️⃣ Carga de Configuración desde `agent.toml`**
La función `read_config()` carga los parámetros del archivo TOML.

### 📌 **Ejemplo en Rust**
```rust
let config = read_config();
println!("Interfaz activa: {}", config.interface);
```

✅ **Lee la configuración al iniciar el agente.**  
✅ **Modificable sin tocar el código fuente.**  

## 📊 **2️⃣ Parámetros Disponibles en `agent.toml`**
Aquí están los parámetros que `config.rs` interpreta desde el archivo de configuración:

```toml
interface = "eth0"
host = "192.168.1.100"
port = 443
ssl_enabled = true
mode = "dual"
api_enabled = true
api_port = 8080
filters = ["tcp port 502", "udp port 161"]
cert_file = "config/server.crt"
key_file = "config/server.key"
metrics_enabled = true
metrics_port = 9090
log_level = "info"
```

| Parámetro       | Descripción |
|----------------|------------|
| `interface`    | Interfaz de captura de tráfico |
| `host`        | IP del servidor central |
| `port`        | Puerto de transmisión |
| `ssl_enabled` | Activar SSL/TLS en la comunicación |
| `mode`        | `"send"`, `"receive"` o `"dual"` según el rol del agente |
| `api_enabled` | Activar API REST para gestión remota |
| `api_port`    | Puerto de la API REST |
| `filters`     | Filtros BPF dinámicos para captura específica |
| `cert_file`   | Ruta del certificado SSL manual (si se usa) |
| `key_file`    | Ruta de la clave privada SSL |
| `metrics_enabled` | Habilitar métricas con Prometheus |
| `metrics_port` | Puerto para monitoreo en tiempo real |
| `log_level`   | Nivel de logging (`info`, `debug`, `error`) |

## 🔍 **3️⃣ Visualización de Configuración**
La función `display_config()` muestra los valores activos en la ejecución.

### 📌 **Ejemplo en Rust**
```rust
let config = read_config();
display_config(&config);
```

✅ **Útil para depuración y revisión rápida de configuración.**  
✅ **Permite validar si los parámetros se aplican correctamente.**  

## 🔐 **4️⃣ Seguridad en la Configuración**
Para evitar errores en los parámetros, se recomienda agregar **validación antes de aplicar valores**.

📌 **Ejemplo en Rust:**
```rust
fn validate_config(config: &Config) {
    assert!(["send", "receive", "dual"].contains(&config.mode.as_str()), "Modo inválido");
    assert!(config.port > 0, "El puerto debe ser mayor a 0");
}
```

✅ **Esto previene configuraciones incorrectas antes de ejecutarlas.**  

