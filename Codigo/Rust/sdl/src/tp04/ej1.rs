/*
1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de
números primos. Cree un trait para la determinación del número primo e impleméntelo
según corresponda. Utilice la función iter sobre el vector y aplique un closure para
resolverlo.
*/

// Trait para verificar si un número es primo
trait EsPrimo {
    fn es_primo(&self) -> bool;
    // Función abstracta, requiere implementación específica para cada tipo que implemente el trait.
}

// Implementación del trait para i32
impl EsPrimo for i32 {
    fn es_primo(&self) -> bool {
        // Desreferenciamos self (que es una &i32) para obtener su valor
        if *self < 2 {
            return false; // Los números menores que 2 no son primos
        }
        // Iteramos desde 2 hasta la raíz cuadrada del número
        for i in 2..=(*self as f64).sqrt() as i32 {
            // Si es divisible por i, no es primo
            if self % i == 0 {
                return false;
            }
        }
        // Si no se encontró ningún divisor, es primo
        true
    }
}

// Función que cuenta los números primos en un vector
fn contar_primos(valores: Vec<i32>) -> u32 {
    valores
        .iter() // Se crea un iterador sobre referencias a los elementos del vector (&i32)
        .filter(|&x| x.es_primo()) // Se filtran los elementos que cumplen con la condición del closure
        // Closure: para cada referencia &x, se llama al método es_primo
        .count() // Cuenta cuántos elementos pasaron el filtro (i.e., cuántos son primos)
}
