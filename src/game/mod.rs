pub mod state;
pub mod system;
pub mod component;
pub mod asset;
pub mod entity;

use self::state::State;
use self::entity::Entity;

pub fn init() -> (State, State) {
    let mut state = State::default();
    let mut next_state = State::default();

    let player = Entity::new(&mut state)
        .with_physics(component::Physics::new())
        .build();

    (state, next_state)
}
