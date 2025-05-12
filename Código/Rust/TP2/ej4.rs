fn es_impar(numero: u32) -> bool {
    numero % 2 != 0
}

fn cantidad_impares(valores: &[u32]) -> u32 {
    let mut cantidad = 0;
    for &v in valores {
        if es_impar(v) {
            cantidad += 1;
        }
    }
    cantidad
}

fn main() {
    let valores = [1, 2, 3, 4, 5, 6, 7, 8];
    let resultado = cantidad_impares(&valores);
    println!("La cantidad de impares es: {}", resultado);
}