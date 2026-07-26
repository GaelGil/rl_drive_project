mod input;
mod render;

use asteroids_core::{state::GameState, update::update_game};
use input::read_input;
use render::draw_game;

use macroquad::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Space Invaders".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    loop {
        let delta_time = get_frame_time();
        let input = read_input();

        update_game(&mut state, input, delta_time);
        draw_game(&state);
        next_frame().await;
    }
}
