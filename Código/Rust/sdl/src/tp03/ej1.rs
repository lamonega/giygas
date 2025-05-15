pub struct Persona {
    nombre: String,
    edad: u32,
    direccion: Option<String>,
}

impl Persona {
    pub fn new(nombre: String, edad: u32, direccion: Option<String>) -> Self {
        Persona {
            nombre,
            edad,
            direccion,
        }
    }
    
    pub fn to_string(&self) -> String {
        let dir = match &self.direccion {
            Some(d) => d.clone(),
            None => String::from("Sin dirección"),
        };
        format!("Nombre: {}, Edad: {}, Dirección: {}", self.nombre, self.edad, dir)
    }
    
    pub fn obtener_edad(&self) -> u32 {
        self.edad
    }

    pub fn actualizar_direccion(&mut self, nueva_direccion: String) {
        self.direccion = Some(nueva_direccion);
    }

    pub fn es_igual_a(&self, otra: &Persona) -> bool {
        self.nombre == otra.nombre &&
        self.edad == otra.edad &&
        self.direccion == otra.direccion
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creacion_persona_sin_direccion() {
        let persona = Persona::new(String::from("Lucía"), 30, None);
        assert_eq!(persona.nombre, "Lucía");
        assert_eq!(persona.edad, 30);
        assert_eq!(persona.direccion, None);
    }

    #[test]
    fn test_to_string_con_y_sin_direccion() {
        let persona_con_direccion = Persona::new(
            String::from("Juan"),
            25,
            Some(String::from("Calle Falsa 123")),
        );
        assert_eq!(
            persona_con_direccion.to_string(),
            "Nombre: Juan, Edad: 25, Dirección: Calle Falsa 123"
        );

        let persona_sin_direccion = Persona::new(String::from("Ana"), 40, None);
        assert_eq!(
            persona_sin_direccion.to_string(),
            "Nombre: Ana, Edad: 40, Dirección: Sin dirección"
        );
    }

    #[test]
    fn test_actualizar_direccion() {
        let mut persona = Persona::new(String::from("Carlos"), 50, None);
        persona.actualizar_direccion(String::from("Av. Siempre Viva 742"));
        assert_eq!(
            persona.direccion,
            Some(String::from("Av. Siempre Viva 742"))
        );
    }

    #[test]
    fn test_comparacion_manual() {
        let p1 = Persona {
            nombre: "Ana".to_string(),
            edad: 30,
            direccion: Some("Calle Falsa 123".to_string()),
        };

        let p2 = Persona {
            nombre: "Ana".to_string(),
            edad: 30,
            direccion: Some("Calle Falsa 123".to_string()),
        };

        assert!(p1.es_igual_a(&p2));
    }

}