pub mod death_aura;
pub mod fireball;
pub mod mine;
pub mod shuriken;

use super::{
    item::{Item, ItemEntityInfo, ItemInfo, ItemLocation},
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use death_aura::DeathAura;
use fireball::FireBallLauncher;
use mine::MineDropper;
use rand::rngs::ThreadRng;
use shuriken::ShurikenLauncher;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
#[require(Item, Skill)]
pub struct SkillGem;

#[derive(Component, Default)]
pub struct Skill;

pub trait SkillUI {
    fn title() -> String;
    fn label() -> String;
    fn tile_index() -> usize;
}

#[derive(Component)]
#[require(ItemLocation)]
pub struct SkillGemLocation;

/// Tag to indicate that the [Skill] is affected by
/// [crate::components::affix::IncreaseAreaOfEffect] affixes
#[derive(Component, Default)]
pub struct AffectedByAreaOfEffect;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillKind {
    DeathAura,
    Fireball,
    MineDropper,
    Shuriken,
}

impl SkillKind {
    fn spawn(&self, commands: &mut Commands) -> ItemEntityInfo {
        match self {
            SkillKind::DeathAura => spawn_skill::<DeathAura>(commands),
            SkillKind::Fireball => spawn_skill::<FireBallLauncher>(commands),
            SkillKind::MineDropper => spawn_skill::<MineDropper>(commands),
            SkillKind::Shuriken => spawn_skill::<ShurikenLauncher>(commands),
        }
    }
}

pub fn spawn_skill<T>(commands: &mut Commands) -> ItemEntityInfo
where
    T: Component + Default + Into<ItemInfo>,
{
    let info: ItemInfo = T::default().into();
    let entity = commands.spawn((T::default(), info.clone())).id();
    ItemEntityInfo { entity, info }
}

pub struct SkillProvider {
    provider: RngKindProvider<SkillKind>,
}

impl SkillProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(SkillKind::Fireball, 40);
        if ilevel > 1 {
            provider.add(SkillKind::DeathAura, 40);
        }
        if ilevel > 2 {
            provider.add(SkillKind::Shuriken, 40);
        }
        if ilevel > 3 {
            provider.add(SkillKind::MineDropper, 40);
        }
        SkillProvider { provider }
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        rng: &mut ThreadRng,
    ) -> Option<ItemEntityInfo> {
        Some(self.provider.gen(rng)?.spawn(commands))
    }
}

/// Event to indicate that a skill should activate
#[derive(Event)]
pub struct ActivateSkill(pub Entity, pub Vec2);
