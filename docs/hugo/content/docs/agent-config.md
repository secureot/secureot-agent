# SecureOT Insight Agent - Configuración (`agent.toml`)

Este archivo `agent.toml` define **todos los parámetros configurables** del SecureOT Insight Agent, permitiendo modificar opciones **sin necesidad de recompilar**.

## 📂 Estructura General

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

---

## 🌐 **Configuración de Red**
Define la interfaz de red utilizada para capturar y enviar tráfico.

```toml
interface = "eth0"     # Nombre de la interfaz de red
host = "192.168.1.100" # IP de la central OT
port = 443             # Puerto de transmisión
```

✅ **Modifica `interface` para capturar tráfico en otra NIC.**  
✅ **Cambia `host` y `port` para ajustar el destino del tráfico.**  

---

## ⚙️ **Modos de Operación**
Define si el agente funcionará como **emisor, receptor o ambos simultáneamente**.

```toml
mode = "dual"         # Opciones: "send", "receive", "dual"
```

| Modo      | Función |
|-----------|--------|
| `"send"`  | Captura tráfico y lo reenvía |
| `"receive"` | Recibe tráfico y lo almacena en PCAP |
| `"dual"` | Captura y recibe tráfico al mismo tiempo |

---

## 🔐 **Seguridad y Certificados**
Configura si la transmisión usará **SSL/TLS** y si los certificados serán manuales o efímeros.

```toml
ssl_enabled = true    # Habilitar SSL/TLS
cert_file = "config/server.crt"  # Certificado manual
key_file = "config/server.key"   # Clave privada SSL manual
```

✅ **Si `ssl_enabled` es `true`, la transmisión será cifrada.**  
✅ **Si no se especifican `cert_file` y `key_file`, se usará un certificado efímero.**  

---

## 🔗 **Configuración de API REST**
Permite activar la **API REST** para controlar filtros y monitorear estadísticas.

```toml
api_enabled = true    # Activar API REST
api_port = 8080       # Puerto de la API
```

✅ **Si `api_enabled` es `true`, el agente iniciará el servidor REST.**  

---

## 🛠️ **Filtros BPF Dinámicos**
Define los filtros de captura, ajustables en tiempo real.

```toml
filters = [
    "tcp port 502",  # Captura tráfico Modbus TCP
    "udp port 161",  # Captura tráfico SNMP
]
```

✅ **Compatible con API REST para ajustes sin reiniciar el agente.**  

---

## 📊 **Métricas y Monitoreo**
Permite activar el monitoreo con **Prometheus y Grafana**.

```toml
metrics_enabled = true  # Activar métricas en tiempo real
metrics_port = 9090     # Puerto donde se publican las métricas
```

✅ **Si `metrics_enabled` es `true`, se generarán estadísticas en Prometheus.**  

---

## 📝 **Nivel de Logging**
Configura el nivel de detalle en los registros del agente.

```toml
log_level = "info"   # Opciones: "info", "debug", "error"
```

| Nivel  | Descripción |
|--------|-------------|
| `"info"`  | Logs normales |
| `"debug"` | Logs detallados para depuración |
| `"error"` | Solo errores críticos |


