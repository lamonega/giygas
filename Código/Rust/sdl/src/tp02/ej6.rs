pub fn longitud_de_cadenas(cadenas: &[String]) -> Vec<i32> {
    let mut res = Vec::new();

    for cadena in cadenas {
        let cantidad = cadena.chars().count() as i32;
        res.push(cantidad);
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*; // Importa la función que estás probando

    #[test]
    fn test_palabras_normales() {
        let entrada = vec![
            String::from("hola"),
            String::from("mundo"),
            String::from("rust"),
        ];
        let esperado = vec![4, 5, 4];
        assert_eq!(longitud_de_cadenas(&entrada), esperado);
    }

    #[test]
    fn test_palabras_vacias() {
        let entrada = vec![
            String::from(""),
            String::from(""),
        ];
        let esperado = vec![0, 0];
        assert_eq!(longitud_de_cadenas(&entrada), esperado);
    }

    #[test]
    fn test_caracteres_especiales() {
        let entrada = vec![
            String::from("¡hola!"),
            String::from("¿qué tal?"),
        ];
        let esperado = vec![6, 9]; // cuenta los caracteres, incluidos los signos y espacios
        assert_eq!(longitud_de_cadenas(&entrada), esperado);
    }
}