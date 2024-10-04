
pub struct Naves {
    pub ancho: f32,
   pub altura: f32,
    pub coor_x: f32,
    pub coor_y: f32,
    pub imagen:String,
    pub life:bool

}

impl Naves {
    pub fn new() -> Naves {
        Naves{

            ancho:50.0,
            altura:50.0,
            coor_x:600.0,
            coor_y:590.0,
            imagen:"C:/Users/enzo/proyectos_rust/invader_spacers/src/imagenes/naves-removebg-preview.png".to_string(),
            life:true

        }
    }

    pub fn mover_iz(&mut self) {
        if self.coor_x > 0.0 {
            self.coor_x -= 10.0;
        }
    }

    pub fn mover_de(&mut self) {
        self.coor_x += 10.0;
    }

    pub fn mover_arr(&mut self) {
        self.coor_y -= 10.0;
    }

    pub fn mover_aba(&mut self) {
        self.coor_y += 10.0;
    }
}
