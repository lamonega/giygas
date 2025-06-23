
### 🧪 **Mocking**

**Objetivo:** Facilitar tests unitarios sin depender de lógica compleja o llamadas externas.

- **Crate usado:** [`faux`](https://crates.io/crates/faux)
    
- Permite crear objetos fake (`.faux()`), interceptar llamadas y definir valores de retorno con `faux::when!(...)`.
    

📌 Ejemplo de uso:

```rust
#[faux::create]
struct Calculadora {
    id: u8
}

#[faux::methods]
impl Calculadora {
    pub fn calcular_anio_nacimiento(&self, per: &Persona) -> u32 {
        2023 - per.edad as u32
    }
}
```

---

### 🧹 **Linters**

- **Lint:** Regla de estilo o seguridad.
    
- **Linter:** Herramienta que aplica esas reglas.
    
- Herramienta principal en Rust: `clippy`
    
    - Instalación: `rustup component add clippy`
        
    - Uso: `cargo clippy`
        

---

### 🧩 **Pending Issues**

- **Visibilidad (`pub`)**:
    
    - `pub`: público
        
    - default: privado
        

📌 Ejemplo de módulos:

```rust
mod interno {
    pub fn funcion_publica() {}
    fn funcion_privada() {}
}

pub mod externo {
    pub fn acceso_publico() {}
}
```

---

### 🔄 **Concurrencia**

#### 🧵 Threads

- `std::thread::spawn(...)` → crea hilos.
    
- `join()` → espera que el hilo termine.
    

#### 📤 Compartir datos entre hilos

- `Arc<Mutex<T>>` → acceso concurrente con sincronización segura.
    
- `mpsc::channel()` → comunicación entre hilos (multi-producer, single-consumer).
    

#### ⚙️ Async

- Asincronía sin bloquear el hilo principal.
    
- Runtimes: `tokio`, `async-std`, `smol`.
    

📌 Ejemplo con `tokio`:

```rust
#[tokio::main]
async fn main() {
    let r1 = tarea(1);
    let r2 = tarea(2);
    tokio::join!(r1, r2);
}

async fn tarea(id: u8) {
    println!("Tarea {} terminada", id);
}
```

---

### ⚙️ **Características Avanzadas del Lenguaje**

#### 🧬 `dyn Trait`

- `Box<dyn Trait>` permite usar tipos heterogéneos que comparten un trait.
    
- Se resuelve en tiempo de ejecución.
    

#### 🧨 `unsafe`

- Permite realizar operaciones fuera de las garantías de seguridad de Rust.
    
- Útil para manipulación de punteros, llamadas a código externo, etc.
    

#### 🧠 Advanced Traits

- **Trait con tipo asociado:**
    

```rust
trait Iterator2 {
    type Item;
    fn next2(&mut self) -> Option<Self::Item>;
}
```

#### 📐 Advanced Types

- `type`: alias de tipo.
    

```rust
type Kilometros = i32;
```

#### 🧮 Advanced Functions & Closures

- Funciones como punteros:
    

```rust
fn sumar_uno(x: i32) -> i32 { x + 1 }
fn dos_veces(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```

---

### 📚 Recursos útiles

- [LogRocket sobre mocking](https://blog.logrocket.com/mocking-rust-mockall-alternatives/#mocking-in-unit-testing)
    
- [Rust Attributes](https://doc.rust-lang.org/reference/attributes.html)
    
- [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
    

---

¿Querés que te arme una guía de práctica o ejercicios para alguno de estos temas?