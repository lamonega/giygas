## 1. Introducción a Rust
Rust es un lenguaje de programación multiparadigma compilado de código abierto que se centra en tres pilares fundamentales: **seguridad, concurrencia y rendimiento**. Diseñado para ayudar a los desarrolladores a escribir código seguro y eficiente, Rust ofrece características únicas que garantizan un manejo de memoria seguro en tiempo de compilación sin necesidad de un recolector de basura.

### Características principales
- **Sistema de tipos estático y fuertemente tipado**: El tipo de cada variable debe ser conocido en tiempo de compilación y no puede cambiar durante la ejecución
- **Seguridad de memoria garantizada**: Mediante un sistema de ownership (propiedad) y borrowing (préstamos)
- **Rendimiento óptimo**: Compila directamente a código máquina nativo y optimizado
- **Mensajes de error descriptivos**: El compilador proporciona mensajes detallados que explican cómo subsanar errores
- **Gestión de errores explícita**: Utiliza el tipo Result para manejar errores de manera segura
- **Macros para metaprogramación**: Permite generación de código en tiempo de compilación
- **Cargo**: Herramienta integrada para construcción y administración de dependencias

### Instalación y primeros pasos
La instalación se realiza mediante rustup, y un programa básico en Rust tiene la siguiente estructura:
```rust
fn main() {
    println!("Seminario Rust 2024!");
}
```
Se compila con `rustc` o preferiblemente usando `cargo`, que facilita la gestión de proyectos.
## 2. Fundamentos del lenguaje
### Variables y mutabilidad
En Rust, las variables son **inmutables por defecto**, lo que obliga a ser explícito sobre qué datos pueden cambiar:
```rust
let numero = 5;        // inmutable
let mut numero = 5;    // mutable
```
Las **constantes** se declaran con `const` y requieren anotación de tipo explícita:
```rust
const MI_CONSTANTE: u8 = 10;
```
### Sistema de tipos
Rust divide sus tipos en dos categorías principales:
**Tipos escalares (Scalar types)**:
- **Enteros**: Con signo (i8, i16, i32, i64, i128, isize) y sin signo (u8, u16, u32, u64, u128, usize)
- **Punto flotante**: f32 y f64
- **Booleano**: bool
- **Carácter**: char (Unicode de 4 bytes)
**Tipos compuestos (Compound types)**:
- **String**: Cadena mutable de longitud variable
- **&str**: Cadena inmutable de longitud fija
- **Tupla**: Agrupa valores de distintos tipos con tamaño fijo
- **Array**: Colección de elementos del mismo tipo con tamaño fijo
### Estructuras de control
Rust proporciona las estructuras de control habituales con algunas particularidades:
- **if/else**: Puede usarse como expresión para asignar valores
- **match**: Pattern matching exhaustivo y poderoso
- **loop**: Bucle infinito que puede retornar valores
- **while**: Bucle condicional tradicional
- **for**: Iteración sobre rangos o iteradores
### Funciones
Las funciones se definen con `fn` y pueden recibir parámetros y retornar valores:
```rust
fn mi_funcion(data: i32) -> i32 {
    data * 2  // sin punto y coma para retornar
}```
## 3. Ownership, Borrowing y Lifetime

### Ownership (Propiedad)
El sistema de ownership es fundamental en Rust y se basa en tres reglas:
1. **Cada valor tiene un único dueño**
2. **Solo puede haber un dueño a la vez**
3. **Cuando el dueño sale del alcance, el valor se elimina**
Este sistema permite gestionar la memoria sin garbage collector. Los datos se almacenan en:
- **Stack**: Para tipos de tamaño conocido en tiempo de compilación (rápido)
- **Heap**: Para tipos de tamaño desconocido (más flexible pero costoso)
### Borrowing (Préstamos)
Rust permite prestar referencias a valores sin transferir la propiedad:
- **Referencias inmutables** (`&T`): Múltiples referencias permitidas
- **Referencias mutables** (`&mut T`): Solo una referencia a la vez
```rust
fn mi_funcion(data: &String) {  // préstamo inmutable
    println!("{}", data);
}

fn mi_funcion_mut(data: &mut String) {  // préstamo mutable
    data.push_str(" más texto");
}```
### Lifetime (Tiempo de vida)
Los lifetimes aseguran que las referencias sean válidas durante su uso. El compilador infiere la mayoría, pero a veces requiere anotaciones explícitas:
```rust
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}```
## 4. Testing

