use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use pcap;


// --- Estructuras de Configuración ---
/// Define la estructura de una regla de enrutamiento IP.
#[derive(Debug, Deserialize, Serialize, Clone)]
struct RoutingRule {
    ip: String,
    output_prefix: String,
}

/// Define la configuración de las opciones de captura de tráfico.
#[derive(Debug, Deserialize, Serialize)]
struct CaptureConfig {
    interface: String, // Interfaz de red a monitorear (ej. "eth0")
    bpf_filter: Option<String>, // Opcional: Filtro BPF para la captura
}

/// Define la configuración de los directorios de salida.
#[derive(Debug, Deserialize, Serialize)]
struct OutputDirectoriesConfig {
    pcap_raw_output_path: String, // Directorio donde se escribirán los archivos PCAP crudos
}

/// Define la estructura de la configuración principal del capturador.
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    capture: CaptureConfig, // <--- ¡CAMBIO AQUÍ! Ahora anidamos CaptureConfig
    routing_rules: Vec<RoutingRule>,
    default_output_prefix: String,
    output_directories: OutputDirectoriesConfig, // <--- ¡CAMBIO AQUÍ! Ahora anidamos OutputDirectoriesConfig
}

// --- Gestión de Archivos PCAP ---
/// Estructura para gestionar un archivo PCAP con rotación horaria.
struct ManagedPcapFile {
    file: File,
    current_hour: u32,
    base_output_prefix: String,
}

impl ManagedPcapFile {
    /// Crea una nueva instancia de ManagedPcapFile, abriendo un nuevo archivo PCAP con el timestamp actual.
    fn new(base_prefix: String, initial_timestamp: DateTime<Local>, output_path: &str) -> io::Result<Self> {
        let current_hour = initial_timestamp.hour();
        let filename = format!("{}/{}", output_path, Self::generate_filename(&base_prefix, initial_timestamp)); // <--- CAMBIO AQUÍ: Añadir output_path
        let mut file = File::create(&filename)?;
        write_pcap_header(&mut file)?;
        println!("[PCAP Manager] Nuevo archivo PCAP creado: {}", filename);
        Ok(ManagedPcapFile {
            file,
            current_hour,
            base_output_prefix: base_prefix,
        })
    }

    /// Genera el nombre de archivo PCAP con el formato de fragmentación horaria.
    /// Ejemplo: "plc1_traffic_20250616_00.pcap"
    fn generate_filename(base_prefix: &str, timestamp: DateTime<Local>) -> String {
        timestamp.format(&format!("{}_%Y%m%d_%H.pcap", base_prefix)).to_string()
    }

    /// Devuelve el descriptor de archivo actual. Si la hora ha cambiado, rota el archivo.
    fn get_current_file(&mut self, current_timestamp: DateTime<Local>, output_path: &str) -> io::Result<&mut File> { // <--- CAMBIO AQUÍ: Añadir output_path
        if current_timestamp.hour() != self.current_hour {
            println!("[PCAP Manager] Rotando archivo PCAP para {}. Nueva hora detectada.", self.base_output_prefix);

            let new_filename = format!("{}/{}", output_path, Self::generate_filename(&self.base_output_prefix, current_timestamp)); // <--- CAMBIO AQUÍ: Añadir output_path

            let mut new_file = File::create(&new_filename)?;
            write_pcap_header(&mut new_file)?;

            self.file = new_file;
            self.current_hour = current_timestamp.hour();
            println!("[PCAP Manager] Nuevo archivo PCAP abierto: {}", new_filename);
        }
        Ok(&mut self.file)
    }
}

// --- Funciones de Escritura PCAP ---
/// Escribe el encabezado global de un archivo PCAP.
fn write_pcap_header(mut file: &File) -> io::Result<()> {
    let header: [u8; 24] = [
        0xd4, 0xc3, 0xb2, 0xa1,
        0x02, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
    ];
    file.write_all(&header)?;
    Ok(())
}

/// Escribe un solo paquete al archivo PCAP, con su encabezado de paquete PCAP.
fn write_pcap_packet(mut file: &File, data: &[u8]) -> io::Result<()> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds = now.as_secs() as u32;
    let micros = now.subsec_micros();
    let len = data.len() as u32;
    let original_len = len;

    let mut header = Vec::new();
    header.extend(&seconds.to_le_bytes());
    header.extend(&micros.to_le_bytes());
    header.extend(&len.to_le_bytes());
    header.extend(&original_len.to_le_bytes());

    file.write_all(&header)?;
    file.write_all(data)?;
    file.flush()?;
    Ok(())
}

