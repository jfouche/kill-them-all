use super::{
    common::{AffixesInserter, EquipmentUI},
    weapon::{BaseAttackSpeed, Weapon},
};
use crate::components::{
    affix::{IncreaseAttackSpeed, IncreaseDamage, MoreDamage, PierceChance},
    damage::BaseHitDamageRange,
    item::{AffixConfigGenerator, ItemEntityInfo, ItemLevel, ItemRarity},
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

/// A [Wand]
#[derive(Component)]
#[require(
    Weapon,
    Name(|| Name::new("Wand")),
    ItemLevel,
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

const WAND_MORE_DAMAGE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const WAND_INCR_DAMAGE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const WAND_PIERCE_CHANCE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 10), (10, (10, 24), 10), (17, (25, 29), 10)];

const WAND_INCR_ATTACK_SPEED_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 10), (10, (10, 24), 10), (17, (25, 29), 10)];

#[derive(Deref, DerefMut)]
struct WandAffixProvider(RngKindProvider<WandAffixKind>);

impl WandAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            WandAffixKind::MoreDamage,
            WAND_MORE_DAMAGE_RANGES.weight(ilevel),
        );
        provider.add(
            WandAffixKind::IncreaseDamage,
            WAND_INCR_DAMAGE_RANGES.weight(ilevel),
        );
        provider.add(
            WandAffixKind::PierceChance,
            WAND_PIERCE_CHANCE_RANGES.weight(ilevel),
        );
        provider.add(
            WandAffixKind::IncreaseAttackSpeed,
            WAND_INCR_ATTACK_SPEED_RANGES.weight(ilevel),
        );
        WandAffixProvider(provider)
    }
}

impl EquipmentUI for Wand {
    fn title() -> String {
        "Wand".into()
    }

    fn tile_index(rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 318,
            ItemRarity::Magic => 320,
            ItemRarity::Rare => 321,
        }
    }
}

impl Wand {
    pub fn spawn(commands: &mut Commands, ilevel: u16, rng: &mut ThreadRng) -> ItemEntityInfo {
        let mut provider = WandAffixProvider::new(ilevel);
        let mut wand = AffixesInserter::spawn(commands, Wand, ilevel, rng);
        for _ in 0..wand.n_affix() {
            match provider.gen(rng) {
                Some(WandAffixKind::MoreDamage) => {
                    wand.insert::<MoreDamage>(WAND_MORE_DAMAGE_RANGES.generate(ilevel, rng));
                }
                Some(WandAffixKind::IncreaseDamage) => {
                    wand.insert::<IncreaseDamage>(WAND_INCR_DAMAGE_RANGES.generate(ilevel, rng));
                }
                Some(WandAffixKind::PierceChance) => {
                    wand.insert::<PierceChance>(WAND_PIERCE_CHANCE_RANGES.generate(ilevel, rng));
                }
                Some(WandAffixKind::IncreaseAttackSpeed) => {
                    wand.insert::<IncreaseAttackSpeed>(
                        WAND_INCR_ATTACK_SPEED_RANGES.generate(ilevel, rng),
                    );
                }
                None => {}
            }
        }
        wand.equipment_entity()
    }
}
