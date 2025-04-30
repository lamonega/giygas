use std::io;

fn main() {
    let a = true;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let b: bool = buf.trim().parse().unwrap();

    println!("{}", a && b);
    println!("{}", a || b);
}
