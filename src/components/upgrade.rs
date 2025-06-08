use super::{
    affix::{
        IncreaseAreaOfEffect, IncreaseAttackSpeed, IncreaseDamage, IncreaseMaxLife,
        IncreaseMovementSpeed, LifeRegen, MoreDamage, MoreLife, PierceChance,
    },
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Copy, Clone)]
pub struct Upgrade;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpgradeKind {
    MoreLife,
    IncreaseMaxLife,
    IncreaseLifeRegen,
    IncreaseAttackSpeed,
    IncreaseMovementSpeed,
    PierceChance,
    MoreDamage,
    IncreaseDamage,
    IncreaseAreaOfEffect,
}

///
/// Struct to store a spawned [Upgrade] and its affix label
///
pub struct UpgradeView {
    pub entity: Entity,
    pub label: String,
}

#[derive(Resource, Deref)]
pub struct UpgradeList(Vec<UpgradeView>);

impl UpgradeList {
    pub fn new(commands: &mut Commands, rng: &mut ThreadRng) -> Self {
        let mut upgrade_provider = UpgradeProvider::new();
        let upgrades = (0..3)
            .into_iter()
            .filter_map(|_| {
                let upgrade = upgrade_provider.gen(rng)?;
                Some(upgrade.generate(commands, rng))
            })
            .collect::<Vec<_>>();
        Self(upgrades)
    }

    pub fn upgrade(&mut self, mut character: EntityCommands, index: usize) {
        let upgrade_view = self.0.swap_remove(index);
        character.add_child(upgrade_view.entity);
    }
}

impl UpgradeKind {
    pub fn generate(&self, commands: &mut Commands, rng: &mut ThreadRng) -> UpgradeView {
        match self {
            UpgradeKind::MoreLife => {
                let upgrade = MoreLife(rng.random_range(2..5) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::IncreaseMaxLife => {
                let upgrade = IncreaseMaxLife(rng.random_range(2..10) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::IncreaseLifeRegen => {
                let upgrade = LifeRegen(rng.random_range(2..10) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::IncreaseAttackSpeed => {
                let upgrade = IncreaseAttackSpeed(rng.random_range(2..20) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::IncreaseMovementSpeed => {
                let upgrade = IncreaseMovementSpeed(rng.random_range(2..20) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::PierceChance => {
                let upgrade = PierceChance(rng.random_range(2..20) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::MoreDamage => {
                let upgrade = MoreDamage(rng.random_range(2..5) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::IncreaseDamage => {
                let upgrade = IncreaseDamage(rng.random_range(10..20) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::IncreaseAreaOfEffect => {
                let upgrade = IncreaseAreaOfEffect(rng.random_range(10..20) as f32);
                Self::spawn(commands, upgrade)
            }
        }
    }

    fn spawn<U>(commands: &mut Commands, upgrade: U) -> UpgradeView
    where
        U: Component + std::fmt::Display,
    {
        let label = upgrade.to_string();
        let name = std::any::type_name::<U>();
        let entity = commands.spawn((Upgrade, upgrade, Name::new(name))).id();
        UpgradeView { entity, label }
    }
}

#[derive(Deref, DerefMut)]
pub struct UpgradeProvider(RngKindProvider<UpgradeKind>);

impl UpgradeProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(UpgradeKind::MoreLife, 40);
        provider.add(UpgradeKind::IncreaseMaxLife, 40);
        provider.add(UpgradeKind::IncreaseLifeRegen, 40);
        provider.add(UpgradeKind::IncreaseAttackSpeed, 20);
        provider.add(UpgradeKind::IncreaseMovementSpeed, 40);
        provider.add(UpgradeKind::PierceChance, 20);
        provider.add(UpgradeKind::MoreDamage, 20);
        provider.add(UpgradeKind::IncreaseDamage, 20);
        provider.add(UpgradeKind::IncreaseAreaOfEffect, 20);
        UpgradeProvider(provider)
    }
}
