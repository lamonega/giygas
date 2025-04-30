fn main() {
    let t = ("Números", [2, 4, 6, 8, 10]);
    println!("{}", t.0);

    let mut suma = 0;
    for i in 0..t.1.len() {
        suma += t.1[i];
    }
    println!("{}", suma);
}
