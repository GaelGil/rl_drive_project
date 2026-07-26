use crate::state::GameState;

pub const MAX_ASTEROIDS: usize = 5;
pub const MAX_BULLETS: usize = 3;

pub const OBSERVATION_SIZE: usize = 3 + MAX_ASTEROIDS * 4 + MAX_BULLETS * 3;

pub fn get_observation(state: &GameState) -> Vec<f32> {
    let mut observation = Vec::with_capacity(OBSERVATION_SIZE);
    observation.push(normalize(state.player.body.x, state.screen_width));
    observation.push(normalize(state.player.shoot_cooldown, 1.0));
    observation.push(if state.player.is_alive { 1.0 } else { 0.0 });

    let mut asteroid_count = 0;

    for asteroid in state.asteroids.iter().filter(|asteroid| asteroid.is_alive) {
        if asteroid_count >= MAX_ASTEROIDS {
            break;
        }

        observation.push(normalize(asteroid.body.x, state.screen_width));
        observation.push(normalize(asteroid.body.y, state.screen_height));
        observation.push(normalize(asteroid.motion.velocity_y, 20.0));
        observation.push(1.0);

        asteroid_count += 1;
    }

    while asteroid_count < MAX_ASTEROIDS {
        observation.push(0.0);
        observation.push(0.0);
        observation.push(0.0);
        observation.push(0.0);
        asteroid_count += 1;
    }

    let mut bullet_count = 0;
    for bullet in state.bullets.iter().filter(|bullet| bullet.is_active) {
        if bullet_count >= MAX_BULLETS {
            break;
        }
        observation.push(normalize(bullet.body.x, state.screen_width));
        observation.push(normalize(bullet.body.y, state.screen_height));
        observation.push(1.0);

        bullet_count += 1;
    }

    while bullet_count < MAX_BULLETS {
        observation.push(0.0);
        observation.push(0.0);
        observation.push(0.0);

        bullet_count += 1;
    }

    return observation;
}

fn normalize(value: f32, max_value: f32) -> f32 {
    if max_value <= 0.0 {
        return 0.0;
    }

    return (value / max_value).clamp(0.0, 1.0);
}
