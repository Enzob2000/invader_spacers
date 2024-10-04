pub struct Invasores {
    pub ancho: f32,
    pub altura: f32,
    pub coor_x: f32,
    pub coor_y: f32,
    pub imagen: String,
}

impl  Invasores {

    pub fn new() -> Invasores {
        Invasores {
            ancho: 80.0,
            altura: 80.0,
            coor_x: 600.0,
            coor_y: 200.,
            imagen:
                "C:/Users/enzo/proyectos_rust/invader_spacers/src/imagenes/enemigo-removebg-preview.png"
                    .to_string(),
        }
    }
    
}

