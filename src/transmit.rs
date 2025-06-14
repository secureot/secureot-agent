use tokio::net::{TcpStream, UdpSocket};
use native_tls::TlsConnector;
use tokio_native_tls::TlsConnector as TokioTlsConnector;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::time::Duration;

/// Inicia transmisión según el protocolo seleccionado (TCP con TLS, TCP sin encriptado, UDP)
pub async fn start_transmission(host: &str, port: u16, data: &[u8], mode: &str) -> Result<(), Box<dyn Error>> {
    if host.is_empty() || port == 0 || data.is_empty() {
        return Err("❌ Entrada inválida: host vacío, puerto 0 o datos vacíos.".into());
    }

    match mode {
        "tls" => send_tcp_tls(host, port, data).await,
        "tcp" => send_tcp_plain(host, port, data).await,
        "udp" => send_udp(host, port, data).await,
        _ => Err(format!("❌ Modo de transmisión desconocido: {}", mode).into()),
    }
}

/// Envía datos vía TCP con TLS, manejando timeouts
pub async fn send_tcp_tls(host: &str, port: u16, data: &[u8]) -> Result<(), Box<dyn Error>> {
    let connector = TokioTlsConnector::from(TlsConnector::new()?);
    let addr = format!("{host}:{port}");

    let stream = match tokio::time::timeout(Duration::from_secs(5), TcpStream::connect(&addr)).await {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => return Err(format!("❌ Error en conexión TCP: {}", e).into()),
        Err(_) => return Err("❌ Timeout: El servidor no respondió en 5 segundos.".into()),
    };

    let mut secure_stream = connector.connect(host, stream)
        .await.map_err(|e| format!("❌ Error en SSL/TLS: {}", e))?;

    secure_stream.write_all(data)
        .await.map_err(|e| format!("❌ Error al enviar datos: {}", e))?;

    Ok(())
}

/// Envía datos vía TCP sin encriptado, evitando bloqueos
pub async fn send_tcp_plain(host: &str, port: u16, data: &[u8]) -> Result<(), Box<dyn Error>> {
    let addr = format!("{host}:{port}");

    let mut stream = match tokio::time::timeout(Duration::from_secs(5), TcpStream::connect(&addr)).await {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => return Err(format!("❌ Error en conexión TCP: {}", e).into()),
        Err(_) => return Err("❌ Timeout: El servidor no respondió en 5 segundos.".into()),
    };

    stream.write_all(data)
        .await.map_err(|e| format!("❌ Error al enviar datos: {}", e))?;

    Ok(())
}

/// Envía datos vía UDP, manejando errores correctamente
pub async fn send_udp(host: &str, port: u16, data: &[u8]) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await.map_err(|e| format!("❌ Error en UDP: {}", e))?;

    socket.send_to(data, format!("{host}:{port}"))
        .await.map_err(|e| format!("❌ Error en envío UDP: {}", e))?;

    Ok(())
}