Rust integra un framework de testing robusto:
```rust
#[test]
fn test_ejemplo() {
    assert_eq!(2 + 2, 4);
}```
Macros de testing principales:
- `assert!`: Verifica que una expresión sea verdadera
- `assert_eq!`: Verifica igualdad
- `assert_ne!`: Verifica desigualdad
Atributos útiles:
- `#[ignore]`: Ignora el test
- `#[should_panic]`: Espera que el test entre en pánico
## 5. Tipos de datos complejos

### Structs
Permiten agrupar datos relacionados:
```rust
struct Persona {
    nombre: String,
    edad: u32,
}

impl Persona {
    fn new(nombre: String, edad: u32) -> Self {
        Persona { nombre, edad }
    }
    
    fn saludar(&self) {
        println!("Hola, soy {}", self.nombre);
    }
}```
### Enums
Definen tipos con variantes limitadas:
```rust
enum Estado {
    Activo,
    Inactivo,
    Pendiente(String),
}```
### Option
Enum especial para manejar valores opcionales, eliminando null pointer exceptions:
```rust
let valor: Option<i32> = Some(5);
let vacio: Option<i32> = None;

match valor {
    Some(v) => println!("Valor: {}", v),
    None => println!("Sin valor"),
}```
## 6. Collections

### Sequences
- **Vec**: Vector dinámico, la colección más utilizada
- **VecDeque**: Cola de doble extremo
- **LinkedList**: Lista enlazada (raramente necesaria)
### Maps
- **HashMap<K, V>**: Mapa hash para búsquedas rápidas
- **BTreeMap<K, V>**: Mapa ordenado por claves
### Sets
- **HashSet**: Conjunto sin duplicados
- **BTreeSet**: Conjunto ordenado
### Especiales
- **BinaryHeap**: Cola de prioridad (max-heap por defecto)

## 7. Generics
Permiten escribir código flexible y reutilizable:
```rust
struct Caja<T> {
    contenido: T,
}

impl<T> Caja<T> {
    fn new(contenido: T) -> Self {
        Caja { contenido }
    }
}```
## 8. Traits
Similar a interfaces en otros lenguajes, definen comportamiento compartido:
```rust
trait Animal {
    fn hablar(&self) -> String;
}

struct Perro;

impl Animal for Perro {
    fn hablar(&self) -> String {
        "Guau!".to_string()
    }
}```
Los traits pueden:
- Tener métodos con implementación por defecto
- Servir como límites en genéricos
- Usarse como parámetros de función
## 9. Programación Orientada a Objetos
Rust implementa conceptos de POO de manera particular:
- **Encapsulamiento**: ✅ Mediante visibilidad pública/privada
- **Abstracción**: ✅ Ocultando detalles de implementación
- **Polimorfismo**: ✅ A través de traits
- **Herencia**: 🤔 No tradicional, usa composición y traits
- **Modularidad**: ✅ Sistema de módulos robusto
## 10. Closures
Funciones anónimas que capturan su entorno:
```rust
// Definición de una closure simple
let suma = |x, y| x + y;
let resultado = suma(5, 3);

// Captura por referencia
let valor = 10;
let closure = || println!("Valor: {}", valor);

// Captura por valor con move
let closure_move = move || println!("Valor: {}", valor);```
## 11. Iterators
Patrón para recorrer colecciones de manera eficiente:
```rust
let numeros = vec![1, 2, 3, 4, 5];

let pares: Vec<_> = numeros.iter()
    .filter(|&x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

## 12. Manejo de errores
Rust clasifica errores en dos categorías:
### Errores irrecuperables: panic!
``` rust 
panic!("Error crítico!");
```
### Errores recuperables: Result<T, E>
```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("División por cero".to_string())
    } else {
        Ok(a / b)
    }
}```

El operador `?` propaga errores automáticamente:
```rust
fn operacion() -> Result<i32, Error> {
    let valor = funcion_que_puede_fallar()?;
    Ok(valor * 2)
}```
## 13. Archivos y Serde

### Manejo básico de archivos
```rust
use std::fs::File;
use std::io::prelude::*;

