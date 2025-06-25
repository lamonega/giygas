/*

2- Dado el siguiente struct:

struct Persona<'a>{
nombre:&'a str,
apellido:&'a str,
direccion:&'a str,
ciudad:&'a str,
salario:f64,
edad:u8,
}

a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
ciudad.
c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
contrario.
d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso
contrario.
e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
persona existe en el arreglo, false caso contrario
f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
edades de las personas.
g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
categoría desempatar por la edad más grande.

Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
Todos los ejercicios deben resolverse con iterator y closure.

*/

#[derive(PartialEq, Clone)]
struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

fn salario_mayor(personas: &[Persona], salario: f64) -> Vec<Persona> {
    // Filtra personas cuyo salario es mayor que el parámetro,
    // luego clona y recolecta desde el iterador resultante para devolver valores
    personas
        .iter()
        .filter(|p| p.salario > salario)
        .cloned()
        .collect()
}

fn mayores_que_y_oriundos_de(personas: &[Persona], ciudad: &str, edad: u8) -> Vec<Persona> {
    // Filtra personas mayores a la edad y que viven en la ciudad dada
    // Luego clona y recolecta
    personas
        .iter()
        .filter(|p| p.ciudad == ciudad && p.edad > edad)
        .cloned()
        .collect()
}

fn todos_oriundos(personas: &[Persona], ciudad: &str) -> bool {
    // Verifica que todas las personas vivan en la ciudad dada
    personas.iter().all(|p| p.ciudad == ciudad)
}

fn al_menos_un_oriundo(personas: &[Persona], ciudad: &str) -> bool {
    // Verifica que al menos una persona viva en la ciudad dada
    personas.iter().any(|p| p.ciudad == ciudad)
}

fn existe_persona(personas: &[Persona], persona: &Persona) -> bool {
    // Busca si la persona existe en el arreglo (usa PartialEq)
    personas.iter().any(|p| p == persona)
}

fn edades_de_personas(personas: &[Persona]) -> Vec<u8> {
    // Obtiene las edades de todas las personas en un nuevo vector
    personas.iter().map(|p| p.edad).collect()
}

fn extremos_salario<'a>(personas: &'a [Persona]) -> Option<(&'a Persona, &'a Persona)> {
    if personas.is_empty() {
        return None;
    }

    // fold acumula la persona con el menor salario y la persona con el mayor salario,
    // usando la edad para desempatar cuando los salarios son iguales.

    // Explicación paso a paso:

    // Se declara una tupla `(menor, mayor)` que almacena referencias a las personas con el salario mínimo y máximo respectivamente.

    // El método `fold` se usa para recorrer el iterador de personas y acumular ambos valores en una sola pasada.

    // La firma de `fold` es:
    // iter.fold(valor_inicial, |acumulador, elemento| {
    //     // lógica para actualizar el acumulador
    //     // retorna el nuevo acumulador
    // });

    // Parámetros:
    // - valor_inicial: es el estado inicial del acumulador, aquí la tupla (&personas[0], &personas[0]).
    // - closure: recibe el acumulador actual (una tupla con menor y mayor persona hasta ahora) y el elemento actual (una persona),
    //   y debe devolver el nuevo acumulador actualizado.

    // En cada iteración, el closure compara el salario (y en caso de empate, la edad) de la persona actual
    // con los valores almacenados en el acumulador, y actualiza la tupla para mantener siempre las personas con salario menor y mayor.

    // Finalmente, `fold` retorna la tupla con las personas que cumplen esas condiciones luego de recorrer todo el arreglo.

    let (menor, mayor) = personas.iter().fold(
        (&personas[0], &personas[0]),
        |(min, max), persona| {
            let nuevo_min = if persona.salario < min.salario
                || (persona.salario == min.salario && persona.edad > min.edad)
            {
                persona
            } else {
                min
            };

            let nuevo_max = if persona.salario > max.salario
                || (persona.salario == max.salario && persona.edad > max.edad)
            {
                persona
            } else {
                max
            };

            (nuevo_min, nuevo_max)
        },
    );

    Some((menor, mayor))
}
