/* 
ESTRATEGIA PARA ENTREGABLE 1:
- Implementar struct Informe que contendrá
    1) nombre: String,
    2) id: u32,
    3) promedio: f32,
    4) examen_nota_max: Examen,
    5) examen_nota_min: Examen
- Implementar para Informe función new que cree un Informe con los campos pasados como parámetro y lo retorne (Igual a las funciones new que venimos haciendo para la práctica)

- Agregar a la implementación de Estudiante una funcion generar_informe que recibe una referencia INMUTABLE a sí mismo y retorne un Option<Informe>:
    - Evalúa si el Estudiante tiene exámenes. Si no los tiene, retorna None
    - Si los tiene:
        - Recorre el vector de exámenes para localizar los exámenes con la máxima y mínima nota, almacenando todo en variables locales.
        - Llama a la función obtener promedio y almacena su resultado en una variable local.
        - Crea un struct Informe con la función new() a la que le pasa como parámetros los datos identificatorios del estudiante (id, nombre) y los que recopiló recorriendo el vector
        - Retorna ese struct como Some<i> donde i es un Informe.
*/

pub struct Informe {
    nombre: String,
    id: u32,
    promedio: f32,
    examen_nota_max: Examen,
    examen_nota_min: Examen,
}

impl Informe {
    pub fn new(nombre: String, id: u32, promedio: f32, )
}


pub struct Examen {
    materia: String,
    nota: f32,
}
impl Examen {
    pub fn new(materia: String, nota: f32) -> Self {
        Examen { materia, nota }
    }
}
pub struct Estudiante {
    nombre: String,
    id: u32,
    examenes: Vec<Examen>,
}

impl Estudiante {
    pub fn new(nombre: String, id: u32, examenes: Vec<Examen>) -> Self {
        Estudiante {
            nombre,
            id,
            examenes,
        }
    }

    pub fn obtener_promedio(&self) -> Option<f32> {
        if self.examenes.is_empty() {
            return None;
        }

        let mut suma = 0.0;
        for examen in &self.examenes {
            suma += examen.nota();
        }

        Some(suma / self.examenes.len() as f32)
    }

    pub fn obtener_calificacion_mas_alta(&self) -> Option<f32> {
        if self.examenes.is_empty() {
            return None;
        }

        let mut max = self.examenes[0].nota();
        for examen in &self.examenes {
            if examen.nota() > max {
                max = examen.nota();
            }
        }
        Some(max)
    }

    pub fn obtener_calificacion_mas_baja(&self) -> Option<f32> {
        if self.examenes.is_empty() {
            return None;
        }

        let mut min = self.examenes[0].nota();
        for examen in &self.examenes {
            if examen.nota() < min {
                min = examen.nota();
            }
        }
        Some(min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_examen() {
        let examen = Examen::new("Matemáticas".to_string(), 8.5);
        assert_eq!(examen.materia, "Matemáticas");
        assert_eq!(examen.nota(), 8.5);
    }

    #[test]
    fn test_promedio_notas() {
        let examenes = vec![
            Examen::new("Matemáticas".to_string(), 8.0),
            Examen::new("Historia".to_string(), 6.0),
            Examen::new("Biología".to_string(), 10.0),
        ];
        let estudiante = Estudiante::new("Juan".to_string(), 123, examenes);
        let promedio = estudiante.obtener_promedio().unwrap();
        assert!((promedio - 8.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_nota_mas_alta() {
        let examenes = vec![
            Examen::new("Lengua".to_string(), 5.5),
            Examen::new("Geografía".to_string(), 7.0),
        ];
        let estudiante = Estudiante::new("Ana".to_string(), 456, examenes);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), Some(7.0));
    }

    #[test]
    fn test_nota_mas_baja() {
        let examenes = vec![
            Examen::new("Física".to_string(), 9.0),
            Examen::new("Química".to_string(), 6.5),
        ];
        let estudiante = Estudiante::new("Luis".to_string(), 789, examenes);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), Some(6.5));
    }

    #[test]
    fn test_estudiante_sin_examenes() {
        let estudiante = Estudiante::new("Sofía".to_string(), 321, vec![]);
        assert_eq!(estudiante.obtener_promedio(), None);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), None);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), None);
    }
}