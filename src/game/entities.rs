#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Player,
    Alien,
}
#[derive(Debug, Clone, Copy)]
pub struct Body {
    // defining a struct body
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Body {
    // methods for the type Body
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn left(&self) -> f32 {
        // Note: Last expression is returned in rust but I wanted
        // explicit returns
        return self.x;
    }

    pub fn right(&self) -> f32 {
        return self.x + self.width;
    }

    pub fn top(&self) -> f32 {
        return self.y;
    }

    pub fn bottom(&self) -> f32 {
        return self.y + self.height;
    }

    pub fn center_x(&self) -> f32 {
        return self.x + self.width * 0.5;
    }

    pub fn center_y(&self) -> f32 {
        return self.y + self.height * 0.5;
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub body: Body,
    pub speed: f32,
    pub is_alive: bool,
    pub shoot_cooldown: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            body: Body::new(x = x, y = y, width = 40.0, height = 20.0),
            speed: 300.0,
            is_alive: true,
            shoot_cooldown: 0.0,
        }
    }
    pub fn can_shoot(&self) -> bool {
        return self.alive && self.shoot_cooldown <= 0.0;
    }

    pub fn shoot(&self) -> Bullet {
        Bullet::new(
            self.body.center_x() - 3.0,
            self.body.y - 12.0,
            0.0,
            -500.0,
            Team::Player,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Alien {
    pub body: Body,
    pub is_alive: bool,
    pub score_value: u32,
}

impl Alien {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            body: Boddy::new(x = x, y = y, width = 30.0, height = 20.0),
            is_alive: true,
            score_value: 100,
        }
    }
    pub fn shoot(&self) -> Bullet {
        Bullet::new(
            x = self.body.center_x() - 3.0,
            y = self.body.bottom(),
            velocity_x = 0.0,
            velocity_y = 300.0,
            tea = Team::Alien,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Bullet {
    pub body: Body,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub is_active: bool,
    pub team: Team,
}

impl Bullet {
    pub fn new(x: f32, y: f32, velocity_x: f32, velocity_y: f32, team: Team) -> Self {
        Self {
            body: Body::new(x = x, y = x, width = 6.0, height = 12.0),
            velocity_x,
            velocity_y,
            is_active: true,
            team,
        }
    }
}
