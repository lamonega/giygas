pub fn incrementar(numero: f64) -> f64 {
    numero + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremento_basico() {
        assert_eq!(incrementar(1.0), 2.0);
    }

    #[test]
    fn test_incremento_negativo() {
        assert_eq!(incrementar(-2.5), -1.5);
    }

    #[test]
    fn test_incremento_con_decimales() {
        assert_eq!(incrementar(3.14), 4.14);
    }
}
