### 🧺 **Colecciones - Sets**

#### 🔹 `HashSet<T>`

- Es como un `HashMap`, pero **sin valores**, solo claves únicas.
    
- No garantiza orden.
    

```rust
use std::collections::HashSet;

fn main() {
    let mut ids = HashSet::from([1, 2, 3]);
    let mut otros_ids = HashSet::from([10, 2, 30]);
    
    ids.insert(4);

    // Operaciones de conjunto
    ids.difference(&otros_ids);
    ids.intersection(&otros_ids);
    ids.union(&otros_ids);

    // Otras utilidades
    ids.remove(&3);
    ids.len();
    ids.is_empty();
}
```

📖 Más info: [HashSet Docs](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html)

---

#### 🔹 `BTreeSet<T>`

- Similar a `HashSet`, pero mantiene los elementos **ordenados**.
    
- Internamente es un **árbol binario balanceado**.
    

```rust
use std::collections::BTreeSet;

fn main() {
    let mut ids = BTreeSet::from([1, 2, 3]);
    
    for id in ids {
        println!("{id}");
    }
}
```

📖 Más info: [BTreeSet Docs](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)

---

### ⛰️ **Extra: BinaryHeap**

#### 🔹 `BinaryHeap<T>`

- Es una **cola de prioridad**, por defecto tipo **max-heap** (mayor valor sale primero).
    

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut max_heap = BinaryHeap::from([1, 2, 3]);
    max_heap.push(4);

    println!("max_heap: {:?}", max_heap);

    if let Some(e) = max_heap.pop() {
        println!("{e}");
    }

    if let Some(e) = max_heap.peek() {
        println!("{e}");
    }

    println!("max_heap: {:?}", max_heap);
}
```

#### 🔹 ¿Y un Min-Heap?

- Se logra usando `Reverse`.
    

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(2));
    min_heap.push(Reverse(3));

    println!("min_heap: {:?}", min_heap);

    if let Some(e) = min_heap.pop() {
        println!("{}", e.0);
    }

    if let Some(e) = min_heap.peek() {
        println!("{}", e.0);
    }
}
```

📖 Más info: [BinaryHeap Docs](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)

---

### 🔍 **¿Cuándo usar cada colección?**

|Colección|Usar cuando...|
|---|---|
|`Vec`|Necesitás una lista dinámica, secuencial, tipo stack o para procesamiento futuro.|
|`VecDeque`|Necesitás insertar al inicio y al final eficientemente (cola doble).|
|`LinkedList`|Realmente necesitás una lista doblemente enlazada.|
|`HashMap`|Asociás claves arbitrarias a valores (como un diccionario/caché).|
|`BTreeMap`|Querés un map ordenado por claves, o acceder al mínimo/máximo.|
|`HashSet`/`BTreeSet`|Solo necesitás conjuntos con operaciones tipo `union`, `intersección`, etc.|
|`BinaryHeap`|Procesás elementos por prioridad, no en orden de inserción.|

---

## 🧬 **Genéricos en Rust**

### 🔸 ¿Qué son?

Permiten **parametrizar tipos**, haciendo el código **reutilizable y flexible**.

---

### 🔸 Ejemplo básico sin genéricos

```rust
#[derive(Debug)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };
    println!("{:?}", p1);
}
```

---

### 🔸 Con genéricos

```rust
struct Punto<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };         // Punto<i32>
    let p2 = Punto { x: 10.5, y: 2.0 };    // Punto<f64>
}
```

---

### 🔸 Genéricos con múltiples tipos

```rust
struct Punto<T, V> {
    x: T,
    y: V,
}
```

---

### 🔸 Genérico anidado

```rust
#[derive(Debug)]
struct Punto<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };
    let p2 = Punto { x: 10, y: 2 };
    let p_esp = Punto { x: p1, y: p2 };

    println!("{:?}", p_esp);
}
```

---

### 📦 **Ejemplo práctico con `Caja<T>`**

```rust
struct Caja<T> {
    dato: T,
    estado: bool,
}

impl<T> Caja<T> {
    fn new(dato: T) -> Caja<T> {
        Caja { dato, estado: false }
    }

    fn abrir(&mut self) -> &T {
        self.estado = true;
        &self.dato
    }

    fn cerrar(&mut self) {
        self.estado = false;
    }
}
```

#### 📦 Uso con `Vec` y tuplas

```rust
fn main() {
    let mut listado = Vec::new();
    listado.push((1, "Jabón de manos"));
    listado.push((2, "Detergente"));

    let mut caja = Caja::new(listado);
    caja.abrir();
    caja.cerrar();
}
```