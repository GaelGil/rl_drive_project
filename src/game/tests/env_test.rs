use crate::game::action::Action;
use crate::game::env::GameEnv;
use crate::game::observation::OBSERVATION_SIZE;

#[test]
fn env_reset_and_step_return_stable_observations() {
    let mut env = GameEnv::new(800.0, 600.0);

    let reset_observation = env.reset();
    assert_eq!(reset_observation.len(), OBSERVATION_SIZE);

    let step_result = env.step(Action::NoOp);
    assert_eq!(step_result.observation.len(), OBSERVATION_SIZE);

    for _ in 0..300 {
        let step_result = env.step(Action::NoOp);
        assert_eq!(step_result.observation.len(), OBSERVATION_SIZE);
    }
}
