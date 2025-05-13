use std::io;

pub fn ej11() {
    let arr = ["manzana", "banana", "uva", "pera", "kiwi"];
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let bus = buf.trim();

    let mut found = false;
    for &item in &arr {
        if item == bus {
            found = true;
        }
    }
    if found {
        println!("Está");
    } else {
        println!("No está");
    }
}
