use crate::components::*;
use bevy::prelude::*;
use equipment::{AffixesInserter, EquipmentUI};
use rand::{rngs::ThreadRng, Rng};
use rng_provider::RngKindProvider;

#[derive(Resource)]
pub struct ShurikenAssets {
    pub shuriken: Handle<Image>,
}

impl FromWorld for ShurikenAssets {
    fn from_world(world: &mut World) -> Self {
        ShurikenAssets {
            shuriken: world.load_asset("shuriken.png"),
        }
    }
}

///
/// Weapon that launch [Shuriken]s
///
#[derive(Component)]
#[require(
    Weapon,
    Name(|| Name::new("ShurikenLauncher")),
    BaseHitDamageRange(|| BaseHitDamageRange::new(2., 4.)),
    BaseAttackSpeed(|| BaseAttackSpeed(1.5)),
)]
pub struct ShurikenLauncher {
    pub dir: Dir2,
}

impl Default for ShurikenLauncher {
    fn default() -> Self {
        ShurikenLauncher { dir: Dir2::NORTH }
    }
}

///
/// A shuriken projectile
///
#[derive(Component)]
#[require(
    Name(|| Name::new("Shuriken")),
    Projectile,
    Sprite,
    Collider(|| Collider::ball(8.))
)]
pub struct Shuriken;

/// All [ShurikenLauncher] available affixes
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ShurikenLauncherAffixKind {
    MoreDamage,
    IncreaseDamage,
    PierceChance,
    IncreaseAttackSpeed,
}

#[derive(Deref, DerefMut)]
struct ShurikenLauncherAffixProvider(RngKindProvider<ShurikenLauncherAffixKind>);

impl ShurikenLauncherAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(ShurikenLauncherAffixKind::MoreDamage, 20);
        provider.add(ShurikenLauncherAffixKind::IncreaseDamage, 20);
        provider.add(ShurikenLauncherAffixKind::PierceChance, 10);
        provider.add(ShurikenLauncherAffixKind::IncreaseAttackSpeed, 10);
        ShurikenLauncherAffixProvider(provider)
    }
}

impl EquipmentUI for ShurikenLauncher {
    fn title() -> String {
        "Shuriken launcher".into()
    }

    fn tile_index(rarity: EquipmentRarity) -> usize {
        match rarity {
            EquipmentRarity::Normal => 156,
            EquipmentRarity::Magic => 151,
            EquipmentRarity::Rare => 150,
        }
    }
}

impl ShurikenLauncher {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntity {
        let mut provider = ShurikenLauncherAffixProvider::new();
        let mut shuriken_launcher =
            AffixesInserter::spawn(commands, ShurikenLauncher::default(), rng);
        for _ in 0..shuriken_launcher.n_affix() {
            match provider.gen(rng) {
                Some(ShurikenLauncherAffixKind::MoreDamage) => {
                    shuriken_launcher.insert::<MoreDamage, u16>(rng.gen_range(1..=3));
                }
                Some(ShurikenLauncherAffixKind::IncreaseDamage) => {
                    shuriken_launcher.insert::<IncreaseDamage, u16>(rng.gen_range(5..=10));
                }
                Some(ShurikenLauncherAffixKind::PierceChance) => {
                    shuriken_launcher.insert::<PierceChance, u16>(rng.gen_range(5..=10));
                }
                Some(ShurikenLauncherAffixKind::IncreaseAttackSpeed) => {
                    shuriken_launcher.insert::<IncreaseAttackSpeed, u16>(rng.gen_range(5..=15));
                }
                None => {}
            }
        }
        shuriken_launcher.equipment_entity()
    }
}
