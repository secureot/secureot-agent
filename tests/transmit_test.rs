#[cfg(test)]
mod tests {
    use crate::transmit::{send_tcp_tls, send_udp};

    #[test]
    fn test_udp_transmission() {
        let data = vec![0x45, 0x00, 0x32];
        send_udp("127.0.0.1", 5000, &data);
        assert_eq!(data.len(), 3, "La transmisión UDP debe enviar los datos correctamente");
    }
}
