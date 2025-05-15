pub struct Triangulo {
    l1: f64,
    l2: f64,
    l3: f64,
}

impl Triangulo {
    pub fn new(l1: f64, l2: f64, l3: f64) -> Result<Self, String> {
        if l1 + l2 > l3 && l1 + l3 > l2 && l2 + l3 > l1 {
            Ok(Triangulo { l1, l2, l3 })
        } else {
            Err("Los lados no forman un triángulo válido".to_string())
        }
    }


    pub fn determinar_tipo(&self) -> String {
        if (self.l1 == self.l2) && (self.l2 == self.l3) {
            "Equilátero".to_string()
        } else if (self.l1 == self.l2) || (self.l1 == self.l3) || (self.l2 == self.l3) {
            "Isósceles".to_string()
        } else {
            "Escaleno".to_string()
        }
    }

    pub fn calcular_area(&self) -> f64 {
        let s = (self.l1 + self.l2 + self.l3) / 2.0;
        let area_sq = s * (s - self.l1) * (s - self.l2) * (s - self.l3);
        if area_sq < 0.0 {
            0.0
        } else {
            area_sq.sqrt()
        }
    }

    pub fn calcular_perimetro(&self) -> f64 {
        self.l1 + self.l2 + self.l3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_triangulo_valido() {
        let t = Triangulo::new(3.0, 4.0, 5.0);
        assert!(t.is_ok());
    }

    #[test]
    fn test_new_triangulo_invalido() {
        let t = Triangulo::new(1.0, 2.0, 3.5);
        assert!(t.is_err());
        assert_eq!(t.unwrap_err(), "Los lados no forman un triángulo válido");
    }

    #[test]
    fn test_determinar_tipo_equilatero() {
        let t = Triangulo::new(5.0, 5.0, 5.0).unwrap();
        assert_eq!(t.determinar_tipo(), "Equilátero");
    }

    #[test]
    fn test_determinar_tipo_isosceles() {
        let t = Triangulo::new(5.0, 5.0, 3.0).unwrap();
        assert_eq!(t.determinar_tipo(), "Isósceles");
    }

    #[test]
    fn test_determinar_tipo_escaleno() {
        let t = Triangulo::new(3.0, 4.0, 5.0).unwrap();
        assert_eq!(t.determinar_tipo(), "Escaleno");
    }

    #[test]
    fn test_calcular_area() {
        let t = Triangulo::new(3.0, 4.0, 5.0).unwrap();
        let area = t.calcular_area();
        // Área de triángulo 3-4-5 es 6.0
        assert!((area - 6.0).abs() < 1e-6);
    }

    #[test]
    fn test_calcular_perimetro() {
        let t = Triangulo::new(3.0, 4.0, 5.0).unwrap();
        assert_eq!(t.calcular_perimetro(), 12.0);
    }
}
