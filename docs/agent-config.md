# 📌 **SecureOT Insight Agent - Configuración**
### 📑 **Descripción**
Este archivo proporciona información sobre la configuración y opciones disponibles en **SecureOT Insight Agent**, incluyendo parámetros, ajustes recomendados y ejemplos de uso.

---

## 🔹 **1️⃣ Configuración General**
El agente permite una configuración modular mediante variables de entorno o un archivo `.env`.

### 📜 **Ejemplo de `agent-config.toml`**
```toml
[network]
host = "192.168.1.100"  # Dirección del servidor de transmisión
port = 443              # Puerto de conexión

[filters]
bpf_rules = ["tcp port 502", "udp port 161"]  # Filtros BPF para captura de tráfico

[security]
use_ssl = true          # Habilitar SSL/TLS
ssl_cert_path = "/etc/ssl/certs/agent-cert.pem"
ssl_key_path = "/etc/ssl/private/agent-key.pem"
```
✅ **Este archivo define la configuración del agente en formato TOML, fácil de modificar.**

---

## 🔹 **2️⃣ Variables de Entorno (Alternativa a `agent-config.toml`)**
Si prefieres usar variables de entorno en lugar de un archivo de configuración, usa `.env`:

```env
HOST=192.168.1.100
PORT=443
USE_SSL=true
BPF_FILTERS=tcp port 502,udp port 161
SSL_CERT_PATH=/etc/ssl/certs/agent-cert.pem
SSL_KEY_PATH=/etc/ssl/private/agent-key.pem
```
✅ **Alternativa rápida sin necesidad de editar `agent-config.toml`.**

---

## 🔹 **3️⃣ Ejemplos de Uso con `curl`**
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

### ✨ **Aplicar filtro BPF dinámico**
```bash
curl -X POST http://localhost:8080/set_bpf \
     -H "Content-Type: application/json" \
     -d '{"filter": "tcp port 502"}'
```
📜 **Respuesta en consola:**
```bash
✅ Filtro BPF aplicado: tcp port 502
```
