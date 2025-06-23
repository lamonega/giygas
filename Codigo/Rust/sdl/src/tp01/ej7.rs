pub fn ej7() {
    const F:i32 = 3;
    let mut arr = [1, 2, 3, 4, 5, 6];
    for i in 0..arr.len() {
        arr[i] = arr[i] * F;
    }
    println!("{:?}", arr);
}
