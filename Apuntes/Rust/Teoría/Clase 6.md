
## 🔁 **Iterators**

### ¿Qué es un Iterator?

Un _iterator_ permite recorrer una colección sin revelar su estructura interna.

### Uso en Rust

Las colecciones como `Vec`, `LinkedList`, `HashMap` implementan el trait `Iterator`.

```rust
let mut iter_v = v.iter(); // Iterador inmutable sobre Vec
iter_v.next();             // Devuelve Some(...) o None
```

### Métodos útiles de los iteradores

- `clone()`
    
- `cycle()`
    
- `enumerate()`
    
- `take(n)`
    
- `step_by(n)`
    
- `skip(n)`
    
- `chain()` → Combina dos iteradores
    
- `filter()`, `filter_map()`, `all()`, `any()`, `skip_while()`
    

### Iterar con for y while

```rust
for i in iter_v.chain(iter_l) {
    println!("{:?}", i);
}

while let Some(e) = otro.next() {
    println!("{:?}", e);
}
```

### Implementar tu propio `Iterator`

```rust
struct Caja { c: i32 }

impl Default for Caja {
    fn default() -> Self { Caja { c: 0 } }
}

impl Iterator for Caja {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.c < 10 {
            self.c += 1;
            Some(self.c)
        } else {
            None
        }
    }
}
```

---

## ⚠️ **Manejo de errores**

### Tipos de errores

- **Recuperables**: Usan `Result<T, E>`
    
- **Irrecuperables**: Usan `panic!`
    

### panic!

```rust
let v = vec![1, 2, 3];
v[10]; // panic!
```

Usar `RUST_BACKTRACE=1` para trazar el error.

### Result<T, E>

```rust
let result = "123".parse::<i32>();
match result {
    Ok(num) => println!("{}", num),
    Err(e) => println!("Error: {}", e),
}
```

### Operador `?`

Propaga el error si ocurre:

```rust
fn convertir(s: String) -> Result<i32, ParseIntError> {
    let dato = s.parse::<i32>()?;
    Ok(dato)
}
```

### Uso en `Option`

```rust
impl Persona {
    fn codigo_area(&self) -> Option<u8> {
        self.trabajo?.telefono?.codigo_de_area
    }
}
```

### expect()

```rust
let n = "12".parse::<i32>().expect("No se pudo convertir");
```

### Errores custom

```rust
struct MiError(String);

impl Display for MiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
```

---

## 🔐 **Errores Custom Avanzados (con lógica de permisos)**

- `PermisoError` para casos como usuarios sin permisos para cambiar estado
    
- Estructura:
    
    - `Usuario` con rol (Staff/Admin)
        
    - `Producto` con estado (`INI`, `ACT`, `FIN`)
        
    - Método `cambiar_estado()` → valida permisos
        
    - Error personalizado si no tiene permisos
        

---

## 📦 **Prelude**

- El _Prelude_ de Rust contiene tipos y traits comunes importados automáticamente.
    
- Ejemplos:
    
    - `Option`, `Result`
        
    - `Vec`, `String`
        
    - Traits como `Copy`, `Clone`, `Debug`
        

Referencia: [https://doc.rust-lang.org/std/prelude/](https://doc.rust-lang.org/std/prelude/)

---

## 📄 **Archivos**

### Leer archivo completo

```rust
let mut archivo = File::open("archivo.txt").expect("No se pudo abrir");
let mut contenido = String::new();
archivo.read_to_string(&mut contenido).expect("No se pudo leer");
```

### Crear archivo y escribir

```rust
let mut archivo = File::create("nuevo.txt").expect("Error al crear");
archivo.write_all("Texto".as_bytes()).expect("No se pudo escribir");
```

### Leer línea por línea

```rust
let archivo = File::open("archivo.txt").unwrap();
let lineas = BufReader::new(archivo).lines();
for linea in lineas {
    println!("{:?}", linea);
}
```
