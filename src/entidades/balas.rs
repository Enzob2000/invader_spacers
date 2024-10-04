
#[derive(Clone)]
pub struct Balas{
     pub ancho:f32,
     pub altura:f32,
    pub coor_x:f32,
     pub coor_y:f32,
     pub coli:bool
    
}

impl Balas {
    
    pub fn new(x:f32,y:f32)->Balas{

        Balas{

            ancho:5.,
            altura:15.,
            coor_y:y,
            coor_x:x,
            coli:true
        }
    }
    
   
}