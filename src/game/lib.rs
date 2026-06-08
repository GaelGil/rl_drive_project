pub mod game;

use game::action::Action;
use game::env::GameEnv;
use game::observation::OBSERVATION_SIZE;
use pyo3::prelude::*;

#[pyclass]
pub struct RustAsteroidEnv {
    env: GameEnv,
}

#[pymethods]
impl RustAsteroidsEnv {
    #[new]
    pub fn new(screen_width: f32, scree_height: f32) -> Self {
        Self {
            env: GameEnv::new(screen_width, screen_height),
        }
    }

    pub fn reste(&mut self) -> Vec<f32> {
        return self.env.reset();
    }

    pub fn step(&mut self, action_index: usize) -> (Vec<f32>, f32, bool, u32) {
        let action = Action::from_index(action_index);
        let result = self.env.step(action);

        return (result.observation, result.reward, result.done, result.score);
    }

    pub fn observation(&self) -> Vec<f32> {
        return self.env.observation();
    }

    pub fn observation_size(&self) -> usize {
        return OBSERVATION_SIZE;
    }

    pub fn action_count(&self) -> usize {
        return Action::action_count();
    }
}

#[pymodule]
fn rl_drive_project(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RustAsteroidsEnv>()?;
    Ok(())
}
