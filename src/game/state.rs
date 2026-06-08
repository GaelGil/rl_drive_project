use crate::game::entities::{Asteroid, Bullet, Player};

pub struct GameState {
    pub player: Player,
    pub asteroids: Vec<Asteroid>,
    pub bullets: Vec<Bullet>,
    pub score: u32,
    pub game_over: bool,
    pub screen_width: f32,
    pub screen_height: f32,
    pub base_y: f32,
    pub asteroid_spawn_timer: f32,
    pub asteroid_spawn_interval: f32,
}

impl GameState {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let player = Player::new(screen_width * 0.5, screen_height - 60.0);
        Self {
            player: player,
            asteroids: Vec::new(),
            bullets: Vec::new(),
            score: 0,
            game_over: false,
            screen_width: screen_width,
            screen_height: screen_height,
            base_y: screen_height - 40.0,
            asteroid_spawn_timer: 0.0,
            asteroid_spawn_interval: 1.0,
        }
    }
}
