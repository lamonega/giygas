### 🧩 Structs

#### ¿Qué es un `struct`?

- Tipo de dato personalizado para agrupar valores relacionados.
    
- Similar a los atributos de una clase en OOP.
    

#### 🧾 Definición básica

```rust
struct Persona {
    nombre: String,
    apellido: String,
    dni: i32,
}
```

#### 🧪 Instanciar y usar

```rust
let persona1 = Persona {
    nombre: "Lionel".to_string(),
    apellido: "Messi".to_string(),
    dni: 1,
};
println!("{}", persona1.nombre);
```

#### 🔁 Shorthand en inicialización

```rust
fn nueva_persona(nombre: String, apellido: String, dni: i32) -> Persona {
    Persona { nombre, apellido, dni }
}
```

#### 🛠️ Modificar instancia

```rust
let mut persona = nueva_persona(...);
persona.dni = 99;
```

#### 🧬 Reutilizar datos con `..`

```rust
let persona2 = Persona {
    nombre: "Thiago".to_string(),
    ..persona1
};
```

#### 📦 Tuple Struct

```rust
struct Coordenada(f64, f64);
let c = Coordenada(-34.92, -57.95);
println!("{}", c.0);
```

#### 🧠 Métodos con `impl`

```rust
impl Coordenada {
    fn es_la_plata(self) -> bool {
        self.0 == -34.9213094 && self.1 == -57.9555699
    }
}
```

#### 📏 Ejemplo método `area`

```rust
struct Rectangulo { ancho: u32, altura: u32 }

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.altura
    }

    fn new(ancho: u32, altura: u32) -> Self {
        Self { ancho, altura }
    }
}
```

#### 📦 Derive `Debug`

```rust
#[derive(Debug)]
// permite imprimir el struct con `{:?}`
```

---

### 🧭 Enums

#### 🔢 Definición

```rust
enum Rol {
    PADRE,
    HIJO,
}
```

#### 🧪 Con Struct y valores

```rust
enum Rol {
    PADRE(i32),
    HIJO(i32),
}

match per1.rol {
    Rol::PADRE(valor) => println!("{}", valor),
    _ => (),
}
```

#### 🧱 Enums con structs internos

```rust
struct StructPadre {}
struct StructHijo {}

enum Rol {
    PADRE(StructPadre),
    HIJO(StructHijo),
}

impl Rol {
    fn hace_algo(self) {
        match self {
            Rol::PADRE(p) => p.hace_algo(),
            Rol::HIJO(h) => h.hace_algo(),
        }
    }
}
```

---

### ❔ Option

#### ❓ ¿Qué es?

- Enum con variantes: `Some(valor)` y `None`.
- Ayuda a evitar errores de punteros nulos.
#### 🧪 Ejemplo básico

```rust
struct Persona {
    nombre: String,
    dni: Option<i32>,
}
```

#### 🔎 Acceso con `match`

```rust
match persona.dni {
    Some(n) => println!("{}", n),
    None => println!("sin dni"),
}
```

#### 🔍 Con métodos

- `.is_none()`
- `.unwrap()`
- `if let Some(x) = val`
- `let Some(x) = val else { ... };`
- `while let Some(x) = val { ... }`
#### 🧱 Option con struct

```rust
struct DNI { tipo: char, nro: u32 }
dni: Option<DNI>
```

---
### 📚 Collections (Parte 1)

#### 📦 Vec
- Vector dinámico, crece automáticamente.
```rust
let mut vec = Vec::new();
vec.push(1);
```
#### 🔍 Acceso
- `first()` / `last()`
- Índice: `vec[0]`
#### 🔁 Modificar
```rust
for i in 0..vec.len() {
    vec[i] += 4;
}
```
#### ❌ Eliminar
```rust
vec.remove(1);
```
#### 🥞 Simular pila
```rust
while let Some(x) = vec.pop() {
    println!("{x}");
}
```
#### 🧪 Inicialización rápida
```rust
let vec = vec![1, 2, 3];
let vec = vec![0; 5];
```

---

### 🔁 VecDeque

- Cola de doble extremo
    

```rust
let mut buf = VecDeque::new();
buf.push_back(1);
buf.push_front(2);
```

#### 🔍 Acceso

- Por índice o `get()`
    

#### 🛠️ Modificación

- `get_mut(i)` para acceso mutable
    

---

### 🔗 LinkedList

- Lista doblemente enlazada
    

```rust
let mut list = LinkedList::new();
list.push_back(1);
list.push_front(2);
```

#### 🔧 Métodos útiles

- `front()`, `back()`
    
- `contains()`, `len()`, `clear()`, `clone()`
    

---

### 🔑 HashMap

```rust
let mut map = HashMap::new();
map.insert(1, 10.0);
```

#### 🔍 Acceso y modificación

```rust
if let Some(x) = map.get_mut(&id) {
    *x += 1.0;
}
```

#### 🗝️ Métodos útiles

- `get_key_value()`, `remove()`, `contains_key()`, `entry().or_insert()`
    

---

### 🌲 BTreeMap

- Igual que HashMap, pero con orden por clave (árbol binario)
    

```rust
let mut mapa = BTreeMap::new();
mapa.insert("a", 1);
```