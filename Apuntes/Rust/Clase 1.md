EVALUACIÓN:
- 2 TP INDIVIDUALES (08/05 - 12/06)
- 1 TP GRUPAL FINAL (10/07)
Promocionable. Las dos notas individuales + la grupal conforman la nota final de la materia. 4 promociona.
 
TP Individuales: Se pide realizar un ejercicio de la práctica, y luego otro muy similar con alguna modificación. Puede no entregarse en el mismo día, pero tiene que haber cierto progreso.
Es recomendable tener todos los trabajos prácticos hechos hasta el momento de la evaluación.

TP Grupal: Desarrollar basado en blockchain.

----
### Comentarios

- Línea: `// esto es un comentario`
    
- Bloque:
    
    ```rust
    /*
       esto es un comentario
       de varias líneas
    */
    ```
    

---

### Variables y mutabilidad

Por defecto, **todas las variables son inmutables**:

```rust
let x = 5; // no puede cambiarse
```

Para hacerla mutable:

```rust
let mut y = 6;
y = y + 1;
```

**Shadowing** (reasignar una nueva variable con el mismo nombre):

```rust
let z = 5;
let z = z + 1;
```

---

### Constantes

- Se definen con `const`
    
- Siempre requieren tipo
    
- Son inmutables **y no pueden cambiar nunca**
    

```rust
const PI: f32 = 3.14;
```

---

### Tipos de datos

#### Escalares

- **Enteros:** `i32`, `u64`, etc.  
    `i` = con signo, `u` = sin signo.  
    Ej: `let n: u32 = 100;`
    
- **Flotantes:** `f32`, `f64`  
    Ej:
    
    ```rust
    let x = 3.0;     // f64 por defecto
    let y: f32 = 2.5;
    ```
    
- **Booleanos:** `true`, `false`  
    Ej: `let b: bool = true;`
    
- **Caracteres (`char`)**  
    Unicode completo (no solo ASCII).  
    Ej:
    
    ```rust
    let letra = 'z';
    let emoji = '😻';
    ```
    

#### Operaciones básicas

```rust
let suma = 2 + 2;
let resto = 10 % 3;
let mayor = 5 > 3;
```

---

#### Compuestos

##### String

- `&str`: inmutable, tamaño fijo
    
- `String`: mutable, tamaño variable
    

```rust
let saludo: &str = "Hola";
let mut nombre = String::from("Rust");
nombre += "ace";
```

##### Tuple

Agrupa valores de distintos tipos:

```rust
let t: (i32, f64, char) = (42, 3.14, 'a');
let (x, y, z) = t;
```

##### Array

Tamaño fijo, mismo tipo:

```rust
let a = [1, 2, 3];
println!("{}", a[0]);
```

---

### Uso de librerías (`use`)

Importar módulos para reutilizar funciones:

```rust
use std::io::stdin;

fn main() {
    let mut nombre = String::new();
    stdin().read_line(&mut nombre).expect("Error");
    println!("Hola, {}!", nombre);
}
```

- `expect`: muestra mensaje si hay error.
    
- `stdin().read_line(...)` devuelve un `Result`.
    

---

### Cargo: gestor de paquetes y proyectos

Permite crear proyectos, compilar y manejar dependencias.

**Crear un proyecto nuevo:**

```bash
cargo new mi_proyecto
```

Estructura básica:

- `Cargo.toml`: configuración y dependencias
    
- `src/main.rs`: punto de entrada
    

**Otros comandos útiles:**

- `cargo build`: compilar
    
- `cargo run`: compilar y ejecutar
    
- `cargo check`: verificar errores sin compilar binario
    

**Repositorio de paquetes:**  
[https://crates.io](https://crates.io)