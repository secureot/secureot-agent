use std::net::{UdpSocket, TcpStream};
use native_tls::TlsConnector;
use std::io::Write;

pub fn start_transmission(host: &str, port: u16, data: &[u8], use_ssl: bool) {
    if use_ssl {
        send_tcp_tls(host, port, data);
    } else {
        send_udp(host, port, data);
    }
}

pub fn send_tcp_tls(host: &str, port: u16, data: &[u8]) {
    let stream = TcpStream::connect(format!("{host}:{port}")).expect("Error en conexión TCP");

    let connector = TlsConnector::new().expect("Error inicializando TLS Connector");
    let mut secure_stream = connector.connect(host, stream).expect("Error en SSL/TLS");

    secure_stream.write_all(data).expect("Error al enviar datos");
}

pub fn send_udp(host: &str, port: u16, data: &[u8]) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Error en UDP");
    socket.send_to(data, format!("{host}:{port}")).expect("Error en envío UDP");
}
