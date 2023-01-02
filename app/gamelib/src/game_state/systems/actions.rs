use crate::game_state::GameState;
use crate::game_state::components::motion::Velocity;

/**
 * Stop the game from running if the home buttons is pressed.
 */
pub fn system_exit_action(state: &mut GameState) {
    if state.is_running {
        state.is_running = state.changes.controls.home_button_down;
    }
}

/**
 * Create velocity in random directions if 'one' is pressed.
 */
pub fn system_shake_action(state: &mut GameState) {
    if state.changes.controls.one_button_down {
        let mut small_rng = SmallRng::seed_from_u64(10u64);
        for (_id, velocity) in state.world.query_mut::<&mut Velocity>() {
            velocity.x += small_rng.next_u32() as f32 / u32::MAX as f32 * 0.2 - 0.1;
            velocity.y += small_rng.next_u32() as f32 / u32::MAX as f32 * 0.2 - 0.1;
        }
    }
}