// --- Parsing de Encabezados IP (Rudimentario) ---
/// Intenta parsear las direcciones IP de origen y destino de un paquete Ethernet crudo.
/// Este es un parser básico solo para IPv4 después de un encabezado Ethernet estándar (14 bytes).
/// Para un parsing robusto en producción, se recomienda una librería como `pnet` o `etherparse`.
fn parse_ip_addresses(packet_data: &[u8]) -> Option<(IpAddr, IpAddr)> {
    if packet_data.len() < 14 + 20 {
        return None;
    }

    let ether_type = u16::from_be_bytes([packet_data[12], packet_data[13]]);
    if ether_type != 0x0800 {
        return None;
    }

    let ip_header_start = 14;
    let ip_version_ihl = packet_data[ip_header_start];
    let ip_header_len = (ip_version_ihl & 0x0F) * 4;

    if packet_data.len() < ip_header_start + ip_header_len as usize {
        return None;
    }

    let src_ip_bytes = [
        packet_data[ip_header_start + 12],
        packet_data[ip_header_start + 13],
        packet_data[ip_header_start + 14],
        packet_data[ip_header_start + 15],
    ];
    let src_ip = IpAddr::from(src_ip_bytes);

    let dst_ip_bytes = [
        packet_data[ip_header_start + 16],
        packet_data[ip_header_start + 17],
        packet_data[ip_header_start + 18],
        packet_data[ip_header_start + 19],
    ];
    let dst_ip = IpAddr::from(dst_ip_bytes);

    Some((src_ip, dst_ip))
}

// --- Lógica Principal de la Aplicación ---
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Uso: {} <ruta_a_config.yaml>", args[0]);
        std::process::exit(1);
    }

    let config_path = &args[1];
    let config_str = std::fs::read_to_string(config_path)
        .expect(&format!("No se pudo leer el archivo de configuración: {}", config_path));

    let config: Config = serde_yaml::from_str(&config_str)
        .expect("Error al parsear el archivo de configuración YAML. Verifique el formato y las dependencias de 'serde_yaml'.");

    println!("Configuración cargada: {:?}", config);
    println!("[*] Iniciando captura en la interfaz: {}", config.capture.interface); // <--- CAMBIO AQUÍ: Acceso a la interfaz

    let mut cap = pcap::Capture::from_device(config.capture.interface.as_str()) // <--- CAMBIO AQUÍ: Acceso a la interfaz
        .expect("Fallo al abrir la interfaz. ¿Permisos (sudo), interfaz correcta, o paquete 'libpcap-dev' instalado?")
        .promisc(true)
        .snaplen(65535)
        .timeout(100)
        .open()
        .expect("Fallo al abrir la captura. La interfaz podría no existir o no tener los permisos suficientes.");

    if let Some(filter) = &config.capture.bpf_filter { // <--- CAMBIO AQUÍ: Acceso al filtro BPF
        cap.filter(filter, true)
            .expect(&format!("Fallo al aplicar filtro BPF: '{}'. Verifique la sintaxis del filtro.", filter));
        println!("[*] Aplicado filtro BPF: '{}'", filter);
    } else {
        println!("[*] No se aplicó filtro BPF. Se capturará todo el tráfico de la interfaz.");
    }


    let mut pcap_file_managers: HashMap<String, ManagedPcapFile> = HashMap::new();

    let mut parsed_routing_rules: Vec<(IpAddr, String)> = Vec::new();
    for rule in &config.routing_rules {
        let ip_addr: IpAddr = rule.ip.parse()
            .expect(&format!("Dirección IP inválida en la configuración: {}", rule.ip));
        parsed_routing_rules.push((ip_addr, rule.output_prefix.clone()));
    }

    // Obtener la ruta de salida de los PCAP crudos
    let pcap_output_path = &config.output_directories.pcap_raw_output_path;

    loop {
        let now = Local::now();
        match cap.next() {
            Ok(packet) => {
                println!("[PCAP] Capturado {} bytes.", packet.data.len());

                if let Some((src_ip, dst_ip)) = parse_ip_addresses(packet.data) {
                    let mut output_prefix = config.default_output_prefix.clone();

                    for (rule_ip, prefix) in &parsed_routing_rules {
                        if src_ip == *rule_ip || dst_ip == *rule_ip {
                            output_prefix = prefix.clone();
                            break;
                        }
                    }

                    let pcap_file_manager = pcap_file_managers
                        .entry(output_prefix.clone())
                        .or_insert_with(|| {
                            ManagedPcapFile::new(output_prefix.clone(), now, pcap_output_path) // <--- CAMBIO AQUÍ: Pasar output_path
                                .expect("Fallo al crear un nuevo archivo PCAP gestionado para enrutamiento")
                        });

                    if let Err(e) = write_pcap_packet(pcap_file_manager.get_current_file(now, pcap_output_path).expect("Fallo al obtener el archivo PCAP actual para escritura"), packet.data) { // <--- CAMBIO AQUÍ: Pasar output_path
                        eprintln!("Error al guardar paquete PCAP en {}: {}", output_prefix, e);
                    }
                } else {
                    println!("[PCAP] Paquete no IP o malformado, ignorando para enrutamiento específico.");
                }
            }
            Err(e) => {
                if e.to_string().contains("timeout") {
                    // El timeout ocurrió, no hay paquetes nuevos en el último periodo. El bucle continúa.
                } else {
                    eprintln!("Error de captura PCAP: {}", e);
                }
            }
        }
    }
}
