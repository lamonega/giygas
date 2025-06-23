
## ✅ **Archivos con Serde**

- **Serde**: Framework para serializar/deserializar estructuras de Rust.
    
- **Formatos compatibles**: JSON, YAML, TOML, Pickle, BSON, etc.
    
- **Uso básico**:
    
    ```rust
    #[derive(Serialize, Deserialize, Debug)]
    struct Punto { x: i32, y: i32 }
    ```
    
- **Serializar a JSON**: `serde_json::to_string(&obj).unwrap()`
    
- **Guardar en archivo**: `File::create`, `write_all`
    
- **Leer y deserializar**: `File::open`, `read_to_string`, `serde_json::from_str`
    
- **Serializar colecciones**: Se puede serializar `Vec<T>` con estructuras derivadas.
    

---

## ✅ **Testing en Rust**

### 📌 Unit Testing

- **Propósito**: Probar pequeñas unidades de código.
    
- **Ventajas**:
    
    - Detección temprana de errores.
        
    - Mejora la calidad del código.
        
    - Facilita refactorizaciones.
        
    - Documentación viva.
        

### 📌 Sintaxis

```rust
#[test]
fn prueba() {
    assert!(expresion);
    assert_eq!(a, b);
    assert_ne!(a, b);
}
```

- **Comandos**:
    
    - `cargo test`
        
    - `cargo test nombre`
        
    - Atributos: `#[ignore]`, `#[should_panic(expected = "...")]`
        

### 📌 Coverage

- **Herramienta**: [`tarpaulin`](https://crates.io/crates/cargo-tarpaulin)
    
- **Comando**:
    
    ```bash
    cargo tarpaulin --target-dir src/coverage --skip-clean --out html
    ```
    

---

## ✅ **Smart Pointers**

### 📌 ¿Qué son?

- Son punteros con metadatos adicionales.
    
- Pueden **poseer** los datos que apuntan.
    
- Implementan `Drop`, `Deref`.
    

### 📌 `Box<T>`

- Heap allocation, ownership exclusivo.
    

```rust
let x = Box::new(5);
if *x == 5 { println!("es cinco!"); }
```

- Listas recursivas:
    

```rust
enum MiLista {
    Nodo(i32, Box<MiLista>),
    Nada
}
```

### 📌 `Deref`

- Permite usar `*obj` como si fuera el tipo apuntado.
    
- Se implementa para personalizar la desreferenciación:
    

```rust
impl<T> Deref for Caja<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.d
    }
}
```

### 📌 `Drop`

- Permite ejecutar código al liberar memoria:
    

```rust
impl<T> Drop for Caja<T> {
    fn drop(&mut self) {
        println!("Adiós!");
    }
}
```

- `drop(obj)` o dejar que el objeto salga de scope.
    

---

## ✅ **Rc – Reference Counted**

- Permite múltiples propietarios de un mismo valor.
    
- Uso común en listas o grafos.
    
- Cuenta automáticamente las referencias:
    

```rust
println!("{}", Rc::strong_count(&rc_val));
```

- Clonación con `Rc::clone(&rc_val)` (incrementa el contador).
    

### 📌 Ejemplo:

```rust
let nodo2_rc = Rc::new(Nodo(2, Rc::new(Nodo(3, Rc::new(Nada)))));
let nodo1 = Nodo(1, Rc::clone(&nodo2_rc));
let nodo5 = Nodo(5, Rc::clone(&nodo2_rc));
```

---

## ✅ **RefCell – Mutabilidad Interior**

- Permite mutar datos a través de referencias inmutables.
    
- Verifica reglas de borrowing **en tiempo de ejecución**.
    
- Común en combinación con `Rc<T>` para permitir acceso mutable compartido.
    

```rust
Nodo(RefCell::new(2), Rc::clone(&otro_nodo));
```

- `.borrow()` → acceso inmutable
    
- `.borrow_mut()` → acceso mutable
    

### 📌 Precaución:

- Pánico en tiempo de ejecución si se violan reglas de borrowing (e.g., doble mut borrow).
    

---

## ✅ **Resumen de Smart Pointers**

|Tipo|Ownership|Mutabilidad|Verificación|
|---|---|---|---|
|`Box<T>`|Único|Sí|Tiempo de compilación|
|`Rc<T>`|Compartido|No|Tiempo de compilación|
|`RefCell<T>`|Único|Sí|Tiempo de ejecución|
|`Rc<RefCell<T>>`|Compartido + mutable|Sí|Tiempo de ejecución|
