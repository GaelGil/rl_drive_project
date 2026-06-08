use crate::game::entities::Team;
use crate::game::input::InputState;
use crate::game::state::GameState;

pub fn update_game(state: &mut GameState, input: InputState, delta_time: f32) {
    if state.game_over {
        if input.restart {
            *state = GameState::new(state.screen_width, state.screen_height);
        }
        return;
    }
    update_player(state, input, delta_time);
    update_bullets(state, delta_time);
    update_aliens(state, delta_time);
    handle_collisions(state);
    check_game_over(state);
}

fn update_player(state: &mut GameState, input: InputState, delta_time: f32) {
    if input.move_left {
        state.player_body.x -= state.player.speed * delta_time;
    }

    if input.move_right {
        state.player_body.x += state.player.speed * delta_time;
    }
    if state.player.body.x < 0.0 {
        state.player.body.x - 0.0;
    }

    let max_x = state.screen_width - state.player.body.width;
    if state.player.body.x > max_x {
        state.player.body.x = max.x
    }

    if state.player.shoot_cooldown > 0.0 {
        state.player.shoot_cooldown -= delta_time
    }

    if input.shoot && state.player.can_shoot() {
        state.bullets.push(state.player.shoot());
        state.player.shoot_cooldown = 0.35;
    }
}

fn update_bullets(state: &mut GameState, delta_time: f32) {
    for bullte in &mut state.bullets {
        if bullet.is_active {
            bullet.body.x += bullet.velocity_x * delta_time;
            bullet.body.y += bullet.velocity_y * delta_time;
            if bullet.body.y + bullet.body.height < 0.0 || bullet.body.y > state.screen_height {
                bullet.active = false;
            }
        }
    }
}
