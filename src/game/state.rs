use std::time;
use std::fmt;
use std::default::Default;

use super::component;

pub struct State {
    pub delta_time: time::Duration,
    pub physics_components: Vec<Option<component::Physics>>,
    pub graphics_components: Vec<Option<component::Graphics>>,
    pub sound_components: Vec<Option<component::Sound>>,
    pub ai_components: Vec<Option<component::AI>>,
    pub entities: Vec<Option<component::Entity>>,
}

// impl fmt::Debug for State {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// },

impl Default for State {
    fn default() -> State {
        State {
            physics_components: vec![None; 2048],
            graphics_components: vec![None; 2048],
            sound_components: vec![None; 2048],
            ai_components: vec![None; 2048],
            entities: vec![None; 2048],
            delta_time: time::Duration::from_millis(16),
        }
    }
}
