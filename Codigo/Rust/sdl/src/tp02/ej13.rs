pub fn ordenar_nombres(nombres: &mut [String]) {
    nombres.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orden_alfabetico() {
        let mut nombres = vec![
            String::from("Carlos"),
            String::from("Ana"),
            String::from("Bruno"),
        ];
        ordenar_nombres(&mut nombres);
        assert_eq!(
            nombres,
            vec![
                String::from("Ana"),
                String::from("Bruno"),
                String::from("Carlos")
            ]
        );
    }

    #[test]
    fn test_nombres_iguales() {
        let mut nombres = vec![
            String::from("Ana"),
            String::from("Ana"),
            String::from("Ana"),
        ];
        ordenar_nombres(&mut nombres);
        assert_eq!(
            nombres,
            vec![
                String::from("Ana"),
                String::from("Ana"),
                String::from("Ana")
            ]
        );
    }

    #[test]
    fn test_arreglo_vacio() {
        let mut nombres: Vec<String> = vec![];
        ordenar_nombres(&mut nombres);
        assert_eq!(nombres, vec![]);
    }
}
