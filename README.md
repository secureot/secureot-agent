

## 🔍 Sniffer pasivo por puerto SPAN
El agente puede operar en modo pasivo escuchando tráfico duplicado desde un puerto SPAN (Switch Port Analyzer). Esta modalidad le permite capturar paquetes en tiempo real sin interferir en la red, ideal para monitoreo continuo, auditorías y detección de anomalías en entornos OT donde la intervención directa no es posible.

### 📘 Ejemplos de configuración SPAN por modelo

| Marca / Modelo         | Método de Configuración SPAN                                                                                          | Observaciones / Limitaciones                                                                 |
|------------------------|-------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------|
| **Cisco 2960X**        | CLI: `monitor session 1 source interface Gi1/0/1`, `monitor session 1 destination Gi1/0/24`                            | Permite múltiples sesiones SPAN, pero sin soporte ERSPAN nativo                              |
| **Cisco SG300 / SF300**| GUI: *Administration > Diagnostics > Port and VLAN Mirroring*                                                          | Solo admite una sesión SPAN a la vez                                                         |
| **Aruba 2030M**        | CLI: `mirror-port 24`, `monitor interface 1 both` (modelos como Aruba 2930M)                                           | Usar sintaxis adecuada según la versión de ArubaOS                                           |
| **HP 1920G**           | GUI: *Network > Port Mirroring*                                                                                        | Interfaz web simple, sin acceso completo a CLI en muchos modelos                             |
| **H3C 5500 EI**        | CLI: `mirroring-group 1`, `mirroring-group 1 mirroring-port GigabitEthernet1/0/1 both`, `mirroring-group 1 monitor-port GigabitEthernet1/0/24` | Soporta grupos de monitoreo, configuración más detallada                                     |
| **3Com 5500 / 5500G**  | CLI: `monitor session 1 source interface ...`, `monitor session 1 destination interface ...`                           | Similar a H3C; firmware compartido puede tener diferencias sutiles en comandos               |
| **FortiSwitch 124F**   | CLI: `config switch mirror`, `edit "m1"`, `set dst port5`, `set src-ingress port3`, `set src-egress port4`, `set status active`, `end` | No admite mirroring entre VLANs sin configuración avanzada                                   |
| **FortiGate 60F / 30D**| CLI: `config system virtual-switch`, `edit lan`, `set span enable`, `set span-source-port`, `set span-dest-port`, `end`| Solo funciona en interfaces tipo hardware/software switch; no aplicable a interfaces físicas |


## 📡 Sniffer remoto por puerto TCP
El agente también puede actuar como receptor activo de tráfico, escuchando conexiones entrantes a través de un puerto configurable (por defecto 443). Esta modalidad permite recibir datos desde nodos remotos o arquitecturas de reenvío, integrándose como colector central en despliegues distribuidos o como nodo puente en capturas fuera de banda.
