pub fn cantidad_cadenas_mayor_a(cadenas: &[String], limite: i32) -> i32{
    let mut cantidad = 0;
    for cadena in cadenas {
        if cadena.chars().count() > limite {
            cantidad += 1;
        }
    }
    cantidad
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cadenas_varias_longitudes() {
        let cadenas = vec![
            String::from("hola"),      // 4
            String::from("mundo"),     // 5
            String::from("rustacean"), // 9
        ];
        let limite = 4;
        let esperado = 2; // "mundo" y "rustacean"
        assert_eq!(cantidad_cadenas_mayor_a(&cadenas, limite), esperado);
    }

    #[test]
    fn test_todas_menores_o_iguales() {
        let cadenas = vec![
            String::from("a"),
            String::from("bb"),
            String::from("ccc"),
        ];
        let limite = 3;
        let esperado = 0;
        assert_eq!(cantidad_cadenas_mayor_a(&cadenas, limite), esperado);
    }

    #[test]
    fn test_con_cadenas_vacias() {
        let cadenas = vec![
            String::from(""),
            String::from("rust"),
            String::from(""),
        ];
        let limite = 0;
        let esperado = 1; // Solo "rust" (4 caracteres) es > 0
        assert_eq!(cantidad_cadenas_mayor_a(&cadenas, limite), esperado);
    }
}
