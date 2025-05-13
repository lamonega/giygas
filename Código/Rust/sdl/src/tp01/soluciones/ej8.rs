use std::io;

fn main() {
    const TEXTO = "Rust es divertido";
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let c = buf.chars().next().unwrap();

    let mut cnt = 0;
    for ch in TEXTO.chars() {
        if ch == c {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
