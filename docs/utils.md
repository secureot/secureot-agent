# 📌 **SecureOT Insight Agent - Utilidades**
### 📑 **Descripción**
Este archivo documenta las funciones auxiliares y herramientas de **SecureOT Insight Agent** para facilitar la manipulación de datos y mejorar el rendimiento.

---

## 🔹 **1️⃣ Funciones Utilitarias**
### 📜 **Lista de funciones disponibles**
| Función           | Descripción                             | Uso |
|------------------|----------------------------------------|-----|
| `parse_packet`  | Convierte un paquete bruto en estructura legible | `parse_packet(&[0x45, 0x00, 0x32])` |
| `hex_to_bytes`  | Transforma una cadena hexadecimal en un `Vec<u8>` | `hex_to_bytes("450032")` |
| `validate_filter` | Verifica la sintaxis de un filtro BPF antes de aplicarlo | `validate_filter("tcp port 502")` |

✅ **Estas funciones facilitan el manejo de datos dentro del agente.**

---

## 🔹 **2️⃣ Implementación de Funciones**
### ✨ **`parse_packet()` → Convertir paquetes en datos legibles**
```rust
pub fn parse_packet(data: &[u8]) -> String {
    format!("Packet: {:?}", data)
}
```
📜 **Ejemplo de uso:**
```rust
let parsed = parse_packet(&[0x45, 0x00, 0x32]);
println!("{}", parsed);
```
📜 **Salida esperada:**
```
Packet: [69, 0, 50]
```
✅ **Convierte paquetes a una estructura fácil de interpretar.**

---

### ✨ **`hex_to_bytes()` → Convertir hexadecimal a `Vec<u8>`**
```rust
pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| u8::from_str_radix(&chunk.iter().collect::<String>(), 16).unwrap_or(0))
        .collect()
}
```
📜 **Ejemplo de uso:**
```rust
let bytes = hex_to_bytes("450032");
println!("{:?}", bytes);
```
📜 **Salida esperada:**
```
[69, 0, 50]
```
✅ **Útil para convertir datos hexadecimales en formato binario.**

---

### ✨ **`validate_filter()` → Validar sintaxis BPF**
```rust
pub fn validate_filter(filter: &str) -> bool {
    filter.contains("port") || filter.contains("ip")  // Validación básica
}
```
📜 **Ejemplo de uso:**
```rust
let is_valid = validate_filter("tcp port 502");
println!("¿Filtro válido?: {}", is_valid);
```
📜 **Salida esperada:**
```
¿Filtro válido?: true
```
✅ **Evita errores al procesar filtros antes de aplicarlos.**

---

## 🔹 **3️⃣ Ejemplos de Uso en `curl`**
✅ **Verificación de datos procesados por el agente**
```bash
curl -X GET http://localhost:8080/stats
```
✅ **Aplicar filtros BPF dinámicamente**
```bash
curl -X POST http://localhost:8080/set_bpf \
     -H "Content-Type: application/json" \
     -d '{"filter": "tcp port 502"}'
```
✅ **Transformar paquetes capturados**
```rust
let parsed = parse_packet(&[0x45, 0x00, 0x32]);
println!("{}", parsed);
```
