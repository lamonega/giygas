pub struct Fecha {
    pub dia: u32,
    pub mes: u32,
    pub anio: u32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, anio: u32) -> Fecha {
        Fecha { dia, mes, anio }
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}-{}", self.dia, self.mes, self.anio)
    }

    pub fn es_bisiesto(&self) -> bool {
        (self.anio % 4 == 0) && (self.anio % 100 != 0 || self.anio % 400 == 0)
    }

    pub fn es_fecha_valida(&self) -> bool {
        if self.mes == 0 || self.mes > 12 || self.dia == 0 || self.dia > 31 {
            return false;
        }

        let dias_mes = match self.mes {
            2 => {
                if self.es_bisiesto() {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };

        self.dia <= dias_mes
    }

    pub fn sumar_dias(&mut self, dias: u32) {
        self.dia += dias;

        loop {
            let dias_mes_actual = match self.mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => {
                    if self.es_bisiesto() {
                        29
                    } else {
                        28
                    }
                }
                _ => 0, // mes inválido, pero debería no ocurrir
            };

            if self.dia > dias_mes_actual {
                self.dia -= dias_mes_actual;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.anio += 1;
                }
            } else {
                break;
            }
        }
    }

    pub fn restar_dias(&mut self, dias: u32) {
        let mut aux = self.dia as i32 - dias as i32;

        while aux <= 0 {
            self.mes -= 1;
            if self.mes == 0 {
                self.mes = 12;
                self.anio -= 1;
            }

            let dias_mes_anterior = match self.mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => {
                    if self.es_bisiesto() {
                        29
                    } else {
                        28
                    }
                }
                _ => 0,
            };

            aux += dias_mes_anterior as i32;
        }

        self.dia = aux as u32;
    }

    pub fn es_mayor(&self, fecha: Fecha) -> bool {
        if self.anio > fecha.anio {
            true
        } else if self.anio == fecha.anio && self.mes > fecha.mes {
            true
        } else if self.anio == fecha.anio && self.mes == fecha.mes && self.dia > fecha.dia {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_bisiesto() {
        let f1 = Fecha::new(1, 1, 2020);
        assert!(f1.es_bisiesto());

        let f2 = Fecha::new(1, 1, 1900);
        assert!(!f2.es_bisiesto());

        let f3 = Fecha::new(1, 1, 2000);
        assert!(f3.es_bisiesto());
    }

    #[test]
    fn test_es_fecha_valida() {
        let f1 = Fecha::new(29, 2, 2020); // bisiesto
        assert!(f1.es_fecha_valida());

        let f2 = Fecha::new(29, 2, 2019); // no bisiesto
        assert!(!f2.es_fecha_valida());

        let f3 = Fecha::new(31, 4, 2021); // abril tiene 30 días
        assert!(!f3.es_fecha_valida());

        let f4 = Fecha::new(31, 12, 2021);
        assert!(f4.es_fecha_valida());
    }

    #[test]
    fn test_sumar_dias() {
        let mut f = Fecha::new(28, 2, 2020);
        f.sumar_dias(1);
        assert_eq!(f.to_string(), "29-2-2020"); // bisiesto

        f.sumar_dias(1);
        assert_eq!(f.to_string(), "1-3-2020");

        let mut f2 = Fecha::new(31, 12, 2020);
        f2.sumar_dias(1);
        assert_eq!(f2.to_string(), "1-1-2021");

        let mut f3 = Fecha::new(30, 4, 2021);
        f3.sumar_dias(3);
        assert_eq!(f3.to_string(), "3-5-2021");
    }

    #[test]
    fn test_restar_dias() {
        let mut f = Fecha::new(1, 3, 2020);
        f.restar_dias(1);
        assert_eq!(f.to_string(), "29-2-2020");

        f.restar_dias(29);
        assert_eq!(f.to_string(), "31-1-2020");

        let mut f2 = Fecha::new(1, 1, 2021);
        f2.restar_dias(1);
        assert_eq!(f2.to_string(), "31-12-2020");
    }

    #[test]
    fn test_es_mayor() {
        let f1 = Fecha::new(2, 5, 2021);
        let f2 = Fecha::new(1, 5, 2021);
        assert!(f1.es_mayor(f2));

        let f3 = Fecha::new(1, 6, 2021);
        assert!(f3.es_mayor(f2));

        let f4 = Fecha::new(1, 5, 2022);
        assert!(f4.es_mayor(f3));

        let f5 = Fecha::new(1, 5, 2021);
        assert!(!f5.es_mayor(f4));
    }
}