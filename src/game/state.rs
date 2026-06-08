use crate::game::entities::{Asteroid, Bullet, Player};

pub struct GameState {
    pub player: Player,
    pub asteroids: Vec<Asteroid>,
    pub bullets: Vec<Bullet>,
    pub score: u32,
    pub game_over: bool,
    pub alien_direction: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    pub base_y: f32,
}

impl GameState {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let player = Player::new(screen_width * 0.5, screen_height - 60.0);
        let asteroids = Self::create_asteroids();
        Self {
            player: player,
            asteroids: asteroids,
            bullets: Vec::new(),
            score: 0,
            game_over: false,
            alien_direction: 1.0,
            screen_width: screen_width,
            screen_height: screen_height,
            base_y: screen_height - 40.0,
        }
    }

    fn create_asteroids() -> Vec<Asteroid> {
        let mut asteroids = Vec::new();
        let rows = 4;
        let cols = 8;
        let start_x = 80.0;
        let start_y = 60.0;
        let horizontal_spacing = 50.0;
        let vertical_spacing = 40.0;
        for row in 0..rows {
            for col in 0..cols {
                let x = start_x + col as f32 * horizontal_spacing;
                let y = start_y + row as f32 * vertical_spacing;
                asteroids.push(Asteroid::new(x, y, 0.0, 20.0));
            }
        }
        return asteroids;
    }
}
