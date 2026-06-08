use crate::game::entities::{Alien, Bullet, Player};

pub struct GameState {
    pub player: Player,
    pub aliens: Vec<Alien>,
    pub bullets: Vec<Bullet>,
    pub score: u32,
    pub game_over: bool,
    pub alien_direction: f32,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl GameState {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let player = Player::new(screen_width * 0.5, screen_height - 60.0);
        let aliens = Self::create_aliens();
        Self {
            player: player,
            aliens: aliens,
            bullets: Vec::new(),
            score: 0,
            game_over: false,
            alien_direction: 1.0,
            screen_width: screen_width,
            screen_height: screen_height,
        }
    }

    fn create_aliens() -> Vec<Alien> {
        let mut aliens = Vec::new();
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
                aliens.push(Alien::new(x, y));
            }
        }
        return aliens;
    }
}
