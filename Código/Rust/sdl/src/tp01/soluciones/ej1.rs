use std::io;

fn main() {
    let base: f64 = 3.14;
    println!("Valor base: {}", base);
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let x: f64 = buf.trim().parse().unwrap();

    println!("{}", base + x);
    println!("{}", base - x);
    println!("{}", base * x);
    if x != 0.0 {
        println!("{}", base / x);
    }
}
