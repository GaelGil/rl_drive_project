use crate::game::entities::{Asteroid, Bullet, Player}; // imports

pub struct GameState {
    // a game state has
    // single player
    // list of asteroids
    // list of bullets
    // score
    // game over check
    // screen width
    // screen height
    // base spawn timer
    // spawn interval
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
    pub asteroid_spawn_interval_decay: f32,
    pub rounds: u32,
    pub current_round: u32,
}

impl GameState {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        // to create a game state we pass in screen height and screen width
        // and create a player at the center x and bottom of the y
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
            asteroid_spawn_interval_decay: 0.20,
            rounds: 5,
            current_round: 0,
        }
    }
}
