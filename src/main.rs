mod game;

use game::input::read_input;
use game::render::draw_game;
use game::state::GameState;
use game::update::update_game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Space Invaders".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen_width = 800.0;
    let screen_height = 600.0;
    let mut state = GameState::new(screen_width, screen_height);

    loop {
        let delta_time = get_frame_time();
        let input = read_input();

        update_game(&mut state, input, delta_time);
        draw_game(&state);
        next_frame().await;
    }
}
