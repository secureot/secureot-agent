# 📌 **SecureOT Insight Agent - API REST**
### 📑 **Descripción**
La API permite gestionar filtros BPF dinámicos y visualizar estadísticas de tráfico capturado y filtrado en tiempo real.

---

## 🔹 **1️⃣ `GET /stats` → Obtener estadísticas de tráfico**
### ✨ **Descripción**
Devuelve el número total de paquetes capturados y filtrados.

### 📌 **Ejemplo de solicitud `curl`**
```bash
curl -X GET http://localhost:8080/stats
```

### 📜 **Ejemplo de respuesta JSON**
```json
{
    "packets_captured": 15023,
    "packets_filtered": 7289
}
```

✅ **Este endpoint ayuda a monitorear el estado del agente en tiempo real.**  

---

## 🔹 **2️⃣ `POST /set_bpf` → Aplicar filtro BPF dinámico**
### ✨ **Descripción**
Agrega un nuevo filtro BPF para capturar tráfico específico.

### 📌 **Ejemplo de solicitud `curl`**
```bash
curl -X POST http://localhost:8080/set_bpf \
     -H "Content-Type: application/json" \
     -d '{"filter": "tcp port 502"}'
```

### 📜 **Ejemplo de respuesta (salida en consola)**
```bash
✅ Filtro BPF aplicado: tcp port 502
```

✅ **Permite ajustar dinámicamente la captura de tráfico sin reiniciar el agente.**  

---

## 📌 **Configuración y Ejecución**
1️⃣ **Asegúrate de que SecureOT Insight Agent está en ejecución**  
```bash
cargo run --release
```

2️⃣ **Verifica que la API REST está activa**  
```bash
curl -X GET http://localhost:8080/stats
```

3️⃣ **Modifica los filtros según necesidad**  
Puedes agregar varios filtros BPF enviando múltiples solicitudes `POST /set_bpf`.

