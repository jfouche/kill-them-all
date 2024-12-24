use crate::components::*;
use bevy::prelude::*;
use equipment::{AffixesInserter, EquipmentUI};
use rand::{rngs::ThreadRng, Rng};
use rng_provider::RngKindProvider;

///
/// Weapon that drop a mine regularly
///
#[derive(Component)]
#[require(
    Name(||Name::new("MineDropper")),
    Weapon,
    HitDamageRange(|| HitDamageRange::new(1., 5.)),
    BaseAttackSpeed(|| BaseAttackSpeed(0.6))
)]
pub struct MineDropper;

///
/// Mine
///
#[derive(Component)]
#[require(
    Name(|| Name::new("Mine")),
    Damager,
    Collider(|| Collider::ball(8.)),
    MineExplodeTimer,
    Sprite,
    CyclicAnimation(|| CyclicAnimation::new(0..2))
)]
pub struct Mine;

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct MineExplodeTimer(Timer);

impl Default for MineExplodeTimer {
    fn default() -> Self {
        MineExplodeTimer(Timer::from_seconds(1.5, TimerMode::Once))
    }
}

#[derive(Component)]
#[require(
    Damager,
    Collider(|| Collider::ball(16.)),
    Sprite,
    OneShotAnimation(|| OneShotAnimation::new(0..8))
)]
pub struct MineExplosion;

#[derive(Resource)]
pub struct MineAssets {
    pub mine_texture: Handle<Image>,
    pub mine_atlas_layout: Handle<TextureAtlasLayout>,
    pub explosion_texture: Handle<Image>,
    pub explosion_atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for MineAssets {
    fn from_world(world: &mut World) -> Self {
        let mine_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
        let explosion_atlas_layout =
            TextureAtlasLayout::from_grid(UVec2::new(32, 32), 8, 1, None, None);

        MineAssets {
            mine_texture: world.load_asset("mine.png"),
            mine_atlas_layout: world.add_asset(mine_atlas_layout),
            explosion_texture: world.load_asset("mine_explosion.png"),
            explosion_atlas_layout: world.add_asset(explosion_atlas_layout),
        }
    }
}

/// All [MineDropper] available affixes
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum MineDropperAffixKind {
    MoreDamage,
    IncreaseDamage,
    IncreaseAttackSpeed,
}

#[derive(Deref, DerefMut)]
struct MineDropperAffixProvider(RngKindProvider<MineDropperAffixKind>);

impl MineDropperAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(MineDropperAffixKind::MoreDamage, 20);
        provider.add(MineDropperAffixKind::IncreaseDamage, 20);
        provider.add(MineDropperAffixKind::IncreaseAttackSpeed, 10);
        MineDropperAffixProvider(provider)
    }
}

impl EquipmentUI for MineDropper {
    fn title() -> String {
        "Mine dropper".into()
    }

    fn tile_index(rarity: EquipmentRarity) -> usize {
        match rarity {
            EquipmentRarity::Normal => 99,
            EquipmentRarity::Magic => 103,
            EquipmentRarity::Rare => 102,
        }
    }
}

impl MineDropper {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntity {
        let mut provider = MineDropperAffixProvider::new();
        let mut mine_dropper = AffixesInserter::spawn(commands, MineDropper, rng);
        for _ in 0..mine_dropper.n_affix() {
            match provider.gen(rng) {
                Some(MineDropperAffixKind::MoreDamage) => {
                    mine_dropper.insert::<MoreDamage, u16>(rng.gen_range(1..=3));
                }
                Some(MineDropperAffixKind::IncreaseDamage) => {
                    mine_dropper.insert::<IncreaseDamage, u16>(rng.gen_range(5..=10));
                }
                Some(MineDropperAffixKind::IncreaseAttackSpeed) => {
                    mine_dropper.insert::<IncreaseAttackSpeed, u16>(rng.gen_range(5..=15));
                }
                None => {}
            }
        }
        mine_dropper.equipment_entity()
    }
}
