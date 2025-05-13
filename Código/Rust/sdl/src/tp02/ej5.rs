pub fn copiar_arreglo(entrada: &[f32]) -> Vec<f32> {
    entrada.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copiar_arreglo_empty() {
        let entrada: Vec<f32> = vec![];
        let salida = copiar_arreglo(&entrada);
        assert_eq!(salida, Vec::<f32>::new());
    }

    #[test]
    fn test_copiar_arreglo_single_element() {
        let entrada = vec![3.14];
        let salida = copiar_arreglo(&entrada);
        assert_eq!(salida, vec![3.14]);
    }

    #[test]
    fn test_copiar_arreglo_multiple_elements() {
        let entrada = vec![1.1, 2.2, 3.3, 4.4];
        let salida = copiar_arreglo(&entrada);
        assert_eq!(salida, entrada);
    }

    #[test]
    fn test_copiar_arreglo_independence() {
        let entrada = vec![5.5, 6.6, 7.7];
        let mut salida = copiar_arreglo(&entrada);
        // Modify the copied vector
        salida[0] = 0.0;
        // The original vector should remain unchanged
        assert_ne!(salida, entrada);
    }
}