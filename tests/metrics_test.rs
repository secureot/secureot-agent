#[cfg(test)]
mod tests {
    use crate::metrics::init_metrics;

    #[test]
    fn test_metrics_initialization() {
        let metrics = init_metrics();
        assert_eq!(metrics.packets_captured.get(), 0, "Las métricas deben iniciar en 0");
    }
}
