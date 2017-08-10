use super::state::State;
use super::*;

pub struct Entity<'a> {
    id: usize,
    game_state: &'a mut State,
}

static mut GLOBAL_ID: usize = 0;

impl<'a> Entity<'a> {
    pub fn new(game_state: &mut State) -> Entity {
        let id = unsafe {
            let id = GLOBAL_ID;
            GLOBAL_ID += 1;
            id
        };

        Entity { game_state, id }
    }

    pub fn with_physics(&mut self, component: component::Physics) -> &mut Entity<'a> {
        self.game_state.physics_components[self.id] = component;
        self
    }

    pub fn with_graphics(&mut self, component: component::Graphics) -> &mut Entity<'a> {
        self.game_state.graphics_components[self.id] = component;
        self
    }

    pub fn build(&mut self) -> usize {
        self.id
    }
}
