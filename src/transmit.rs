use tokio::net::{TcpStream, UdpSocket};
use native_tls::TlsConnector;
use tokio_native_tls::TlsConnector as TokioTlsConnector;
use tokio::io::AsyncWriteExt;
use std::error::Error;

pub async fn start_transmission(host: &str, port: u16, data: &[u8], use_ssl: bool) -> Result<(), Box<dyn Error>> {
    if use_ssl {
        send_tcp_tls(host, port, data).await?;
    } else {
        send_udp(host, port, data).await?;
    }
    Ok(())
}

pub async fn send_tcp_tls(host: &str, port: u16, data: &[u8]) -> Result<(), Box<dyn Error>> {
    let connector = TokioTlsConnector::from(TlsConnector::new().unwrap());
    let stream = TcpStream::connect(format!("{host}:{port}"))
        .await.map_err(|e| format!("❌ Error en conexión TCP: {}", e))?;

    let mut secure_stream = connector.connect(host, stream)
        .await.map_err(|e| format!("❌ Error en SSL/TLS: {}", e))?;

    secure_stream.write_all(data)
        .await.map_err(|e| format!("❌ Error al enviar datos: {}", e))?;

    Ok(())
}

pub async fn send_udp(host: &str, port: u16, data: &[u8]) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await.map_err(|e| format!("❌ Error en UDP: {}", e))?;

    socket.send_to(data, format!("{host}:{port}"))
        .await.map_err(|e| format!("❌ Error en envío UDP: {}", e))?;

    Ok(())
}
