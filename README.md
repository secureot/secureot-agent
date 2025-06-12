# 📌 **SecureOT Insight Agent - README**
### 🛡️ **Descripción**
**SecureOT Insight Agent** es una plataforma modular para la captura, filtrado y retransmisión de tráfico OT. Se integra con **SecureOT Insight Central** para monitoreo en tiempo real.

---

## 🚀 **Características Principales**
- 🦀 **Implementado en Rust** para alta eficiencia  
- 🔍 **Filtros BPF dinámicos** para segmentación de tráfico  
- 🔐 **Seguridad con SSL/TLS** para transmisión segura  
- 📡 **API REST** para monitoreo y gestión  
- 🏗️ **Despliegue con Docker** para entornos escalables  
- 📊 **Exportación de métricas a Prometheus** para análisis en Grafana  

---

## 🔹 **1️⃣ Instalación**
Asegúrate de tener **Rust** y **Cargo** instalados antes de compilar el agente:

```bash
git clone https://github.com/tuusuario/secureot-agent.git
cd secureot-agent
cargo build --release
```
✅ **Esto generará los binarios optimizados para producción.**

---

## 🔹 **2️⃣ Configuración**
Puedes configurar el agente usando `config.toml` o **variables de entorno**.

### 📜 **Ejemplo de `config.toml`**
```toml
[network]
host = "192.168.1.100"
port = 443

[filters]
bpf_rules = ["tcp port 502", "udp port 161"]

[security]
use_ssl = true
ssl_cert_path = "/etc/ssl/certs/agent-cert.pem"
ssl_key_path = "/etc/ssl/private/agent-key.pem"
```
✅ **Define reglas de captura, seguridad y conectividad en un solo archivo.**

---

## 🔹 **3️⃣ Ejecución**
Para ejecutar el agente con configuración personalizada:

```bash
cargo run --release
```
✅ **Inicia el agente y expone la API REST.**

Si prefieres ejecutarlo en **Docker**, usa:

```bash
docker-compose up -d
```
✅ **Permite un despliegue portátil sin depender de Rust localmente.**

---

## 🔹 **4️⃣ API REST - Ejemplos de Uso**
### 📜 **Obtener estadísticas**
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

### 📜 **Aplicar filtro BPF**
```bash
curl -X POST http://localhost:8080/set_bpf \
     -H "Content-Type: application/json" \
     -d '{"filter": "tcp port 502"}'
```
📜 **Salida esperada:**
```
✅ Filtro BPF aplicado: tcp port 502
```

---

## 🔹 **5️⃣ Monitoreo de Métricas**
Si usas **Prometheus**, el agente expone métricas en `http://localhost:8080/metrics`.

```bash
curl -X GET http://localhost:8080/metrics
```
📜 **Ejemplo de salida Prometheus:**
```
# HELP packets_captured Total de paquetes capturados
# TYPE packets_captured counter
packets_captured 15023

# HELP packets_filtered Total de paquetes filtrados
# TYPE packets_filtered counter
packets_filtered 7289
```
