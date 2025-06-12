# SecureOT Insight Agent - API REST (`api.rs`)

## 📌 Introducción
Este módulo permite la **gestión remota del agente**, incluyendo el control de filtros BPF y la consulta de métricas de tráfico en tiempo real mediante **una API REST en Rust con `axum` y `prometheus`**.

La API se activa **solo si se configura en `agent.toml`** o si se ejecuta con `--api`.

---

## 🚀 **1️⃣ Activación de la API REST**
Puedes iniciar el servidor REST de forma manual con:

```bash
./secureot-agent --api
```

También puedes activarlo en la configuración:

```toml
api_enabled = true
api_port = 8080
```

✅ **La API se ejecutará en el puerto definido (`8080` por defecto).**  

---

## 📊 **2️⃣ Endpoint `/stats` → Consultar métricas del agente**
Este endpoint devuelve estadísticas clave sobre el tráfico procesado.

### 📌 **Método:** `GET`
### 📌 **URL:** `/stats`

✅ **Ejemplo de consulta:**
```bash
curl -X GET "http://192.168.1.100:8080/stats"
```

### 📌 **Ejemplo de respuesta JSON**
```json
{
    "packets_captured": 12500,
    "packets_filtered": 3800,
    "bytes_transmitted": 357680
}
```

📌 **Métricas disponibles:**  
| Campo               | Descripción |
|---------------------|------------|
| `packets_captured` | Total de paquetes capturados por el agente |
| `packets_filtered` | Cantidad de paquetes descartados por filtros BPF |
| `bytes_transmitted` | Cantidad de datos transmitidos a la central OT |

---

## 🔗 **3️⃣ Endpoint `/set_bpf` → Aplicar filtros BPF dinámicamente**
Este endpoint permite **modificar filtros BPF en tiempo real** sin reiniciar el agente.

### 📌 **Método:** `POST`
### 📌 **URL:** `/set_bpf`
### 📌 **Formato del JSON de entrada:**
```json
{
    "filter": "tcp port 502"
}
```

✅ **Ejemplo de consulta:**
```bash
curl -X POST "http://192.168.1.100:8080/set_bpf" -d '{"filter": "udp port 161"}' -H "Content-Type: application/json"
```

📌 **Esto aplicará el filtro `udp port 161` de forma dinámica.**  

---

## 🔐 **4️⃣ Seguridad en la API**
Para evitar accesos no autorizados, se recomienda integrar **autenticación con claves API o JWT**.

📌 **Ejemplo de configuración de seguridad en `agent.toml`:**
```toml
api_auth_enabled = true
api_secret_key = "mi_clave_segura_123"
```

✅ **Si `api_auth_enabled` es `true`, se requerirá una clave en cada solicitud.**  
✅ **La clave `api_secret_key` se usará para validar autenticación.**  

---

## 🎯 **Conclusión**
La API REST del SecureOT Insight Agent proporciona **gestión remota avanzada**, permitiendo ajustes dinámicos sin detener la captura de tráfico.

✅ **Módulo activable solo cuando es necesario (`--api`).**  
✅ **Endpoints optimizados para monitoreo y control en tiempo real.**  
