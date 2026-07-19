use crate::game::state::GameState;
use macroquad::prelude::*;

pub fn draw_game(state: &GameState) {
    // pass in state as reference only since we are just reading the value
    clear_background(BLACK);
    // draw the player if alive
    if state.player.is_alive {
        draw_rectangle(
            state.player.body.x,
            state.player.body.y,
            state.player.body.width,
            state.player.body.height,
            GREEN,
        )
    }

    // iterate by reference so we can read asteroids
    // we are only reading them not modifying them
    for asteroid in &state.asteroids {
        if asteroid.is_alive {
            draw_rectangle(
                asteroid.body.x,
                asteroid.body.y,
                asteroid.body.width,
                asteroid.body.height,
                RED,
            )
        }
    }

    for bullet in &state.bullets {
        if bullet.is_active {
            draw_rectangle(
                bullet.body.x,
                bullet.body.y,
                bullet.body.width,
                bullet.body.height,
                YELLOW,
            )
        }
    }

    // display the score and current round
    draw_text(&format!("Score: {}", state.score), 20.0, 30.0, 30.0, WHITE);
    draw_text(
        &format!("Round: {}/{}", state.current_round, state.rounds),
        20.0,
        50.0,
        30.0,
        WHITE,
    );

    // if game is over
    if state.current_round == state.rounds {
        draw_text(
            "You Won!",
            state.screen_width * 0.5 - 100.0,
            state.screen_height * 0.5,
            40.0,
            WHITE,
        );
    }

    // if game is over
    if state.game_over {
        draw_text(
            "Game Over",
            state.screen_width * 0.5 - 100.0,
            state.screen_height * 0.5,
            40.0,
            WHITE,
        );
    }
}
