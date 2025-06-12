# 📌 **SecureOT Insight Agent - Métricas**
### 📑 **Descripción**
Este archivo documenta las métricas disponibles en **SecureOT Insight Agent**, su recopilación y cómo visualizar datos en tiempo real.

---

## 🔹 **1️⃣ Métricas Recopiladas**
SecureOT Insight Agent usa **Prometheus** para gestionar métricas de tráfico.

### 📜 **Lista de métricas disponibles**
| Métrica              | Descripción                             | Tipo        |
|----------------------|----------------------------------------|------------|
| `packets_captured`  | Total de paquetes capturados           | `IntCounter` |
| `packets_filtered`  | Total de paquetes filtrados            | `IntCounter` |
| `active_filters`    | Filtros BPF activos                    | `Gauge` |

✅ **Estas métricas permiten monitorear el tráfico en tiempo real.**

---

## 🔹 **2️⃣ Configuración de Métricas**
Puedes inicializar las métricas en `metrics.rs`:

```rust
use prometheus::IntCounter;

pub struct Metrics {
    pub packets_captured: IntCounter,
    pub packets_filtered: IntCounter,
}

pub fn init_metrics() -> Metrics {
    Metrics {
        packets_captured: IntCounter::new("packets_captured", "Total de paquetes capturados").unwrap(),
        packets_filtered: IntCounter::new("packets_filtered", "Total de paquetes filtrados").unwrap(),
    }
}
```
✅ **Este código registra las métricas para su monitoreo.**

---

## 🔹 **3️⃣ Exportar métricas a Prometheus**
Si usas **Prometheus**, define el endpoint de métricas:

```rust
use prometheus::Encoder;
use axum::{routing::get, Router};

async fn metrics_handler() -> String {
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();
    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

fn create_router() -> Router {
    Router::new().route("/metrics", get(metrics_handler))
}
```
✅ **Esto expone las métricas en `http://localhost:8080/metrics`.**

---

## 🔹 **4️⃣ Ejemplos de Uso con `curl`**
### ✨ **Obtener estadísticas del agente**
```bash
curl -X GET http://localhost:8080/stats
```
📜 **Ejemplo de respuesta:**
```json
{
    "packets_captured": 15023,
    "packets_filtered": 7289
}
```

### ✨ **Exportar métricas a Prometheus**
```bash
curl -X GET http://localhost:8080/metrics
```
📜 **Ejemplo de salida:**
```
# HELP packets_captured Total de paquetes capturados
# TYPE packets_captured counter
packets_captured 15023

# HELP packets_filtered Total de paquetes filtrados
# TYPE packets_filtered counter
packets_filtered 7289
```
