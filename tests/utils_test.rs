#[cfg(test)]
mod tests {
    use crate::utils::{optimize_memory, format_bytes};

    #[test]
    fn test_memory_optimization() {
        let raw_data = vec![0x00, 0x45, 0x00, 0x32];
        let optimized_data = optimize_memory(raw_data.clone());
        assert!(optimized_data.len() < raw_data.len(), "La optimización debe reducir bytes innecesarios");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB", "El formato de bytes debe ser correcto");
    }
}
