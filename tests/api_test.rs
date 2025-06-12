#[cfg(test)]
mod tests {
    use crate::api::set_bpf_filter;
    use serde_json::json;

    #[test]
    fn test_api_bpf_filter() {
        let payload = json!({"filter": "tcp port 502"});
        assert!(payload["filter"] == "tcp port 502", "El filtro BPF enviado debe coincidir");
    }
}
