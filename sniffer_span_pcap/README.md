# Módulo: `sniffer_span_pcap`

## Descripción

El módulo `sniffer_span_pcap` es la versión más sencilla del sniffer pasivo. Su único propósito es **capturar el tráfico de una interfaz de red local** y almacenarlo en archivos PCAP en el disco, sin ninguna funcionalidad de reenvío.

Es la solución ideal para una monitorización local no intrusiva, como la depuración de una máquina o la auditoría de un segmento de red específico, donde no se requiere la centralización de los datos.

## Funcionalidad Clave

-   **Captura de Tráfico**: Utiliza la biblioteca `pcap` para "escuchar" y capturar todo el tráfico que fluye a través de una interfaz de red.
-   **Gestión de Archivos PCAP**: Almacena el tráfico en archivos PCAP válidos con rotación horaria.
-   **Enrutamiento por IP**: Categoriza los paquetes capturados en diferentes archivos PCAP según las reglas de IP configuradas.
-   **Filtro BPF**: Permite aplicar un filtro para capturar solo los paquetes que cumplen con ciertos criterios.

## Configuración

El módulo se configura a través de un archivo `config.yaml`. Un ejemplo de configuración sería:

```yaml
capture:
  interface: "enp0s3"
  bpf_filter: "port 502"
routing_rules:
  - ip: "192.168.1.1"
    output_prefix: "plc1_traffic"
default_output_prefix: "general_traffic"
output_directories:
  pcap_raw_output_path: "/var/log/my_pcaps"
```

## Uso
Para ejecutar el módulo, navega a la carpeta del proyecto y utiliza el siguiente comando:

```
./sniffer_span_pcap /path/to/your/config.yaml
```
