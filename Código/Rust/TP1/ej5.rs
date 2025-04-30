use std::io;

fn main() {
    let base = String::from("Rust es ");
    let mut extra = String::new();
    io::stdin().read_line(&mut extra).unwrap();

    let resultado = base + &extra.trim();
    println!("{}", resultado.to_uppercase());
}
