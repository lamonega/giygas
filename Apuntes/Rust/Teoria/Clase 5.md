d## **Traits**

### ¿Qué es un Trait?
Un **trait** define un comportamiento que puede ser compartido entre tipos. Es similar a una interfaz en otros lenguajes.  
Se puede usar para:

- Declarar funciones abstractas.
- Proveer implementaciones por defecto.
- Restringir tipos genéricos.

### Ejemplo básico
```rust
pub trait Multiplica {
    fn mul(&self, otro: i32) -> f64;
    fn mensaje(&self) {
        println!("Método por defecto");
    }
}

impl Multiplica for f64 {
    fn mul(&self, otro: i32) -> f64 {
        self * otro as f64
    }
}

fn main() {
    let num = 2.5;
    println!("{}", num.mul(3));
}
```
### Trait como interfaz común
```rust
trait Animal {
    fn hablar(&self) -> String;
}

struct Perro;
struct Gato;

impl Animal for Perro {
    fn hablar(&self) -> String {
        "Guau!".to_string()
    }
}
impl Animal for Gato {
    fn hablar(&self) -> String {
        "Miau!".to_string()
    }
}
```
### Uso con genéricos
```rust
fn imprimir_hablar<T: Animal>(animal: &T) {
    println!("{}", animal.hablar());
}
```

### Parámetros con múltiples traits
```rust
fn imprimir_hablar<T>(animal: &T)
where T: Animal + OtroTrait
{
    println!("{}", animal.hablar());
}
```

---
## **Programación Orientada a Objetos (POO)**
### Elementos básicos
- **Clases**: Representadas por structs.
- **Atributos**: Campos de un struct.
- **Métodos**: Implementados con `impl`.
- **Objetos**: Instancias de structs.

### Conceptos fundamentales

- **Encapsulamiento**: Oculta detalles internos.
    
- **Abstracción**: Representa conceptos reales.
    
- **Polimorfismo**: Mismo mensaje, diferente comportamiento.
    
- **Herencia**: No está soportada como en otros lenguajes.
    
- **Modularidad**: Organización del código en módulos y crates.
    

### Encapsulamiento en Rust

```rust
pub struct Ejemplo {
    atr1: i32,
    atr2: i32,
}

impl Ejemplo {
    pub fn new(atr1: i32, atr2: i32) -> Self {
        Self { atr1, atr2 }
    }

    pub fn calcular(&self) -> i32 {
        self.atr1 * self.atr2
    }
}
```

### Polimorfismo mediante traits

```rust
trait PushBack<T> {
    fn push_back(&mut self, elem: T);
}

impl<T> PushBack<T> for VecDeque<T> {
    fn push_back(&mut self, elem: T) {
        self.push_back(elem);
    }
}
```

### Herencia mediante composición

Rust no permite herencia directa. Sin embargo, se puede reutilizar comportamiento con traits y composición:

```rust
struct DatosVehiculo {
    matricula: String,
    modelo: i32,
    potencia: i32,
}

trait Vehiculo {
    fn get_matricula(&self, datos: &DatosVehiculo) -> String {
        datos.matricula.clone()
    }
}

struct Taxi {
    datos: DatosVehiculo,
}

impl Vehiculo for Taxi {}
```

### Modularidad

Rust permite modularidad usando:
- Módulos (`mod`)
- Archivos separados
- Crates externos

---

## **Closures**

### ¿Qué son?
Son funciones anónimas que se pueden:
- Guardar en variables
- Pasar como argumentos
- Usar con acceso a variables del entorno
### Ejemplo básico

```rust
let suma = |x: i32, y: i32| x + y;
println!("{}", suma(2, 3));
```

### Inferencia de tipos
Rust puede deducir tipos automáticamente:

```rust
let suma = |x, y| x + y;
```

### Closure con estructuras

```rust
#[derive(Debug)]
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

fn main() {
    let mut items = [
        Rectangulo { ancho: 5, alto: 2 },
        Rectangulo { ancho: 3, alto: 8 },
    ];
    items.sort_by_key(|r| r.ancho);
}
```

### Uso de referencias y ownership

Closures pueden:
- Tomar prestado (`&`)
- Modificar (`&mut`)
- Tomar propiedad (`move`)

```rust
let lista = vec![1, 2, 3];
let cerrar = || println!("{:?}", lista); // toma prestado
cerrar();
```
