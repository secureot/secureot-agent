# SecureOT Insight Agent 🚀

## 📌 Introducción
SecureOT Insight Agent es una solución modular para captura, filtrado y retransmisión de tráfico OT, diseñada para ofrecer máxima eficiencia, seguridad y escalabilidad.

✅ **Soporte para filtros BPF dinámicos.**  
✅ **Gestión remota mediante API REST.**  
✅ **Transmisión segura con SSL/TLS.**  
✅ **Monitoreo avanzado con Prometheus y Grafana.**  

## 🔹 **1️⃣ Instalación del agente**
El script `install.sh` **automatiza la instalación** del SecureOT Insight Agent, incluyendo la configuración de dependencias y compilación.

### 📌 **Instalación automática**
```bash
chmod +x scripts/install.sh
./scripts/install.sh
```

### 📌 **Qué hace el script**
✅ **Actualiza paquetes del sistema.**  
✅ **Instala Rust y dependencias necesarias.**  
✅ **Compila el proyecto en modo `release`.**  
✅ **Configura permisos y estructura de archivos.**  

Una vez instalado, puedes ejecutar el agente con:
```bash
./target/release/secureot-agent --dual
```
Esto iniciará **captura y transmisión simultáneas**.

## 🔹 **2️⃣ Rotación de logs**
El script `rotate_logs.sh` **gestiona automáticamente la rotación de archivos de registro**, asegurando que los logs se organicen por fecha y se mantenga un historial limpio.

### 📌 **Ejecución manual**
```bash
chmod +x scripts/rotate_logs.sh
./scripts/rotate_logs.sh
```

✅ **Organiza logs por fecha y hora.**  
✅ **Elimina registros antiguos cuando superan el límite.**  
✅ **Optimiza el almacenamiento evitando archivos innecesarios.**  

Los registros se almacenan en `secureot-agent/logs/` con nombres como:
```
secureot-agent-2025-06-12_14-30-00.log
```

## 🔹 **3️⃣ Levantar la documentación con Hugo**
SecureOT Insight Agent usa **Hugo** para gestionar la documentación del proyecto.

### 📌 **Pasos para iniciar Hugo**
1️⃣ **Instalar Hugo** si no está presente:
```bash
sudo apt install hugo
```

2️⃣ **Ejecutar Hugo en modo servidor:**
```bash
hugo server --watch
```

3️⃣ **Acceder a la documentación en el navegador:**
```
http://localhost:1313/docs/
```

📌 **La documentación está en `content/docs/` y se renderiza dinámicamente.**  

✅ **Formato Markdown compatible con GitHub y Hugo.**  
✅ **Integrable con GitHub Pages para documentación pública.**  

// Actualizacion de codigo 2025-06-04
// Actualizacion de codigo 2025-05-29
// Actualizacion de codigo 2025-05-16
// Actualizacion de codigo 2025-05-30
// Actualizacion de codigo 2025-05-27
// Actualizacion de codigo 2025-05-25
// Actualizacion de codigo 2025-05-20
