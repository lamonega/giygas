pub struct Producto {
    id: u32,
    nombre: String,
    precio_bruto: f32,
}

impl Producto {
    pub fn new(id: u32, nombre: String, precio_bruto: f32) -> Self {
        Producto {
            id,
            nombre,
            precio_bruto,
        }
    }

    pub fn calcular_impuestos(&self, porcentaje_impuestos: f32) -> f32 {
        (self.precio_bruto * porcentaje_impuestos) / 100.0
    }

    pub fn aplicar_descuento(&self, porcentaje_descuento: f32) -> f32 {
        (self.precio_bruto * porcentaje_descuento) / 100.0
    }

    pub fn calcular_precio_total(&self, porcentaje_impuestos: Option<f32>, porcentaje_descuento: Option<f32>) -> f32 {
        let mut precio = self.precio_bruto;

        if let Some(impuestos) = porcentaje_impuestos {
            precio += self.calcular_impuestos(impuestos);
        }

        if let Some(descuento) = porcentaje_descuento {
            precio -= self.aplicar_descuento(descuento);
        }

        precio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_producto() {
        let p = Producto::new(1, "Manzana".to_string(), 100.0);
        assert_eq!(p.id, 1);
        assert_eq!(p.nombre, "Manzana");
        assert_eq!(p.precio_bruto, 100.0);
    }

    #[test]
    fn test_calcular_impuestos() {
        let p = Producto::new(1, "Manzana".to_string(), 100.0);
        assert_eq!(p.calcular_impuestos(10.0), 10.0);
    }

    #[test]
    fn test_calcular_precio_total_con_impuestos_y_descuento() {
        let p = Producto::new(1, "Manzana".to_string(), 100.0);
        // Precio total = 100 + 10% impuestos - 5% descuento = 100 + 10 - 5 = 105
        let total = p.calcular_precio_total(Some(10.0), Some(5.0));
        assert!((total - 105.0).abs() < f32::EPSILON);
    }
}
