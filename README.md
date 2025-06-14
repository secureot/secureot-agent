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
- 🔥 **Solución a Errores Comunes**

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
---

## 🔹 **5️⃣ Solución a Errores Comunes**
### 📌 **Error: OpenSSL no encontrado**
📜 **Mensaje de error al compilar:**
```
cargo:warning=Could not find directory of OpenSSL installation, and this `-sys` crate cannot proceed without this knowledge.
```

📌 **Causa:**  
Rust no encuentra **OpenSSL** en el sistema, lo que impide la compilación de dependencias que requieren cifrado.  

📌 **Solución:**  
Ejecuta los siguientes comandos para definir correctamente las variables de entorno y asegurarte de que OpenSSL esté instalado:  

```bash
export OPENSSL_DIR=/usr/lib/ssl
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
export OPENSSL_INCLUDE_DIR=/usr/include/openssl
```
En el caso de usar una Raspberry Pi
```bash
export OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu/
```

✅ **Esto le indica a Cargo dónde encontrar OpenSSL.**  

Si el error persiste, instala las dependencias necesarias:
```bash
sudo apt update && sudo apt install -y pkg-config libssl-dev
```

📜 **Verifica que OpenSSL está correctamente instalado:**
```bash
openssl version
```
✅ **Debe mostrar una versión activa, como `OpenSSL 3.0.2 15 Mar 2022`.**  

🔥 **Después de estos ajustes, recompila tu proyecto con:**
```bash
cargo clean
cargo build --release
```
Esto asegurará que SecureOT Insight Agent se compile sin errores de OpenSSL. 🚀  

