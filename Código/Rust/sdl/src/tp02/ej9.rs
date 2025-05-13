pub fn cantidad_en_rango(arreglo: &[i32], li: i32, ls: i32) -> i32 {
    let mut cantidad = 0;
    for numero in arreglo {
        if numero >= li && numero <= ls {
            cantidad += 1;
        }
    }
    cantidad
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valores_en_rango() {
        let arreglo = vec![5, 10, 15, 20, 25];
        let li = 10;
        let ls = 20;
        let esperado = 3; // 10, 15, 20
        assert_eq!(cantidad_en_rango(&arreglo, li, ls), esperado);
    }

    #[test]
    fn test_sin_valores_en_rango() {
        let arreglo = vec![1, 2, 3];
        let li = 10;
        let ls = 20;
        let esperado = 0; // ninguno está entre 10 y 20
        assert_eq!(cantidad_en_rango(&arreglo, li, ls), esperado);
    }

    #[test]
    fn test_con_limites_iguales() {
        let arreglo = vec![5, 5, 5, 6];
        let li = 5;
        let ls = 5;
        let esperado = 3; // solo los 5 cuentan
        assert_eq!(cantidad_en_rango(&arreglo, li, ls), esperado);
    }
}
