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
        let player = Player::new(x=screen_wdith * 0.5, y=screen_height -60.0);
        let aliens = Self::create_aliens();
        Self{
            player: player,
            aliens: aliens,
            bullets: Vec::new(),
            score: 0,
            game_over: false,
            alien_direction: 1.0,
            screen_width: screen_width,
            screen_height: screen_height
        }
    }

    fn create_aliens() -> Vec<Aliens> {
        ...
    }
}
