fn es_par(numero: u32) -> bool {
    numero % 2 == 0
}

fn suma_pares(valores: &[u32]) -> u32 {
    let mut suma: u32 = 0;
    for v in valores {
        if es_par(*v) {
            suma += v;
        }
    }
    suma
}

fn main() {
    let valores = [1, 2, 3, 4, 5, 6, 7, 8];
    let resultado = suma_pares(&valores);
    println!("La suma de los pares es: {}", resultado);
}
