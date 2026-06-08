use crate::game::action::Action;
use crate::game::input::InputState;
use crate::game::observation::get_observation;
use crate::game::state::GameState;
use crate::game::update::update_game;

const FIXED_DELTA_TIME: f32 = 1.0 / 60.0;

#[derive(Debug, Clone)]
pub struct StepResult {
    pub observation: Vec<f32>,
    pub reward: f32,
    pub done: bool,
    pub score: u32,
}

pub struct GameEnv {
    pub state: GameState,
}

impl GameEnv {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            state: GameState::new(screen_width, screen_height),
        }
    }

    pub fn reset(&mut self) -> Vec<f32> {
        let screen_width = self.state.screen_width;
        let screen_height = self.state.screen_height;

        self.state = GameState::new(screen_width, screen_height);

        return get_observation(&self.state);
    }

    pub fn step(&mut self, action: Action) -> StepResult {
        let score_before = self.state.score;

        let input = action_to_input(action);

        update_game(&mut self.state, input, FIXED_DELTA_TIME);

        let reward = self.calculate_reward(score_before);
        let observation = get_observation(&self.state);

        return StepResult {
            observation,
            reward,
            done: self.state.game_over,
            score: self.state.score,
        };
    }

    pub fn observation(&self) -> Vec<f32> {
        return get_observation(&self.state);
    }

    fn calculate_reward(&self, score_before: u32) -> f32 {
        let score_gained = self.state.score.saturating_sub(score_before);
        let destroy_reward = score_gained as f32 / 100.0;

        if self.state.game_over && !self.state.player.is_alive {
            return -10.0 + destroy_reward;
        }
        return 0.01 + destroy_reward;
    }
}

fn action_to_input(action: Action) -> InputState {
    match action {
        Action::NoOp => InputState::default(),
        Action::MoveLeft => InputState {
            move_left: true,
            ..InputState::default()
        },
        Action::MoveRight => InputState {
            move_right: true,
            ..InputState::default()
        },
        Action::Shoot => InputState {
            shoot: true,
            ..InputState::default()
        },
    }
}
