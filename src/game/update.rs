use crate::game::entities::Asteroid;
use crate::game::input::InputState;
use crate::game::state::GameState;
use macroquad::window::screen_width;
use rand::{Rng, RngExt};

pub fn update_game(state: &mut GameState, input: InputState, delta_time: f32) {
    if state.game_over {
        if input.restart {
            *state = GameState::new(state.screen_width, state.screen_height);
        }
        return;
    }
    update_player(state, input, delta_time);
    update_bullets(state, delta_time);
    update_asteroids(state, delta_time);
    handle_collisions(state);
    check_game_over(state);
}

fn update_player(state: &mut GameState, input: InputState, delta_time: f32) {
    if input.move_left {
        state.player.body.x -= state.player.speed * delta_time;
    }

    if input.move_right {
        state.player.body.x += state.player.speed * delta_time;
    }
    if state.player.body.x < 0.0 {
        state.player.body.x = 0.0;
    }

    let max_x = state.screen_width - state.player.body.width;
    if state.player.body.x > max_x {
        state.player.body.x = max_x
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
    for bullet in &mut state.bullets {
        if bullet.is_active {
            bullet.body.x += bullet.motion.velocity_x * delta_time;
            bullet.body.y += bullet.motion.velocity_y * delta_time;
            if bullet.body.y + bullet.body.height < 0.0 || bullet.body.y > state.screen_height {
                bullet.is_active = false;
            }
        }
    }
}

fn update_asteroids(state: &mut GameState, delta_time: f32) {
    for asteroid in &mut state.asteroids {
        if asteroid.is_alive {
            // asteroid.body.x += state.alien_direction * alien_speed * delta_time;
            asteroid.body.y += asteroid.motion.velocity_y * delta_time;
        }
    }

    state.asteroid_spawn_timer -= delta_time;
    if state.asteroid_spawn_timer <= 0.0 {
        spawn_asteroid(state);
        state.asteroid_spawn_timer = state.asteroid_spawn_interval;
    }
}

fn handle_collisions(state: &mut GameState) {
    for bullet in &mut state.bullets {
        if !bullet.is_active {
            continue;
        }

        for asteroid in &mut state.asteroids {
            if asteroid.is_alive && overlaps(&bullet.body, &asteroid.body) {
                bullet.is_active = false;
                asteroid.is_alive = false;
                state.score += asteroid.score_value;
                break;
            }
        }
    }
    //
    // Note: |for a bullet| is it active?
    state.bullets.retain(|bullet| bullet.is_active);
}

fn check_game_over(state: &mut GameState) {
    // Note: |for asteroid| is it alive?
    if state.asteroids.iter().all(|asteroid| !asteroid.is_alive) {
        state.game_over = true;
        return;
    }
    for asteroid in &state.asteroids {
        if asteroid.is_alive
            && state.player.is_alive
            && overlaps(&asteroid.body, &state.player.body)
        {
            state.player.is_alive = false;
            state.game_over = true;
            return;
        }
    }
}

fn overlaps(a: &crate::game::entities::Body, b: &crate::game::entities::Body) -> bool {
    a.left() < b.right() && a.right() > b.left() && a.top() < b.bottom() && a.bottom() > b.top()
}

fn spawn_asteroid(state: &mut GameState) {
    let mut rng = rand::rng();
    let asteroid_width = 30.0;
    let max_x = state.screen_width - asteroid_width;

    let x = rng.random_range(0.0..max_x);
    let y = -20.0;
    let velocity_y = rng.random_range(100.0..180.0);

    state.asteroids.push(Asteroid::new(x, y, 0.0, velocity_y));
}
