use std::io;

fn main() {
    let base: bool = true; // 1. Defino valor booleano base
    println!("Valor base: {}", base);
    println!("Ingrese un valor booleano: true o false"); // 2. Solicito por consola el input

    let mut input = String::new(); // 3. Preparo la variable donde almacenaré el input
    io::stdin().read_line(&mut input).expect("Error al leer"); // 4. Leo de teclado y almaceno como String.
    let x: bool = s.trim().parse().expect("Debe ser booleano"); // 5. "Casteo" a booleano

    // 6. Imprimo AND y OR de base y x.
    println!("{} AND {} = {}", base, x, base && x);
    println!("{} OR {} = {}", base, x, base || x);
}