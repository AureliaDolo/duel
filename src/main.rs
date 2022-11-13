use macroquad::prelude::*;

#[macroquad::main("Duel")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);


        next_frame().await
    }
}