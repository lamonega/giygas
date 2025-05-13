pub fn es_primo(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_primo_edge_cases() {
        assert_eq!(es_primo(0), false);
        assert_eq!(es_primo(1), false);
    }

    #[test]
    fn test_es_primo_small_primes() {
        assert_eq!(es_primo(2), true);
        assert_eq!(es_primo(3), true);
        assert_eq!(es_primo(5), true);
        assert_eq!(es_primo(7), true);
    }

    #[test]
    fn test_es_primo_small_composites() {
        assert_eq!(es_primo(4), false);
        assert_eq!(es_primo(6), false);
        assert_eq!(es_primo(8), false);
        assert_eq!(es_primo(9), false);
        assert_eq!(es_primo(10), false);
    }

    #[test]
    fn test_es_primo_larger_numbers() {
        assert_eq!(es_primo(29), true);
        assert_eq!(es_primo(31), true);
        assert_eq!(es_primo(37), true);
        assert_eq!(es_primo(49), false);
        assert_eq!(es_primo(100), false);
    }
}