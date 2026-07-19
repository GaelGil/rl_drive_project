pub struct Body {
    // defining a struct for Body
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Body {
    // methods for the struct Body
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn left(&self) -> f32 {
        // the start point of the body x axis
        // Note: &self refers to current instance and is read only
        return self.x;
    }

    pub fn right(&self) -> f32 {
        // the right end of the body
        return self.x + self.width;
    }

    pub fn top(&self) -> f32 {
        // the start top boint of the body y axis
        return self.y;
    }

    pub fn bottom(&self) -> f32 {
        // bottom of the body
        return self.y + self.height;
    }

    pub fn center_x(&self) -> f32 {
        // the center of the body,
        return self.x + self.width * 0.5;
    }
}

pub struct Player {
    // a player has body
    // speed
    // can be alive or not
    // has shoot cooldown
    pub body: Body,
    pub speed: f32,
    pub is_alive: bool,
    pub shoot_cooldown: f32,
}

impl Player {
    // methods for the struct Body
    pub fn new(x: f32, y: f32) -> Self {
        // constructor for a new player
        Self {
            body: Body::new(x, y, 40.0, 20.0),
            speed: 300.0,
            is_alive: true,
            shoot_cooldown: 0.0,
        }
    }

    pub fn can_shoot(&self) -> bool {
        // a player can shoot if they are alive and cooldown is
        return self.is_alive && self.shoot_cooldown <= 0.0;
    }

    pub fn shoot(&self) -> Bullet {
        // funciton to create a new bullte from the
        Bullet::new(self.body.center_x() - 3.0, self.body.y - 12.0, 0.0, -500.0)
    }
}

pub struct Motion {
    pub velocity_x: f32,
    pub velocity_y: f32,
}

impl Motion {
    pub fn new(velocity_x: f32, velocity_y: f32) -> Self {
        Self {
            velocity_x: velocity_x,
            velocity_y: velocity_y,
        }
    }
}

pub struct Asteroid {
    pub body: Body,
    pub motion: Motion,
    pub is_alive: bool,
    pub score_value: u32,
}

impl Asteroid {
    pub fn new(x: f32, y: f32, velocity_x: f32, velocity_y: f32) -> Self {
        Self {
            body: Body::new(x, y, 30.0, 20.0),
            motion: Motion::new(velocity_x, velocity_y),
            is_alive: true,
            score_value: 100,
        }
    }
}
pub struct Bullet {
    pub body: Body,
    pub motion: Motion,
    pub is_active: bool,
}

impl Bullet {
    pub fn new(x: f32, y: f32, velocity_x: f32, velocity_y: f32) -> Self {
        Self {
            body: Body::new(x, y, 6.0, 12.0),
            motion: Motion::new(velocity_x, velocity_y),
            is_active: true,
        }
    }
}
