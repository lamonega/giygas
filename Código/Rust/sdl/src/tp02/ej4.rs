
pub fn cantidad_impares(valores: &[u32]) -> u32 {
    let mut cantidad = 0;
    for &v in valores {
        if es_impar(v) {
            cantidad += 1;
        }
    }
    cantidad
}

// Asumiendo que la función es_impar se define así:
fn es_impar(n: u32) -> bool {
    n % 2 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantidad_impares_empty() {
        let valores: Vec<u32> = vec![];
        assert_eq!(cantidad_impares(&valores), 0);
    }

    #[test]
    fn test_cantidad_impares_all_even() {
        let valores = vec![2, 4, 6, 8, 10];
        // No hay números impares
        assert_eq!(cantidad_impares(&valores), 0);
    }

    #[test]
    fn test_cantidad_impares_all_odd() {
        let valores = vec![1, 3, 5, 7, 9];
        // Todos son impares, cantidad = longitud del vector
        assert_eq!(cantidad_impares(&valores), 5);
    }

    #[test]
    fn test_cantidad_impares_mixed() {
        let valores = vec![1, 2, 3, 4, 5];
        // Impares: 1, 3, 5 => cantidad = 3
        assert_eq!(cantidad_impares(&valores), 3);
    }

    #[test]
    fn test_cantidad_impares_repeated() {
        let valores = vec![1, 1, 2, 2, 3, 3, 4, 4];
        // Impares: 1, 1, 3, 3 => cantidad = 4
        assert_eq!(cantidad_impares(&valores), 4);
    }

    #[test]
    fn test_cantidad_impares_large_numbers() {
        let valores = vec![999, 1000, 1001, 1002];
        // Impares: 999, 1001 => cantidad = 2
        assert_eq!(cantidad_impares(&valores), 2);
    }
}

// We recommend installing an extension to run rust tests.