// Leer archivo
let mut file = File::open("archivo.txt")?;
let mut contenido = String::new();
file.read_to_string(&mut contenido)?;

// Escribir archivo
let mut file = File::create("nuevo.txt")?;
file.write_all(b"Contenido")?;
```
### Serialización con Serde

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Persona {
    nombre: String,
    edad: u32,
}

// Serializar a JSON
let json = serde_json::to_string(&persona)?;

// Deserializar desde JSON
let persona: Persona = serde_json::from_str(&json)?;
```

## 14. Testing avanzado y herramientas

### Mocking

Usando la librería `faux` para crear objetos simulados en tests:

```rust
#[faux::create]
struct Servicio {
    id: u8
}

#[faux::methods]
impl Servicio {
    fn obtener_dato(&self) -> i32 {
        // implementación real
        42
    }
}

#[test]
fn test_con_mock() {
    let mut servicio_mock = Servicio::faux();
    faux::when!(servicio_mock.obtener_dato()).then_return(100);
    
    assert_eq!(servicio_mock.obtener_dato(), 100);
}
```

### Coverage

La cobertura de código mide qué porcentaje del código es ejecutado durante las pruebas. Se utiliza `cargo-tarpaulin`:

```bash
cargo tarpaulin --out html
```

### Linters

**Clippy** es el linter oficial de Rust que detecta patrones problemáticos y sugiere mejoras:

```bash
cargo clippy
```

## 15. Smart Pointers

Los smart pointers son estructuras que actúan como referencias pero con capacidades adicionales:

### Box<>

Almacena datos en el heap, útil para:
- Tipos de tamaño desconocido en tiempo de compilación
- Tipos recursivos
- Transferir ownership de grandes cantidades de datos

```rust
enum Lista {
    Nodo(i32, Box<Lista>),
    Vacio,
}
```

### Rc<> (Reference Counted)

Permite múltiples propietarios del mismo dato:

```rust
use std::rc::Rc;

let dato = Rc::new(5);
let copia1 = Rc::clone(&dato);
let copia2 = Rc::clone(&dato);

println!("Referencias: {}", Rc::strong_count(&dato)); // 3
```

### RefCell<>

Implementa el patrón de "mutabilidad interior", permitiendo mutación con referencias inmutables:

```rust
use std::cell::RefCell;

let valor = RefCell::new(5);
*valor.borrow_mut() += 1;
println!("Valor: {}", *valor.borrow()); // 6
```

### Combinaciones comunes

- `Rc<RefCell<T>>`: Múltiples propietarios con mutabilidad interior
- `Box<dyn Trait>`: Para polimorfismo dinámico

## 16. Programación concurrente

### Threads

Rust garantiza seguridad en concurrencia mediante su sistema de tipos:

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hola desde un thread!");
});

handle.join().unwrap();
```

### Compartir datos entre threads

**Arc<Mutex<>>** permite compartir datos mutables de forma segura:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let contador = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let contador_clon = Arc::clone(&contador);
    let handle = thread::spawn(move || {
        let mut num = contador_clon.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

### Canales (Channels)

Para comunicación entre threads:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("Hola!").unwrap();
});

let mensaje = rx.recv().unwrap();
```

### Async/Await

Para concurrencia basada en tareas I/O intensivas:

```rust
// Con tokio
#[tokio::main]
async fn main() {
    let resultado = operacion_asincrona().await;
}

async fn operacion_asincrona() -> i32 {
    // simulación de operación I/O
    42
}
```

**Cuándo usar cada enfoque**:
- **Threads**: Para tareas CPU intensivas
- **Async**: Para tareas I/O intensivas (red, archivos, bases de datos)

## 17. Características avanzadas

### dyn (Dynamic dispatch)

Para polimorfismo dinámico cuando el tipo concreto no se conoce en tiempo de compilación:

```rust
trait Dibujable {
    fn dibujar(&self);
}

fn dibujar_todo(elementos: Vec<Box<dyn Dibujable>>) {
    for elemento in elementos {
        elemento.dibujar();
    }
}
```

### Type aliases

Simplificar tipos complejos:

```rust
type Resultado<T> = std::result::Result<T, MiError>;
type Coordenadas = (f64, f64);
```

### Funciones como valores

