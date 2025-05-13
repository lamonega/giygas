pub fn multiplicar_valores(arreglo: &mut [i32], factor: i32) {
    for i in 0..arreglo.len() {
        arreglo[i] *= factor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplicacion_normal() {
        let mut datos = vec![1, 2, 3];
        multiplicar_valores(&mut datos, 2);
        assert_eq!(datos, vec![2, 4, 6]);
    }

    #[test]
    fn test_multiplicacion_por_cero() {
        let mut datos = vec![5, 10, 15];
        multiplicar_valores(&mut datos, 0);
        assert_eq!(datos, vec![0, 0, 0]);
    }

    #[test]
    fn test_arreglo_vacio() {
        let mut datos: Vec<i32> = vec![];
        multiplicar_valores(&mut datos, 10);
        assert_eq!(datos, vec![]); // Sigue vacío
    }
}
