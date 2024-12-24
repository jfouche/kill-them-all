use crate::components::*;
use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};
use bevy_rapier2d::prelude::*;
use equipment::{AffixesInserter, EquipmentUI};
use rand::{rngs::ThreadRng, Rng};
use rng_provider::RngKindProvider;

///
/// Death aura weapon
///
#[derive(Component)]
#[require(
    Name(|| Name::new("DeathAura")),
    Weapon,
    Damager,
    BaseDamageOverTime(|| BaseDamageOverTime(1.)),
    Transform,
    Visibility,
    Collider(|| Collider::ball(32.))
)]
pub struct DeathAura;

///
/// Assets for [DeathAura]
///
#[derive(Resource)]
pub struct DeathAuraAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<DeathAuraMaterial>,
}

impl FromWorld for DeathAuraAssets {
    fn from_world(world: &mut World) -> Self {
        DeathAuraAssets {
            mesh: world.add_asset(Circle::new(32.)),
            material: world.add_asset(DeathAuraMaterial {
                color: LinearRgba::BLUE,
            }),
        }
    }
}

///
///  Material for [DeathAura]
///
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DeathAuraMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for DeathAuraMaterial {
    fn fragment_shader() -> ShaderRef {
        "death_aura.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

/// All [DeathAura] available affixes
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum DeathAuraAffixKind {
    MoreDamage,
    IncreaseDamage,
}

#[derive(Deref, DerefMut)]
struct DeathAuraAffixProvider(RngKindProvider<DeathAuraAffixKind>);

impl DeathAuraAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(DeathAuraAffixKind::MoreDamage, 20);
        provider.add(DeathAuraAffixKind::IncreaseDamage, 20);
        DeathAuraAffixProvider(provider)
    }
}

impl EquipmentUI for DeathAura {
    fn title() -> String {
        "Death aura".into()
    }

    fn tile_index(rarity: EquipmentRarity) -> usize {
        match rarity {
            EquipmentRarity::Normal => 26,
            EquipmentRarity::Magic => 47,
            EquipmentRarity::Rare => 61,
        }
    }
}

impl DeathAura {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntity {
        let mut provider = DeathAuraAffixProvider::new();
        let mut death_aura = AffixesInserter::spawn(commands, DeathAura, rng);
        // TODO: bad rarity icon!
        for _ in 0..death_aura.n_affix() {
            match provider.gen(rng) {
                Some(DeathAuraAffixKind::MoreDamage) => {
                    death_aura.insert::<MoreDamage, u16>(rng.gen_range(1..=3));
                }
                Some(DeathAuraAffixKind::IncreaseDamage) => {
                    death_aura.insert::<IncreaseDamage, u16>(rng.gen_range(5..=10));
                }
                None => {}
            }
        }
        death_aura.equipment_entity()
    }
}
