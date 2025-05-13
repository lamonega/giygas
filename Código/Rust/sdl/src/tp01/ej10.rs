pub fn ej10() {
    let a = [1, 2, 3, 4, 5];
    let b = [10, 20, 30, 40, 50];
    let mut c = [0; 5];
    for i in 0..5 {
        c[i] = a[i] + b[i];
    }
    println!("{:?}", c);
}
