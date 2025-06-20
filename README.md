

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

## Sniffer pasivo por puerto SPAN y Reenvío

Este agente permite capturar tráfico en crudo desde una interfaz de red (por ejemplo, un puerto SPAN) y reenviarlo en tiempo real a un colector central remoto mediante socket TCP o UDP, según configuración. Además, segmenta y almacena localmente el tráfico en archivos PCAP rotativos por hora y por dirección IP origen, lo cual permite disponer de un respaldo forense y operativo robusto en todo momento. La configuración es completamente dinámica vía YAML, soportando filtros BPF, reglas de enrutamiento por IP y parámetros de reenvío. Este componente está diseñado para desplegarse en entornos OT como puente de captura fuera de banda o colector distribuido.


### 📡 Sniffer remoto con recepción por puerto TCP/UDP
El agente también puede actuar como receptor activo de tráfico, escuchando conexiones entrantes a través de un puerto configurable. Esta modalidad permite recibir datos desde nodos remotos o arquitecturas de reenvío, integrándose como colector central en despliegues distribuido. Además, implementa un sistema de rotación horaria de archivos PCAP y definición por IP de origen, permitiendo segmentar el tráfico recibido en archivos independientes para cada fuente. Compatible con LINKTYPE_RAW, puede registrar payloads sin encabezados Ethernet, facilitando integraciones en entornos OT que requieren flexibilidad y separación lógica del tráfico.
