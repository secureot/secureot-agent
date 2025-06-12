use std::net::TcpListener;
use std::fs::File;
use std::io::{Read, Write};

pub fn start_reception() {
    let listener = TcpListener::bind("0.0.0.0:443").expect("Error al iniciar servidor");
    let mut file = File::create("received_traffic.pcap").expect("Error archivo");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Error en conexión");
        let mut buffer = [0; 4096];

        while let Ok(size) = stream.read(&mut buffer) {
            if size == 0 { break; }
            file.write_all(&buffer[..size]).expect("Error escritura");
        }
    }
}
