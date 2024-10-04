
mod entidades;
mod framework;
mod casos_usos;

#[macroquad::main("invader espacer")]
async fn main() {
    
    framework::vista::Vista::new().juego().await
}
