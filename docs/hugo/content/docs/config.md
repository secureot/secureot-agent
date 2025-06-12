# SecureOT Insight Agent - Configuración (`config.rs`)

## 📌 Introducción
El módulo `config.rs` se encarga de **gestionar la configuración del agente** mediante un archivo TOML (`agent.toml`). Esto permite modificar parámetros **sin necesidad de recompilar**, facilitando ajustes dinámicos.

---

## 🚀 **1️⃣ Lectura de configuración desde `agent.toml`**
La función `read_config()` carga todos los parámetros definidos en el archivo de configuración.

### 📌 **Ejemplo de uso en Rust**
```rust
let config = read_config();
println!("Interfaz activa: {}", config.interface);
```

✅ **Lee la configuración al iniciar el agente.**  
✅ **Permite modificar opciones sin tocar el código fuente.**  

---

## 🔍 **2️⃣ Parámetros disponibles**
`config.rs` interpreta la configuración desde el archivo TOML con la siguiente estructura:

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
| `interface`    | Interfaz de red utilizada para captura |
| `host`        | IP del servidor central |
| `port`        | Puerto de transmisión de tráfico |
| `ssl_enabled` | Habilitar transmisión segura con SSL/TLS |
| `mode`        | `"send"`, `"receive"` o `"dual"` según el rol del agente |
| `api_enabled` | Activar API REST para configuración remota |
| `api_port`    | Puerto de la API REST |
| `filters`     | Filtros BPF para captura de tráfico específico |
| `cert_file`   | Ruta al certificado SSL manual (si se usa) |
| `key_file`    | Ruta a la clave privada SSL |
| `metrics_enabled` | Habilitar monitoreo con Prometheus |
| `metrics_port` | Puerto para estadísticas en tiempo real |
| `log_level`   | Nivel de logging (`info`, `debug`, `error`) |

---

## 🛠️ **3️⃣ Visualización de configuración**
La función `display_config()` muestra los valores activos en tiempo de ejecución.

### 📌 **Ejemplo de uso**
```rust
let config = read_config();
display_config(&config);
```

### 📌 **Salida esperada**
```
🔹 Configuración del Agente:
  - Interfaz de captura: eth0
  - Destino: 192.168.1.100:443
  - SSL habilitado: true
  - Modo de operación: dual
  - API REST habilitada: true en puerto 8080
  - Filtros BPF activos: ["tcp port 502", "udp port 161"]
  - Certificado SSL: config/server.crt
  - Clave privada SSL: config/server.key
  - Métricas activadas: true en puerto 9090
  - Nivel de logs: info
```

✅ **Útil para depuración y revisión rápida de configuración.**  

