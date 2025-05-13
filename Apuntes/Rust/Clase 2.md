## 🔁 ESTRUCTURAS DE CONTROL

### `if` y `if-else`

```rust
if condicion {
    // se ejecuta si la condición es verdadera
} else {
    // se ejecuta si la condición es falsa
}
```

### `if-else if`

```rust
if condicion {
    // ...
} else if otra_condicion {
    // ...
} else {
    // ninguna de las anteriores se cumplió
}
```

### `if` con declaración

```rust
let valor = if condicion { 1 } else { 0 };
```

### `match`

`match` permite evaluar patrones de forma más expresiva:

```rust
match valor {
    1 => println!("uno"),
    2 => println!("dos"),
    _ => println!("otro"), // comodín
}
```

Patrones válidos: literales, tuplas, enums, arrays, structs, variables, comodines.

### `loop`

Ciclo infinito (hasta `break`):

```rust
loop {
    if condicion {
        break;
    }
}
```

También se puede devolver un valor:

```rust
let resultado = loop {
    if condicion {
        break 42;
    }
};
```

### `loop` con etiquetas

Permite salir de bucles anidados:

```rust
'outer: loop {
    loop {
        break 'outer;
    }
}
```

### `while`

```rust
while condicion {
    // código
}
```

### `for`

Iterar sobre arrays o rangos:

```rust
for i in 1..=5 {
    println!("{}", i);
}

for i in (1..=5).rev() {
    println!("{}", i);
}
```

---

## 🧩 FUNCIONES

### Definición básica

```rust
fn nombre(arg1: Tipo, arg2: Tipo) {
    // cuerpo
}
```

Usamos `snake_case` y los tipos deben estar siempre explícitos.

### Ejemplo simple

```rust
fn imprimir(mensaje: &str) {
    println!("{}", mensaje);
}
```

### Devolver valores

```rust
fn suma(a: i32, b: i32) -> i32 {
    a + b
}
```

También se puede usar `return`, pero no es obligatorio si el último valor se devuelve sin `;`.

---

## 📦 OWNERSHIP & BORROWING

### ¿Qué es ownership?

Rust maneja la memoria automáticamente con un sistema de **propiedad**.
#### Reglas:
1. Cada valor tiene un **dueño**.
2. Solo puede haber **un dueño** a la vez.
3. Cuando el dueño **sale de alcance**, el valor se libera automáticamente.
### Tipos `Copy`
Algunos tipos como `i32`, `bool`, `char`, `f64` o tuplas de esos tipos se copian en vez de moverse.
### Ejemplo de "mover" (`String`)
```rust
let s1 = String::from("Hola");
let s2 = s1; // s1 ya no es válido
```
### Borrowing (préstamos)
```rust
fn imprimir(data: &String) {
    println!("{}", data); // solo lee
}
```
### Mutable Borrowing
```rust
fn modificar(data: &mut String) {
    data.push_str(" mundo!");
}
```

En este caso solo puede haber **una referencia mutable activa** a la vez.

---

## 🕒 LIFETIMES

### ¿Qué es un lifetime?

Especifica cuánto tiempo es válida una referencia. Rust verifica que nunca tengamos referencias a datos eliminados.

### Inferencia automática

Rust deduce los lifetimes la mayoría del tiempo. Pero a veces se necesita ser explícito.

### Lifetime explícito

```rust
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Ambas referencias deben tener el mismo lifetime `'a`.

### Ejemplo de error

```rust
let r;
{
    let x = 5;
    r = &x;
}
println!("{}", r); // ⚠ error: x ya no vive
```

Rust lo detecta en tiempo de compilación.

---

## 🧪 TESTING EN RUST

### ¿Qué es unit testing?

Pruebas automáticas que verifican que una función haga lo que debe.
### Sintaxis en Rust

```rust
#[test]
fn prueba_suma() {
    assert_eq!(2 + 2, 4);
}
```

Otros macros útiles:
- `assert!(condición)`
- `assert_ne!(a, b)`

### Ignorar test o esperar errores

```rust
#[ignore]
fn test_lento() {}

#[should_panic(expected = "error")]
fn test_panico() {
    panic!("error");
}
```

### Ejecutar test

```sh
cargo test
cargo test nombre_test
```

### Ejemplo de función con test

```rust
fn contar_letras(texto: &str, letra: char) -> usize {
    texto.chars().filter(|&c| c == letra).count()
}

#[test]
fn test_contar() {
    assert_eq!(contar_letras("banana", 'a'), 3);
}
```
