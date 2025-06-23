#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

impl Color {
    pub fn es_primario(&self) -> bool {
        matches!(self, Color::Rojo | Color::Amarillo | Color::Azul)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Auto {
    marca: String,
    modelo: String,
    anio: u16,
    precio_bruto: f64,
    color: Color,
}

impl Auto {
    pub fn new(marca: String, modelo: String, anio: u16, precio_bruto: f64, color: Color) -> Self {
        Auto {
            marca,
            modelo,
            anio,
            precio_bruto,
            color,
        }
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut precio = self.precio_bruto;

        if self.color.es_primario() {
            precio *= 1.25;
        } else {
            precio *= 0.90;
        }

        if self.marca.to_lowercase() == "bmw" {
            precio *= 1.15;
        }

        if self.anio < 2000 {
            precio *= 0.95;
        }

        precio
    }
}

pub struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad_maxima: u32,
    autos: Vec<Auto>,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad_maxima: u32) -> Self {
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad_maxima,
            autos: Vec::new(),
        }
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() as u32 >= self.capacidad_maxima {
            false
        } else {
            self.autos.push(auto);
            true
        }
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) -> bool {
        if let Some(pos) = self.autos.iter().position(|x| x == auto) {
            self.autos.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        self.autos.iter().find(|&x| x == auto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_precio_color_primario_bmw_antiguo() {
        let auto = Auto::new(
            "BMW".to_string(),
            "Serie 3".to_string(),
            1995,
            10000.0,
            Color::Rojo,
        );
        let precio_esperado = 13656.25;
        assert!((auto.calcular_precio() - precio_esperado).abs() < f64::EPSILON);
    }

    #[test]
    fn test_agregar_y_buscar_auto() {
        let mut concesionario = ConcesionarioAuto::new(
            "MiConcesionario".to_string(),
            "Calle Falsa 123".to_string(),
            2,
        );

        let auto = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2020,
            20000.0,
            Color::Blanco,
        );

        assert!(concesionario.agregar_auto(auto.clone()));
        assert!(concesionario.buscar_auto(&auto).is_some());
    }

    #[test]
    fn test_eliminar_auto() {
        let mut concesionario = ConcesionarioAuto::new(
            "El Garage".to_string(),
            "Av. Siempre Viva 742".to_string(),
            1,
        );

        let auto = Auto::new(
            "Ford".to_string(),
            "Focus".to_string(),
            2010,
            15000.0,
            Color::Negro,
        );

        concesionario.agregar_auto(auto.clone());
        assert!(concesionario.eliminar_auto(&auto));
        assert!(concesionario.buscar_auto(&auto).is_none());
    }

    #[test]
    fn test_agregar_auto_falla_por_capacidad() {
        let mut concesionario = ConcesionarioAuto::new(
            "Chico Cars".to_string(),
            "Ruta 1 km 5".to_string(),
            1,
        );

        let auto1 = Auto::new(
            "Honda".to_string(),
            "Civic".to_string(),
            2015,
            18000.0,
            Color::Azul,
        );

        let auto2 = Auto::new(
            "Mazda".to_string(),
            "3".to_string(),
            2016,
            19000.0,
            Color::Verde,
        );

        assert!(concesionario.agregar_auto(auto1));
        assert!(!concesionario.agregar_auto(auto2)); // Capacidad excedida
    }
}
