pub fn suma_pares(valores: &[u32]) -> u32 {
    let mut suma: u32 = 0;
    for v in valores {
        if es_par(*v) {
            suma += *v;
        }
    }
    suma
}

fn es_par(n: u32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma_pares_empty() {
        let valores: Vec<u32> = vec![];
        assert_eq!(suma_pares(&valores), 0);
    }

    #[test]
    fn test_suma_pares_all_even() {
        let valores = vec![2, 4, 6, 8];
        // Suma: 2+4+6+8 = 20
        assert_eq!(suma_pares(&valores), 20);
    }

    #[test]
    fn test_suma_pares_all_odd() {
        let valores = vec![1, 3, 5, 7];
        // No hay números pares, suma = 0
        assert_eq!(suma_pares(&valores), 0);
    }

    #[test]
    fn test_suma_pares_mixed() {
        let valores = vec![1, 2, 3, 4, 5, 6];
        // Números pares: 2, 4, 6 => suma = 12
        assert_eq!(suma_pares(&valores), 12);
    }

    #[test]
    fn test_suma_pares_single_even() {
        let valores = vec![10];
        assert_eq!(suma_pares(&valores), 10);
    }

    #[test]
    fn test_suma_pares_single_odd() {
        let valores = vec![11];
        assert_eq!(suma_pares(&valores), 0);
    }
}