use component;
use std::time;

#[derive(Debug)]
pub struct World {
    pub delta_time: time::Duration,
    pub physics_components: Vec<component::Physics>,
    pub graphics_components: Vec<component::Graphics>,
    pub sound_components: Vec<component::Sound>,
    pub ai_components: Vec<component::AI>,
    pub entities: Vec<component::Entity>,
}