```rust
fn aplicar_operacion(f: fn(i32) -> i32, valor: i32) -> i32 {
    f(valor)
}

fn duplicar(x: i32) -> i32 {
    x * 2
}

let resultado = aplicar_operacion(duplicar, 5); // 10
```

### Macros

Metaprogramación para generar código:

```rust
macro_rules! crear_funcion {
    ($nombre:ident) => {
        fn $nombre() {
            println!("Función {} llamada", stringify!($nombre));
        }
    };
}

crear_funcion!(saludar);
saludar(); // "Función saludar llamada"
```

## 18. Modularización y visibilidad

### Sistema de módulos

```rust
mod mi_modulo {
    pub fn funcion_publica() {}
    
    fn funcion_privada() {}
    
    pub mod submodulo {
        pub fn otra_funcion() {}
    }
}

use mi_modulo::submodulo::otra_funcion;
```

### Creación de crates

- **Binary crate**: Ejecutable con `main.rs`
- **Library crate**: Biblioteca reutilizable con `lib.rs`

### Documentación

```rust
/// Calcula el factorial de un número
/// 
/// # Ejemplos
/// 
/// ```
/// assert_eq!(factorial(5), 120);
/// ```
pub fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}
```

Generar documentación: `cargo doc --open`

## 19. Blockchain con Rust

### Conceptos fundamentales

**Blockchain**: Estructura de datos donde los bloques se enlazan secuencialmente mediante hash criptográfico.

**Componentes principales**:
- **Bloque**: Hash, timestamp, referencia al anterior, transacciones
- **Transacción**: Hash, remitente, receptor, valor, firma digital
- **Nodo**: Dispositivo que mantiene copia de la blockchain y valida transacciones

### Algoritmos de consenso

- **Proof of Work (PoW)**: Minería mediante resolución de problemas criptográficos
- **Proof of Stake (PoS)**: Validación basada en cantidad de activos apostados

### Criptografía en blockchain

Jerarquía de claves:
1. **Frase semilla** → genera → **Clave privada** → deriva → **Clave pública**

### Smart Contracts con !ink

!ink es un SDK para desarrollar smart contracts en Rust para Substrate/Polkadot:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod mi_contrato {
    #[ink(storage)]
    pub struct MiContrato {
        valor: i32,
    }

    impl MiContrato {
        #[ink(constructor)]
        pub fn new(valor_inicial: i32) -> Self {
            Self { valor: valor_inicial }
        }

        #[ink(message)]
        pub fn obtener(&self) -> i32 {
            self.valor
        }

        #[ink(message)]
        pub fn incrementar(&mut self) {
            self.valor += 1;
        }
    }
}
```

Comandos básicos:
- `cargo contract new nombre_contrato`
- `cargo contract build`
- `cargo test`

### Evolución de blockchains

- **1era Gen (Bitcoin)**: Transferencias básicas, 1 bloque/10 min
- **2da Gen (Ethereum)**: Smart contracts, 1 bloque/15 seg
- **3era Gen (Polkadot)**: Multichain, parachains, interoperabilidad

## 20. Mejores prácticas y consejos finales

### Principios de diseño en Rust

1. **Preferir inmutabilidad**: Usar `mut` solo cuando sea necesario
2. **Ownership claro**: Diseñar APIs que hagan evidente quién posee qué
3. **Manejo de errores explícito**: Usar `Result` en lugar de `panic!`
4. **Zero-cost abstractions**: Las abstracciones no deben tener costo en runtime
5. **Expresividad**: Aprovechar el sistema de tipos para hacer código autodocumentado

### Patrones comunes

- **Builder pattern**: Para construcción compleja de objetos
- **Type state pattern**: Usar el sistema de tipos para representar estados
- **Interior mutability**: `RefCell` cuando se necesita mutabilidad con referencias inmutables
- **RAII**: Los recursos se liberan automáticamente cuando salen de scope

### Herramientas del ecosistema

- **rustfmt**: Formateo automático del código
- **clippy**: Linter con sugerencias de mejora
- **cargo-edit**: Gestión simplificada de dependencias
- **cargo-watch**: Recompilación automática al detectar cambios
- **rust-analyzer**: LSP para IDEs

### Recursos para continuar aprendiendo

- Documentación oficial: https://doc.rust-lang.org
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rustlings: Ejercicios interactivos
- Crates.io: Repositorio de paquetes
