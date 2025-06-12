#[cfg(test)]
mod tests {
    use crate::config::read_config;

    #[test]
    fn test_config_loading() {
        let config = read_config();
        assert_eq!(config.mode, "dual", "El modo de operación debe ser 'dual'");
    }
}
