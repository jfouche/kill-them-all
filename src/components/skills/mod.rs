mod death_aura;
mod fireball;
mod mine;
mod shuriken;

pub use death_aura::*;
pub use fireball::*;
pub use mine::*;
pub use shuriken::*;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Skill;

pub trait SkillUI {
    fn title() -> String;
    fn label() -> String;
    fn tile_index() -> usize;
}
