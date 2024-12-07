use super::{rng_provider::RngKindProvider, *};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Copy, Clone)]
pub struct Upgrade;

#[derive(Bundle)]
struct UpgradeBundle<U: Component> {
    tag: Upgrade,
    upgrade: U,
    name: Name,
}

impl<U> UpgradeBundle<U>
where
    U: Component,
{
    pub fn new(upgrade: U) -> Self {
        UpgradeBundle {
            tag: Upgrade,
            upgrade,
            name: std::any::type_name::<U>().into(),
        }
    }
}

pub struct UpgradeView {
    pub entity: Entity,
    pub label: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpgradeKind {
    IncreaseMaxLife,
    IncreaseLifeRegen,
    IncreaseAttackSpeed,
    IncreaseMovementSpeed,
    PierceChance,
}

impl UpgradeKind {
    pub fn generate(&self, commands: &mut Commands, rng: &mut ThreadRng) -> UpgradeView {
        match self {
            UpgradeKind::IncreaseMaxLife => {
                let upgrade = IncreaseMaxLife(rng.gen_range(2..10) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::IncreaseLifeRegen => {
                let upgrade = LifeRegen(rng.gen_range(2..10) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::IncreaseAttackSpeed => {
                let upgrade = IncreaseAttackSpeed(rng.gen_range(2..20) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::IncreaseMovementSpeed => {
                let upgrade = IncreaseMovementSpeed(rng.gen_range(2..20) as f32);
                Self::spawn(commands, upgrade)
            }
            UpgradeKind::PierceChance => {
                let upgrade = PierceChance(rng.gen_range(2..20) as f32);
                Self::spawn(commands, upgrade)
            }
        }
    }

    fn spawn<U>(commands: &mut Commands, upgrade: U) -> UpgradeView
    where
        U: Component + Clone + std::fmt::Display,
    {
        let label = upgrade.to_string();
        let bundle = UpgradeBundle::new(upgrade);
        let entity = commands.spawn(bundle).id();
        UpgradeView { entity, label }
    }
}

#[derive(Deref, DerefMut)]
pub struct UpgradeProvider(RngKindProvider<UpgradeKind>);

impl UpgradeProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(UpgradeKind::IncreaseMaxLife, 40);
        provider.add(UpgradeKind::IncreaseLifeRegen, 40);
        provider.add(UpgradeKind::IncreaseAttackSpeed, 20);
        provider.add(UpgradeKind::IncreaseMovementSpeed, 40);
        provider.add(UpgradeKind::PierceChance, 20);

        UpgradeProvider(provider)
    }
}
