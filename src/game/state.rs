use std::time;
use std::fmt;
use std::default::Default;

use super::component;

pub struct State {
    pub delta_time: time::Duration,
    pub physics_components: [Option<component::Physics>; 2048],
    pub graphics_components: [Option<component::Graphics>; 2048],
    pub sound_components: [Option<component::Sound>; 2048],
    pub ai_components: [Option<component::AI>; 2048],
    pub entities: [Option<component::Entity>; 2048],
}

// impl fmt::Debug for State {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

impl Default for State {
    fn default() -> State {
        State {
            physics_components: [None; 2048],
        }
    }
}
