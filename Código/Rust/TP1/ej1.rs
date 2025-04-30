use std::io;

fn main() {
    let base: f64 = 3.14;
    println!("Valor base: {}", base);
    println!("Ingresa un número decimal:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer");
    let x: f64 = input.trim().parse().expect("Debe ser un decimal");

    println!("{} + {} = {}", base, x, base + x);
    println!("{} - {} = {}", base, x, base - x);
    println!("{} * {} = {}", base, x, base * x);
    if x != 0.0 {
        println!("{} / {} = {}", base, x, base / x);
    } else {
        println!("No se puede dividir por cero");
    }
}