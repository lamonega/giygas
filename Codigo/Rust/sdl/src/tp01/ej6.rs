use std::io;

pub fn ej6() {
    let a: u32 = 7;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let b: u32 = buf.trim().parse().unwrap();

    let s = a + b;
    println!("{}", s * s);
}
