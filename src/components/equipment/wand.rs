use crate::components::*;
use bevy::prelude::*;
use equipment::{AffixesInserter, EquipmentUI};
use rand::{rngs::ThreadRng, Rng};
use rng_provider::RngKindProvider;

/// A [Wand]
#[derive(Component)]
#[require(
    Weapon,
    Name(|| Name::new("Wand")),
    BaseHitDamageRange(|| BaseHitDamageRange::new(1., 2.)),
    BaseAttackSpeed(|| BaseAttackSpeed(1.2))
)]
pub struct Wand;

/// All [Wand] available affixes
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum WandAffixKind {
    MoreDamage,
    IncreaseDamage,
    PierceChance,
    IncreaseAttackSpeed,
}

#[derive(Deref, DerefMut)]
struct WandAffixProvider(RngKindProvider<WandAffixKind>);

impl WandAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(WandAffixKind::MoreDamage, 20);
        provider.add(WandAffixKind::IncreaseDamage, 20);
        provider.add(WandAffixKind::PierceChance, 10);
        provider.add(WandAffixKind::IncreaseAttackSpeed, 10);
        WandAffixProvider(provider)
    }
}

impl EquipmentUI for Wand {
    fn title() -> String {
        "Wand".into()
    }

    fn tile_index(rarity: EquipmentRarity) -> usize {
        match rarity {
            EquipmentRarity::Normal => 318,
            EquipmentRarity::Magic => 320,
            EquipmentRarity::Rare => 321,
        }
    }
}

impl Wand {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntityInfo {
        let mut provider = WandAffixProvider::new();
        let mut wand = AffixesInserter::spawn(commands, Wand, rng);
        for _ in 0..wand.n_affix() {
            match provider.gen(rng) {
                Some(WandAffixKind::MoreDamage) => {
                    wand.insert::<MoreDamage, u16>(rng.gen_range(1..=3));
                }
                Some(WandAffixKind::IncreaseDamage) => {
                    wand.insert::<IncreaseDamage, u16>(rng.gen_range(5..=10));
                }
                Some(WandAffixKind::PierceChance) => {
                    wand.insert::<PierceChance, u16>(rng.gen_range(5..=10));
                }
                Some(WandAffixKind::IncreaseAttackSpeed) => {
                    wand.insert::<IncreaseAttackSpeed, u16>(rng.gen_range(5..=15));
                }
                None => {}
            }
        }
        wand.equipment_entity()
    }
}
