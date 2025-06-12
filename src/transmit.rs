use std::net::{UdpSocket, TcpStream};
use native_tls::TlsConnector;
use std::io::Write;

pub fn send_tcp_tls(host: &str, port: u16, data: &[u8], use_ssl: bool) {
    let stream = TcpStream::connect(format!("{host}:{port}")).expect("Error en conexión TCP");
    let mut stream = if use_ssl {
        let connector = TlsConnector::new().unwrap();
        connector.connect(host, stream).unwrap()
    } else {
        stream
    };
    stream.write_all(data).expect("Error al enviar datos");
}

pub fn send_udp(host: &str, port: u16, data: &[u8]) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Error en UDP");
    socket.send_to(data, format!("{host}:{port}")).expect("Error en envío UDP");
}
