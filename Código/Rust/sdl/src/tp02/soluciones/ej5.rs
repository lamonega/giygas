fn copiar_arreglo(entrada: &[f32]) -> Vec<f32> {
    entrada.to_vec()
}

fn main() {
    let original = [10, 20, 30, 40];
    let copia = copiar_arreglo(&original);
    println!("{:?}", copia);
}