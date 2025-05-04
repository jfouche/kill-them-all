pub mod death_aura;
pub mod fireball;
pub mod mine;
pub mod shuriken;

use super::{
    item::{Item, ItemEntityInfo, ItemInfo, ItemLocation},
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use death_aura::DeathAuraBook;
use fireball::FireBallLauncherBook;
use mine::MineDropperBook;
use rand::rngs::ThreadRng;
use shuriken::ShurikenLauncherBook;

#[derive(Component, Copy, Clone, Default)]
pub struct Skill;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
#[require(Item)]
pub struct SkillBook;

pub trait SkillBookUI {
    fn title() -> String;
    fn label() -> String;
    fn tile_index() -> usize;
}

pub trait SkillOfBook {
    type Skill;
}

#[derive(Component, Copy, Clone, Deref, Reflect)]
pub struct OfBook(pub Entity);

#[derive(Component)]
#[require(ItemLocation)]
pub struct SkillBookLocation;

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
            SkillKind::DeathAura => spawn_book::<DeathAuraBook>(commands),
            SkillKind::Fireball => spawn_book::<FireBallLauncherBook>(commands),
            SkillKind::MineDropper => spawn_book::<MineDropperBook>(commands),
            SkillKind::Shuriken => spawn_book::<ShurikenLauncherBook>(commands),
        }
    }
}

pub fn spawn_book<T>(commands: &mut Commands) -> ItemEntityInfo
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
