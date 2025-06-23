fn es_par(numero: i32) -> bool {
    numero % 2 == 0
}

pub fn reemplazar_pares(arreglo: &mut[i32]) {
    for i in 0..arreglo.len() {
        if es_par(arreglo[i]) {
            arreglo[i] = -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn es_par(numero: i32) -> bool {
        numero % 2 == 0
    }

    #[test]
    fn test_reemplazo_normal() {
        let mut datos = vec![1, 2, 3, 4, 5];
        reemplazar_pares(&mut datos);
        assert_eq!(datos, vec![1, -1, 3, -1, 5]);
    }

    #[test]
    fn test_todos_pares() {
        let mut datos = vec![2, 4, 6];
        reemplazar_pares(&mut datos);
        assert_eq!(datos, vec![-1, -1, -1]);
    }

    #[test]
    fn test_ningun_par() {
        let mut datos = vec![1, 3, 5];
        reemplazar_pares(&mut datos);
        assert_eq!(datos, vec![1, 3, 5]); // No cambia nada
    }
}
