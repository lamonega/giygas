pub fn ej9() {
    let arr = [10, 20, 30, 40, 50];
    let mut suma = 0;
    for i in 0..arr.len() {
        suma += arr[i];
    }
    println!("{}", suma);
}
