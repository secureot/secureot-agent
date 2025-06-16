use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{IpAddr, TcpListener, UdpSocket};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// --- Estructuras de Configuración ---
/// Define la estructura de una regla de enrutamiento IP.
#[derive(Debug, Deserialize, Serialize, Clone)]
struct RoutingRule {
    ip: String, // La dirección IP a la que se aplicará la regla
    output_prefix: String, // Prefijo para los archivos PCAP de esta IP
}

/// Define la configuración de los directorios de salida.
#[derive(Debug, Deserialize, Serialize)]
struct OutputDirectoriesConfig {
    pcap_raw_output_path: String, // Directorio donde se escribirán los archivos PCAP crudos
}

/// Define la estructura de la configuración principal del receptor remoto.
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    port: u16, // Puerto en el que el programa escuchará el tráfico remoto
    protocol: String, // Protocolo a escuchar ("udp" o "tcp")
    routing_rules: Vec<RoutingRule>, // Lista de reglas de enrutamiento IP
    default_output_prefix: String, // Prefijo por defecto para IPs no cubiertas por reglas
    output_directories: OutputDirectoriesConfig, // Configuración de directorios de salida
}

// --- Gestión de Archivos PCAP ---
/// Estructura para gestionar un archivo PCAP con rotación horaria.
struct ManagedPcapFile {
    file: File,                 // El descriptor de archivo actual
    current_hour: u32,          // La hora (0-23) para la que este archivo está activo
    base_output_prefix: String, // El prefijo base para el nombre del archivo (ej. "plc1_traffic")
}

