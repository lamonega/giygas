pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

impl PartialEq for Genero {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Genero::Rock, Genero::Rock)
                | (Genero::Pop, Genero::Pop)
                | (Genero::Rap, Genero::Rap)
                | (Genero::Jazz, Genero::Jazz)
                | (Genero::Otros, Genero::Otros)
        )
    }
}

impl Clone for Genero {
    fn clone(&self) -> Self {
        match self {
            Genero::Rock => Genero::Rock,
            Genero::Pop => Genero::Pop,
            Genero::Rap => Genero::Rap,
            Genero::Jazz => Genero::Jazz,
            Genero::Otros => Genero::Otros,
        }
    }
}

pub struct Cancion {
    pub titulo: String,
    pub artista: String,
    pub genero: Genero,
}

impl PartialEq for Cancion {
    fn eq(&self, other: &Self) -> bool {
        self.titulo == other.titulo
            && self.artista == other.artista
            && self.genero == other.genero
    }
}

impl Clone for Cancion {
    fn clone(&self) -> Self {
        Cancion {
            titulo: self.titulo.clone(),
            artista: self.artista.clone(),
            genero: self.genero.clone(),
        }
    }
}

pub struct Playlist {
    pub nombre: String,
    pub canciones: Vec<Cancion>,
}

impl Playlist {
    pub fn new(nombre: String) -> Self {
        Playlist {
            nombre,
            canciones: Vec::new(),
        }
    }

    pub fn modificar_nombre(&mut self, nuevo_nombre: String) {
        self.nombre = nuevo_nombre;
    }

    pub fn agregar_cancion(&mut self, c: Cancion) {
        self.canciones.push(c);
    }

    pub fn buscar_cancion_y_retornar_indice(&self, c: &Cancion) -> Option<u32> {
        for i in 0..self.canciones.len() {
            if &self.canciones[i] == c {
                return Some(i);
            }
        }
        None
    }

    pub fn eliminar_cancion(&mut self, c: &Cancion) -> bool {
        if let Some(i) = self.buscar_cancion_y_retornar_indice(c) {
            self.canciones.remove(i);
            true
        } else {
            false
        }
    }

    pub fn mover_cancion(&mut self, indice_origen: u32, indice_destino: u32) -> bool {
        if indice_origen < self.canciones.len() && indice_destino < self.canciones.len() {
            let cancion = self.canciones.remove(indice_origen);
            self.canciones.insert(indice_destino, cancion);
            true
        } else {
            false
        }
    }

    pub fn buscar_por_nombre(&self, nombre: &str) -> Option<&Cancion> {
        for cancion in &self.canciones {
            if cancion.titulo == nombre {
                return Some(cancion);
            }
        }
        None
    }

    pub fn canciones_por_genero(&self, genero: &Genero) -> Vec<&Cancion> {
        let mut resultado = Vec::new();
        for cancion in &self.canciones {
            if &cancion.genero == genero {
                resultado.push(cancion);
            }
        }
        resultado
    }

    pub fn canciones_por_artista(&self, artista: &str) -> Vec<&Cancion> {
        let mut resultado = Vec::new();
        for cancion in &self.canciones {
            if cancion.artista == artista {
                resultado.push(cancion);
            }
        }
        resultado
    }

    pub fn eliminar_todas(&mut self) {
        self.canciones.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_playlist_de_prueba() -> Playlist {
        let mut playlist = Playlist::new("Mi Playlist".to_string());

        let c1 = Cancion {
            titulo: "Bohemian Rhapsody".to_string(),
            artista: "Queen".to_string(),
            genero: Genero::Rock,
        };

        let c2 = Cancion {
            titulo: "Billie Jean".to_string(),
            artista: "Michael Jackson".to_string(),
            genero: Genero::Pop,
        };

        let c3 = Cancion {
            titulo: "Lose Yourself".to_string(),
            artista: "Eminem".to_string(),
            genero: Genero::Rap,
        };

        playlist.agregar_cancion(c1);
        playlist.agregar_cancion(c2);
        playlist.agregar_cancion(c3);

        playlist
    }

    #[test]
    fn test_agregar_canciones() {
        let playlist = crear_playlist_de_prueba();
        assert_eq!(playlist.canciones.len(), 3);
    }

    #[test]
    fn test_mover_cancion() {
        let mut playlist = crear_playlist_de_prueba();
        let resultado = playlist.mover_cancion(2, 0);
        assert!(resultado);
        assert_eq!(playlist.canciones[0].titulo, "Lose Yourself");
    }

    #[test]
    fn test_filtrar_por_genero() {
        let playlist = crear_playlist_de_prueba();
        let rap = playlist.canciones_por_genero(&Genero::Rap);
        assert_eq!(rap.len(), 1);
        assert_eq!(rap[0].titulo, "Lose Yourself");
    }
}
