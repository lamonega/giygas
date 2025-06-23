pub struct Informe { nombre: String, id: u32, promedio: f32, examen_nota_max: Examen, examen_nota_min: Examen }
// Después releyendo la consigna me dí cuenta que mi estrategia para resolver planteaba un struct que no
// contaba la cantidad de examenes del alumno
// Decidí no agregarlo en la v2 porque como te mandé el detalle del struct en la v1 y nos dijiste que tenían que ser
// congruentes una con la otra, decidí dejarlo como estaba
impl Informe { 
    pub fn new(nombre: String, id: u32, promedio: f32, examen_nota_max: Examen, examen_nota_min: Examen) -> Self { 
        Informe { nombre, id, promedio, examen_nota_max, examen_nota_min } 
    } 
}

#[derive(Clone)] // Esto no estaba implementado antes pero lo agrego ahora para clonar un examen y mandarlo para crear un informe
pub struct Examen { materia: String, nota: f32, }
impl Examen { 
    pub fn new(materia: String, nota: f32) -> Self { 
        Examen { materia, nota } 
        // Cuando hice el ejercicio implementé unas funciones getter para nota
        // Pero cuando te la mandé parece que lo borré y no me dí cuenta
        // Así que todas las líneas en las que accedo a la nota de un examen fueron editadas para acceder directamente
        // en vez de usar el getter
    } 
}

pub struct Estudiante { nombre: String, id: u32, examenes: Vec<Examen> }

impl Estudiante {
    pub fn new(nombre: String, id: u32, examenes: Vec<Examen>) -> Self { 
        Estudiante { nombre, id, examenes } 
    }

    pub fn obtener_promedio(&self) -> Option<f32> { 
        if self.examenes.is_empty() { return None }
        let mut suma = 0.0;
        for examen in &self.examenes { suma += examen.nota }
        Some(suma / self.examenes.len() as f32)
    }

    pub fn obtener_calificacion_mas_alta(&self) -> Option<f32> {
        if self.examenes.is_empty() { return None }
        let mut max = self.examenes[0].nota;
        for examen in &self.examenes { if examen.nota > max { max = examen.nota } }
        Some(max)
    }

    pub fn obtener_calificacion_mas_baja(&self) -> Option<f32> {
        if self.examenes.is_empty() { return None }
        let mut min = self.examenes[0].nota;
        for examen in &self.examenes { if examen.nota < min { min = examen.nota } }
        Some(min)
    }

    pub fn generar_informe(&self) -> Option<Informe> { 
        if self.examenes.is_empty() { return None; } 
        let mut max = &self.examenes[0]; 
        let mut min = &self.examenes[0]; 
        for examen in &self.examenes { 
            if examen.nota > max.nota { max = examen } 
            if examen.nota < min.nota { min = examen; } 
        } 
        Some(Informe::new(self.nombre.clone(), self.id, self.obtener_promedio().unwrap(), max.clone(), min.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_examen() {
        let examen = Examen::new("Matemáticas".to_string(), 8.5);
        assert_eq!(examen.materia, "Matemáticas");
        assert_eq!(examen.nota, 8.5);
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
// Dos test para probar los informes
    #[test] 
    fn test_generar_informe() { 
        let e = vec![Examen::new("Arte".to_string(), 4.0), Examen::new("Música".to_string(), 9.5), Examen::new("Deportes".to_string(), 7.0)]; 
        let est = Estudiante::new("Carlos".to_string(), 111, e); 
        let inf = est.generar_informe().unwrap(); 
        assert_eq!(inf.nombre, "Carlos"); 
        assert_eq!(inf.id, 111); 
        assert_eq!(inf.promedio, 6.833333); 
        assert_eq!(inf.examen_nota_max.materia, "Música"); 
        assert_eq!(inf.examen_nota_max.nota, 9.5); 
        assert_eq!(inf.examen_nota_min.materia, "Arte"); 
        assert_eq!(inf.examen_nota_min.nota, 4.0); 
    }

    #[test]
    fn test_generar_informe_de_alumno_sin_examenes() {
        let examenes_vacios: Vec<Examen> = vec![];
        let est = Estudiante::new("María".to_string(), 444, examenes_vacios);
        let informe = est.generar_informe();
        assert_eq!(informe, None);
    }
}