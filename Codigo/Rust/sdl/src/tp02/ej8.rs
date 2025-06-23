pub fn sumar_arreglos(a1: &[f32], a2: &[f32]) -> Vec<f32> {
    let mut res = Vec::new();
    for i in 0..a1.len() {
        res.push(a1[i] + a2[i]);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma_basica() {
        let a1 = vec![1.0, 2.0, 3.0];
        let a2 = vec![4.0, 5.0, 6.0];
        let esperado = vec![5.0, 7.0, 9.0];
        assert_eq!(sumar_arreglos(&a1, &a2), esperado);
    }

    #[test]
    fn test_con_ceros() {
        let a1 = vec![0.0, 0.0, 0.0];
        let a2 = vec![1.1, 2.2, 3.3];
        let esperado = vec![1.1, 2.2, 3.3];
        assert_eq!(sumar_arreglos(&a1, &a2), esperado);
    }

    #[test]
    fn test_arreglos_vacios() {
        let a1: Vec<f32> = vec![];
        let a2: Vec<f32> = vec![];
        let esperado: Vec<f32> = vec![];
        assert_eq!(sumar_arreglos(&a1, &a2), esperado);
    }
}
