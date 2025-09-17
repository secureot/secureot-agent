# Módulo: `sniffer_span_forwarder`

## Descripción

El módulo `sniffer_span_forwarder` es un **agente de captura de red** diseñado para ser desplegado en ubicaciones remotas. Su propósito es capturar de forma pasiva el tráfico de una interfaz de red, guardarlo localmente en archivos PCAP y, opcionalmente, reenviar una copia a un servidor central para un análisis más profundo.

Este módulo es ideal para monitorizar la actividad en un **puerto SPAN** o *mirror port*, ya que opera de manera no intrusiva y puede enviar datos a una ubicación central sin necesidad de una conexión de red activa entre la máquina de análisis y el segmento monitorizado.

## Funcionalidad Clave

-   **Captura de Tráfico**: Utiliza la biblioteca `pcap` para capturar todo el tráfico de una interfaz de red específica.
-   **Reenvío de Paquetes**: Puede reenviar los paquetes capturados a un servidor remoto (como el módulo `sniffer_remote_pcap`) a través de UDP o TCP.
-   **Gestión de Archivos PCAP**: Guarda una copia local del tráfico en archivos PCAP con rotación horaria, categorizando los paquetes por IP de origen/destino.
-   **Filtro BPF**: Permite aplicar un filtro BPF (Berkeley Packet Filter) para capturar solo el tráfico relevante, optimizando el rendimiento.

## Configuración

El módulo se configura a través de un archivo `config.yaml`.

```yaml
capture:
  interface: "eth0"
  bpf_filter: "ip and not host 192.168.1.1"
routing_rules:
  - ip: "10.0.0.100"
    output_prefix: "robot_traffic"
default_output_prefix: "general_network_traffic"
output_directories:
  pcap_raw_output_path: "/var/log/pcaps"
forward:
  enable: true
  host: "192.168.1.50"
  port: 50000
  protocol: "udp"
```

## Uso
Para ejecutar el módulo, navega a la carpeta del proyecto y utiliza el siguiente comando:

```
./sniffer_span_forwarder /path/to/your/config.yaml
```
