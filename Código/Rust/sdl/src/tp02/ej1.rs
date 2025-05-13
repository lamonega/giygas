pub fn es_par(numero: i32) -> bool {
    numero % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_par_even_positive() {
        assert_eq!(es_par(2), true);
        assert_eq!(es_par(4), true);
        assert_eq!(es_par(100), true);
    }

    #[test]
    fn test_es_par_even_negative() {
        assert_eq!(es_par(-2), true);
        assert_eq!(es_par(-10), true);
    }

    #[test]
    fn test_es_par_odd_positive() {
        assert_eq!(es_par(1), false);
        assert_eq!(es_par(3), false);
        assert_eq!(es_par(99), false);
    }

    #[test]
    fn test_es_par_odd_negative() {
        assert_eq!(es_par(-1), false);
        assert_eq!(es_par(-7), false);
    }

    #[test]
    fn test_es_par_zero() {
        // Zero is even
        assert_eq!(es_par(0), true);
    }
}