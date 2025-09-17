# Módulo: `sniffer_remote_pcap`

## Descripción

El módulo `sniffer_remote_pcap` es un **servidor de recepción pasiva** diseñado para recolectar datos de red enviados por agentes remotos. Actúa como un punto de agregación central, escuchando en un puerto de red y guardando el tráfico recibido en archivos PCAP locales, con rotación horaria y categorización basada en la dirección IP de origen del remitente.

Este componente es esencial en una arquitectura distribuida de monitoreo de red, permitiendo la centralización del registro de tráfico desde múltiples ubicaciones, especialmente en entornos de **Tecnología Operativa (OT)** donde la visibilidad es crítica.

## Funcionalidad Clave

-   **Receptor de Tráfico**: Escucha en un puerto TCP o UDP para recibir datos.
-   **Enrutamiento por IP**: Clasifica los paquetes entrantes y los almacena en archivos PCAP distintos según la IP de origen del agente que los envía.
-   **Gestión de Archivos PCAP**: Genera archivos PCAP válidos y los rota automáticamente cada hora para una fácil gestión del almacenamiento.

## Configuración

El módulo se configura a través de un archivo `config.yaml`. Un ejemplo de configuración sería:

```yaml
port: 50000
protocol: "udp"
routing_rules:
  - ip: "192.168.10.10"
    output_prefix: "plc1_traffic"
  - ip: "192.168.20.20"
    output_prefix: "hmi_traffic"
default_output_prefix: "unclassified_traffic"
output_directories:
  pcap_raw_output_path: "/data/raw_pcaps/remote"
```

## Uso
Para ejecutar el módulo, navega a la carpeta del proyecto y utiliza el siguiente comando, especificando la ruta a tu archivo de configuración:

```
./sniffer_remote_pcap /path/to/your/config.yaml
```
