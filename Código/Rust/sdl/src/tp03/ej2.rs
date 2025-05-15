pub struct Rectangulo {
    longitud: u32,
    ancho: u32
}

impl Rectangulo {
    pub fn new(longitud: u32, ancho: u32) -> Self {
        Rectangulo {
            longitud, 
            ancho
        }
    }

    pub fn calcular_area(&self) -> u32 {
        return self.longitud * self.ancho
    }

    pub fn calcular_perimetro(&self) -> u32 {
        return (self.longitud + self.ancho) * 2
    }

    pub fn es_cuadrado(&self) -> bool {
        return self.longitud == self.ancho
    }

    pub fn es_igual_a(&self, otra: &Rectangulo) -> bool {
        return self.longitud == otra.longitud && self.ancho == otra.ancho
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_y_perimetro() {
        let r = Rectangulo::new(5, 3);
        assert_eq!(r.calcular_area(), 15);
        assert_eq!(r.calcular_perimetro(), 16);
    }

    #[test]
    fn test_es_cuadrado() {
        let cuadrado = Rectangulo::new(4, 4);
        let rectangulo = Rectangulo::new(4, 2);
        assert!(cuadrado.es_cuadrado());
        assert!(!rectangulo.es_cuadrado());
    }

    #[test]
    fn test_comparacion_manual() {
        let r1 = Rectangulo::new(6, 2);
        let r2 = Rectangulo::new(6, 2);
        let r3 = Rectangulo::new(2, 6);
        assert!(r1.es_igual_a(&r2));
        assert!(!r1.es_igual_a(&r3));
    }
}