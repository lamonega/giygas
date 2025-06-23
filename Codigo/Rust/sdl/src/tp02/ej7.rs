pub fn cantidad_de_mayores(arreglo: &[i32], limite: i32) -> i32 {
    let mut cantidad = 0;
    for numero in arreglo {
        if numero > limite {
            cantidad += 1;
        }
    }
    cantidad
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_con_valores_mayores() {
        let numeros = vec![5, 10, 15, 20];
        let limite = 10;
        let esperado = 2; // 15 y 20 son mayores que 10
        assert_eq!(cantidad_de_mayores(&numeros, limite), esperado);
    }

    #[test]
    fn test_sin_valores_mayores() {
        let numeros = vec![1, 2, 3, 4];
        let limite = 10;
        let esperado = 0; // Ninguno es mayor que 10
        assert_eq!(cantidad_de_mayores(&numeros, limite), esperado);
    }

    #[test]
    fn test_todos_iguales_al_limite() {
        let numeros = vec![7, 7, 7];
        let limite = 7;
        let esperado = 0; // Ninguno es mayor que 7
        assert_eq!(cantidad_de_mayores(&numeros, limite), esperado);
    }
}
