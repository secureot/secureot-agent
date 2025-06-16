use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream, UdpSocket, ToSocketAddrs};
use std::time::{SystemTime, Duration, UNIX_EPOCH};

use chrono::prelude::*;
use serde::Deserialize;
use pcap;

// ---------- Configuración ----------

#[derive(Debug, Deserialize)]
struct RoutingRule {
    ip: String,
    output_prefix: String,
}

#[derive(Debug, Deserialize)]
struct CaptureConfig {
    interface: String,
    bpf_filter: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OutputDirectoriesConfig {
    pcap_raw_output_path: String,
}

#[derive(Debug, Deserialize, Clone)]
struct ForwardConfig {
    enable: bool,
    host: String,
    port: u16,
    protocol: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    capture: CaptureConfig,
    routing_rules: Vec<RoutingRule>,
    default_output_prefix: String,
    output_directories: OutputDirectoriesConfig,
    forward: ForwardConfig,
}

// ---------- PCAP Writer ----------

struct ManagedPcapFile {
    file: File,
    current_hour: u32,
    prefix: String,
}

impl ManagedPcapFile {
    fn new(prefix: &str, ts: DateTime<Local>, path: &str) -> io::Result<Self> {
        let filename = format!("{}/{}_{}.pcap", path, prefix, ts.format("%Y%m%d_%H"));
        let mut file = File::create(&filename)?;
        write_pcap_header(&mut file)?;
        Ok(Self {
            file,
            current_hour: ts.hour(),
            prefix: prefix.into(),
        })
    }

    fn get_file(&mut self, ts: DateTime<Local>, path: &str) -> io::Result<&mut File> {
        if ts.hour() != self.current_hour {
            let filename = format!("{}/{}_{}.pcap", path, self.prefix, ts.format("%Y%m%d_%H"));
            let mut new_file = File::create(&filename)?;
            write_pcap_header(&mut new_file)?;
            self.file = new_file;
            self.current_hour = ts.hour();
        }
        Ok(&mut self.file)
    }
}

fn write_pcap_header(file: &mut File) -> io::Result<()> {
    let hdr: [u8; 24] = [
        0xd4, 0xc3, 0xb2, 0xa1, 0x02, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];
    file.write_all(&hdr)
}

fn write_pcap_packet(file: &mut File, data: &[u8]) -> io::Result<()> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ts_sec = now.as_secs() as u32;
    let ts_usec = now.subsec_micros();
    let len = data.len() as u32;

    file.write_all(&ts_sec.to_le_bytes())?;
    file.write_all(&ts_usec.to_le_bytes())?;
    file.write_all(&len.to_le_bytes())?;
    file.write_all(&len.to_le_bytes())?;
    file.write_all(data)?;
    file.flush()
}

// ---------- Reenvío ----------

fn forward_packet(cfg: &ForwardConfig, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    if !cfg.enable {
        return Ok(());
    }

    let addr = format!("{}:{}", cfg.host, cfg.port);

    match cfg.protocol.to_lowercase().as_str() {
        "udp" => {
            let sock = UdpSocket::bind("0.0.0.0:0")?;
            sock.set_write_timeout(Some(Duration::from_secs(2)))?;
            sock.send_to(data, addr)?;
        }
        "tcp" => {
            let mut stream = TcpStream::connect_timeout(
                &addr.to_socket_addrs()?.next().ok_or("No se pudo resolver destino")?,
                Duration::from_secs(2),
            )?;
            stream.write_all(data)?;
            stream.flush()?;
        }
        other => {
            return Err(format!("Protocolo no soportado: {}", other).into());
        }
    }

    Ok(())
}

// ---------- IP Routing ----------

fn parse_ip_addrs(pkt: &[u8]) -> Option<(IpAddr, IpAddr)> {
    if pkt.len() >= 34 && pkt[12] == 0x08 && pkt[13] == 0x00 {
        let src = IpAddr::from([pkt[26], pkt[27], pkt[28], pkt[29]]);
        let dst = IpAddr::from([pkt[30], pkt[31], pkt[32], pkt[33]]);
        Some((src, dst))
    } else {
        None
    }
}

// ---------- Main ----------

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Uso: {} <config.yaml>", args[0]);
        std::process::exit(1);
    }

    let config_str = std::fs::read_to_string(&args[1]).expect("No se pudo leer el archivo YAML");
    let config: Config = serde_yaml::from_str(&config_str).expect("YAML inválido");

    let mut cap = pcap::Capture::from_device(config.capture.interface.as_str())
        .unwrap()
        .promisc(true)
        .snaplen(65535)
        .timeout(100)
        .open()
        .unwrap();

    if let Some(bpf) = &config.capture.bpf_filter {
        cap.filter(bpf, true).expect("Filtro BPF inválido");
    }

    let rules: Vec<(IpAddr, String)> = config.routing_rules.iter()
        .map(|r| (r.ip.parse().unwrap(), r.output_prefix.clone()))
        .collect();

    let mut pcaps: HashMap<String, ManagedPcapFile> = HashMap::new();

    loop {
        if let Ok(packet) = cap.next() {
            let ts = Local::now();
            let prefix = parse_ip_addrs(packet.data)
                .and_then(|(src, dst)| {
                    rules.iter()
                        .find(|(ip, _)| ip == &src || ip == &dst)
                        .map(|(_, p)| p.clone())
                })
                .unwrap_or_else(|| config.default_output_prefix.clone());

            let mgr = pcaps.entry(prefix.clone())
                .or_insert_with(|| ManagedPcapFile::new(&prefix, ts, &config.output_directories.pcap_raw_output_path).unwrap());

            let file = mgr.get_file(ts, &config.output_directories.pcap_raw_output_path).unwrap();
            let _ = write_pcap_packet(file, packet.data);
            let _ = forward_packet(&config.forward, packet.data);
        }
    }
}
