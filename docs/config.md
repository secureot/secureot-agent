# 📌 **SecureOT Insight Agent - Configuración**
### 📑 **Descripción**
Este archivo documenta las configuraciones esenciales para **SecureOT Insight Agent**, incluyendo ajustes de seguridad, red y captura de tráfico.

---

## 🔹 **1️⃣ Configuración General**
Puedes configurar el agente mediante **variables de entorno**, un archivo `.env` o `config.toml`.

### 📜 **Ejemplo de `config.toml`**
```toml
[network]
host = "192.168.1.100"  # Dirección del servidor
port = 443              # Puerto de transmisión

[filters]
bpf_rules = ["tcp port 502", "udp port 161"]  # Filtros BPF para captura de tráfico

[security]
use_ssl = true          # Habilitar SSL/TLS
ssl_cert_path = "/etc/ssl/certs/agent-cert.pem"
ssl_key_path = "/etc/ssl/private/agent-key.pem"
```

✅ **Este archivo define la configuración del agente en formato TOML, fácil de modificar.**

---

## 🔹 **2️⃣ Variables de Entorno (`.env`)**
Puedes definir la configuración en un archivo `.env` en lugar de `config.toml`.

```env
HOST=192.168.1.100
PORT=443
USE_SSL=true
BPF_FILTERS=tcp port 502,udp port 161
SSL_CERT_PATH=/etc/ssl/certs/agent-cert.pem
SSL_KEY_PATH=/etc/ssl/private/agent-key.pem
```

✅ **Alternativa rápida sin necesidad de modificar `config.toml`.**  

---

## 🔹 **3️⃣ Configuración en Docker**
Si despliegas el agente en **Docker**, asegúrate de incluir las variables de entorno en `docker-compose.yml`:

```yaml
version: "3.8"
services:
  secureot-agent:
    image: secureot-agent:latest
    environment:
      - HOST=192.168.1.100
      - PORT=443
      - USE_SSL=true
      - BPF_FILTERS=tcp port 502,udp port 161
      - SSL_CERT_PATH=/etc/ssl/certs/agent-cert.pem
      - SSL_KEY_PATH=/etc/ssl/private/agent-key.pem
    ports:
      - "8080:8080"
```

✅ **Esto garantiza que el agente reciba correctamente las configuraciones en entornos Docker.**

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
