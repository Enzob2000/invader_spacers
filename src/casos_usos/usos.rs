use crate::entidades;
use entidades::*;
use balas::Balas;
use invasores::Invasores;
use naves::Naves;


pub struct Coordinar{

    pub nave:Naves,
    pub invasores:Invasores,
    pub balas:Vec<Balas>,
    pub invasor:Vec<Invasores>
 
}

impl Coordinar {

    pub fn new()->Coordinar{

        Coordinar{
            nave:Naves::new(),
            invasores:Invasores::new(),
            balas:vec![],
            invasor:vec![Invasores::new()]
        }
    }

    pub fn crear_b(&mut self,x:f32,y:f32){

     self.balas.push(Balas::new(x, y));
  
    }

    pub fn colision(&mut self){

        

      for  i in  self.balas.iter_mut(){

        self.invasor.retain( |x|
            
        if !(i.coor_x>=x.coor_x && i.coor_x<=x.coor_x+x.ancho && i.coor_y>=x.coor_y && i.coor_y<=x.coor_y+x.altura ){
        

           
           return  true;
         
        }else{
            i.coli=false;
            false
        }
        
        );
          
      }

      self.balas.retain(|x|x.coli);
      }

  

    }

    
