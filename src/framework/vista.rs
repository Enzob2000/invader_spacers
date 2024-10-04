

use crate::casos_usos;
use casos_usos::usos::Coordinar;
use macroquad::prelude::*;




pub struct Vista {
    casos:Coordinar,
    imagen_n: Texture2D,
    imagen_i:Texture2D
    
}

impl Vista {
    pub fn new() -> Vista {

        
        Vista {
            casos:Coordinar::new(),
            imagen_n:Texture2D::empty(),
            imagen_i:Texture2D::empty()
        }
    }

    pub async fn juego(&mut self) { 

        self.cargar().await;

        loop {
            clear_background(RED);

            self.controles().await;
            self.mov_bala();
            self.ima_nave().await;
            self.ima_in().await;
            self.casos.colision();
           

            next_frame().await;
        }
    }

    pub fn mov_bala(&mut self) {
        self.casos.balas
        .iter_mut()
        .for_each(|x| {
            x.coor_y -= 10.;

            draw_rectangle(x.coor_x, x.coor_y, x.ancho, x.altura, WHITE)
        });
    }

    pub async fn  cargar(&mut self){

        self.imagen_n=load_texture(&self.casos.nave.imagen).await.unwrap();
        self.imagen_i=load_texture(&self.casos.invasores.imagen).await.unwrap()
    }

    pub async fn controles(&mut self) {
        
        if (is_key_pressed(KeyCode::Left) || is_key_down(KeyCode::Left)) && self.casos.nave.coor_x > 0. {
            self.casos.nave.mover_iz();
        }

        if (is_key_pressed(KeyCode::Right) || is_key_down(KeyCode::Right)) && self.casos.nave.coor_x < 1240.
        {
            self.casos.nave.mover_de();
        }

        if (is_key_pressed(KeyCode::Up) || is_key_down(KeyCode::Up)) && self.casos.nave.coor_y > 0. {
            self.casos.nave.mover_arr();
        }

        if (is_key_pressed(KeyCode::Down) || is_key_down(KeyCode::Down)) && self.casos.nave.coor_y < 590.
        {
            self.casos.nave.mover_aba();
        }

        if is_key_pressed(KeyCode::Space) {
            self.casos.crear_b(self.casos.nave.coor_x + 22., self.casos.nave.coor_y - 10.);
        }
    }
                   
    pub async fn ima_nave(&mut self){
        draw_texture_ex(
            &self.imagen_n,
            self.casos.nave.coor_x, // Posición X
            self.casos.nave.coor_y, // Posición Y
            WHITE,            // Color de tinte
            DrawTextureParams {
                dest_size: Some(vec2(self.casos.nave.ancho, self.casos.nave.altura)), // Nuevo tamaño (ancho, alto)
                ..Default::default()
            },
        );
    }

    pub async fn ima_in(&mut self){

       self.casos.invasor
       .iter()
       .for_each(|x|
         {

            draw_texture_ex(
                &self.imagen_i,
                x.coor_x, // Posición X
                x.coor_y, // Posición Y
                WHITE,            // Color de tinte
                DrawTextureParams {
                    dest_size: Some(vec2(x.ancho, x.altura)), // Nuevo tamaño (ancho, alto)
                    ..Default::default()
                },
            );

         }
    
    
    );
    }
}