impl ManagedPcapFile {
    /// Crea una nueva instancia de ManagedPcapFile, abriendo un nuevo archivo PCAP con el timestamp actual.
    fn new(base_prefix: String, initial_timestamp: DateTime<Local>, output_path: &str) -> io::Result<Self> {
        let current_hour = initial_timestamp.hour();
        let filename = format!("{}/{}", output_path, Self::generate_filename(&base_prefix, initial_timestamp));
        let mut file = File::create(&filename)?; // Abre/crea el archivo
        write_pcap_header(&mut file)?; // Escribe el encabezado PCAP global
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
    fn get_current_file(&mut self, current_timestamp: DateTime<Local>, output_path: &str) -> io::Result<&mut File> {
        if current_timestamp.hour() != self.current_hour {
            println!("[PCAP Manager] Rotando archivo PCAP para {}. Nueva hora detectada.", self.base_output_prefix);

            let new_filename = format!("{}/{}", output_path, Self::generate_filename(&self.base_output_prefix, current_timestamp));

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
/// **MODIFICADO: LINKTYPE a LINKTYPE_RAW (101)**
fn write_pcap_header(mut file: &File) -> io::Result<()> {
    let header: [u8; 24] = [
        0xd4, 0xc3, 0xb2, 0xa1, // magic number
        0x02, 0x00, 0x04, 0x00, // version major, version minor
        0x00, 0x00, 0x00, 0x00, // thiszone
        0x00, 0x00, 0x00, 0x00, // sigfigs
        0xff, 0xff, 0x00, 0x00, // snaplen
        // 0x01, 0x00, 0x00, 0x00, // old network (LINKTYPE_ETHERNET)
        0x65, 0x00, 0x00, 0x00, // new network (LINKTYPE_RAW = 101 decimal = 0x65 hex)
    ];
    file.write_all(&header)?;
    Ok(())
}

/// Escribe un solo paquete al archivo PCAP, con su encabezado de paquete PCAP.
/// Para este programa, 'data' es el payload recibido por el socket.
/// Si la fuente remota no envía tramas Ethernet completas, LINKTYPE_RAW (101) es más apropiado.
fn write_pcap_packet(mut file: &File, data: &[u8]) -> io::Result<()> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds = now.as_secs() as u32;
    let micros = now.subsec_micros();
    let len = data.len() as u32; // captured length
    let original_len = len;     // original length

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

    let addr = format!("0.0.0.0:{}", config.port);
    let pcap_output_path = &config.output_directories.pcap_raw_output_path;

    // HashMap para gestionar múltiples ManagedPcapFile, mapeado por su output_prefix
    let mut pcap_file_managers: HashMap<String, ManagedPcapFile> = HashMap::new();

    // Parsear reglas de enrutamiento a IpAddr para búsquedas más eficientes
    let mut parsed_routing_rules: Vec<(IpAddr, String)> = Vec::new();
    for rule in &config.routing_rules {
        let ip_addr: IpAddr = rule.ip.parse()
            .expect(&format!("Dirección IP inválida en la configuración: {}", rule.ip));
        parsed_routing_rules.push((ip_addr, rule.output_prefix.clone()));
    }

    match config.protocol.to_lowercase().as_str() {
        "udp" => {
            let socket = UdpSocket::bind(&addr).expect("No se pudo abrir el puerto UDP");
            println!("[*] Escuchando UDP en {}", addr);
            let mut buf = [0u8; 65535]; // Buffer para paquetes UDP (max size)

            loop {
                let now = Local::now(); // Obtener la hora actual para la rotación de archivos
                match socket.recv_from(&mut buf) {
                    Ok((len, src_addr)) => {
                        let packet_data = &buf[..len];
                        println!("[UDP] Recibido {} bytes de {}", len, src_addr);

                        // --- Lógica de Enrutamiento por IP ---
                        // La IP relevante para el enrutamiento es la IP de ORIGEN del remitente del datagrama.
                        let peer_ip = src_addr.ip(); // IP del remitente del paquete UDP
                        let mut output_prefix = config.default_output_prefix.clone();

                        for (rule_ip, prefix) in &parsed_routing_rules {
                            // Comprobar si la IP del remitente coincide con alguna regla
                            if peer_ip == *rule_ip {
                                output_prefix = prefix.clone();
                                break;
                            }
                        }

                        let pcap_file_manager = pcap_file_managers
                            .entry(output_prefix.clone())
                            .or_insert_with(|| {
                                ManagedPcapFile::new(output_prefix.clone(), now, pcap_output_path)
                                    .expect("Fallo al crear un nuevo archivo PCAP gestionado")
                            });

                        // Escribir el paquete (payload) recibido al archivo PCAP correcto
                        if let Err(e) = write_pcap_packet(pcap_file_manager.get_current_file(now, pcap_output_path).expect("Fallo al obtener el archivo PCAP actual"), packet_data) {
                            eprintln!("Error al guardar paquete UDP en {}: {}", output_prefix, e);
                        }
                    }
                    Err(e) => eprintln!("Error al recibir UDP: {}", e),
                }
            }
        }
        "tcp" => {
            let listener = TcpListener::bind(&addr).expect("No se pudo abrir el puerto TCP");
            println!("[*] Escuchando TCP en {}", addr);

            for stream in listener.incoming() {
                match stream {
                    Ok(mut s) => {
                        let peer_addr = s.peer_addr().unwrap();
                        println!("[TCP] Conectado: {}", peer_addr);
                        let mut buf = [0u8; 65535]; // Buffer para datos TCP

                        loop {
                            let now = Local::now(); // Obtener la hora actual para la rotación de archivos
                            match s.read(&mut buf) {
                                Ok(0) => { // Conexión cerrada por el cliente
                                    println!("[TCP] Desconectado: {}", peer_addr);
                                    break;
                                }
                                Ok(len) => {
                                    let packet_data = &buf[..len];
                                    println!("[TCP] Recibido {} bytes", len);

                                    // --- Lógica de Enrutamiento por IP ---
                                    // La IP relevante para el enrutamiento es la IP de ORIGEN del cliente TCP.
                                    let peer_ip = peer_addr.ip(); // IP del cliente conectado
                                    let mut output_prefix = config.default_output_prefix.clone();

                                    for (rule_ip, prefix) in &parsed_routing_rules {
                                        // Comprobar si la IP del cliente coincide con alguna regla
                                        if peer_ip == *rule_ip {
                                            output_prefix = prefix.clone();
                                            break;
                                        }
                                    }

                                    let pcap_file_manager = pcap_file_managers
                                        .entry(output_prefix.clone())
                                        .or_insert_with(|| {
                                            ManagedPcapFile::new(output_prefix.clone(), now, pcap_output_path)
                                                .expect("Fallo al crear un nuevo archivo PCAP gestionado")
                                        });

                                    // Escribir el paquete (payload) recibido al archivo PCAP correcto
                                    if let Err(e) = write_pcap_packet(pcap_file_manager.get_current_file(now, pcap_output_path).expect("Fallo al obtener el archivo PCAP actual"), packet_data) {
                                        eprintln!("Error al guardar paquete TCP en {}: {}", output_prefix, e);
                                    }
                                }
                                Err(e) => { // Error de lectura del stream (ej., conexión reseteada)
                                    eprintln!("Error al leer TCP de {}: {}", peer_addr, e);
                                    break; // Romper el bucle y esperar la siguiente conexión
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Error de conexión TCP entrante: {}", e),
                }
            }
        }
        _ => {
            eprintln!("Error: Protocolo no soportado en la configuración: '{}'. Solo 'udp' o 'tcp' son válidos.", config.protocol);
            std::process::exit(1);
        }
    }
}
