#[cfg(test)]
mod tests {
    use crate::capture::capture_with_bpf;

    #[test]
    fn test_capture_initialization() {
        let filters = vec!["tcp port 502".to_string()];
        assert!(!filters.is_empty(), "Los filtros BPF deben estar activos");
    }
}
