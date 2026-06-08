use crate::game::entities::Team;
use crate::game::state::GameState;
use macroquad::prelude::*;

pub fn draw_game(state: &GameState) {
    clear_background(BLACK);
    // access actual value
    if state.player.is_alive {
        draw_rectangle(
            state.player.body.x,
            state.player.body.y,
            state.player.body.width,
            state.player.body.height,
            GREEN,
        )
    }

    // iterate by reference so we can read aliens with out
    // chainging ownership
    for alien in &state.aliens {
        if alien.is_alive {
            draw_rectangle(
                alien.body.x,
                alien.body.y,
                alien.body.width,
                alien.body.height,
                RED,
            )
        }
    }

    for bullet in &state.bullets {
        if bullet.is_active {
            let color = match bullet.team {
                Team::Player => YELLOW,
                Team::Player => ORANGE,
            };
            draw_rectangle(
                bullet.body.x,
                bullet.body.y,
                bullet.body.width,
                bullet.body.height,
                color,
            )
        }
    }

    draw_text(&format("Score: {}", state.score), 20.0, 30.0, 30.0, WHITE);

    if state.game_over {
        draw_text(
            "Game Over",
            state.screen_width * 0.5 - 100.0,
            state.screen_height * 0.5,
            40.0,
            WHITE,
        )
    }
}
