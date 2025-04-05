use super::{
    common::{AffixProvider, EntityInserter, EquipmentUI},
    weapon::{BaseAttackSpeed, Weapon},
};
use crate::components::{
    affix::{IncreaseAttackSpeed, IncreaseDamage, MoreDamage, PierceChance},
    damage::BaseHitDamageRange,
    item::{AffixConfigGenerator, ItemLevel, ItemRarity},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

/// A [Wand]
#[derive(Component)]
#[require(
    Name(|| Name::new("Wand")),
    Weapon,
    BaseHitDamageRange(|| BaseHitDamageRange::new(1., 2.)),
    BaseAttackSpeed(|| BaseAttackSpeed(1.2)),
    MoreDamage,
    IncreaseDamage,
    PierceChance,
    IncreaseAttackSpeed
)]
pub struct Wand;

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
    pub fn generate_affixes<E: EntityInserter>(
        entity: &mut E,
        rarity: ItemRarity,
        ilevel: u16,
        rng: &mut ThreadRng,
    ) -> String {
        let mut provider = WandAffixProvider::new(ilevel);
        for _ in 0..rarity.n_affix() {
            match provider.gen(rng) {
                Some(WandAffixKind::MoreDamage) => {
                    let value_and_tier = WAND_MORE_DAMAGE_RANGES.generate(ilevel, rng);
                    provider.set::<MoreDamage, _>(entity, value_and_tier);
                }
                Some(WandAffixKind::IncreaseDamage) => {
                    let value_and_tier = WAND_INCR_DAMAGE_RANGES.generate(ilevel, rng);
                    provider.set::<IncreaseDamage, _>(entity, value_and_tier);
                }
                Some(WandAffixKind::PierceChance) => {
                    let value_and_tier = WAND_PIERCE_CHANCE_RANGES.generate(ilevel, rng);
                    provider.set::<PierceChance, _>(entity, value_and_tier);
                }
                Some(WandAffixKind::IncreaseAttackSpeed) => {
                    let value_and_tier = WAND_INCR_ATTACK_SPEED_RANGES.generate(ilevel, rng);
                    provider.set::<IncreaseAttackSpeed, _>(entity, value_and_tier);
                }
                None => {}
            }
        }
        provider.item_text()
    }
}

impl OrbAction for Wand {
    fn reset(item: &mut EntityWorldMut) {
        assert!(item.contains::<Self>());
        item.insert((
            MoreDamage(0.),
            IncreaseDamage(0.),
            PierceChance(0.),
            IncreaseAttackSpeed(0.),
        ));
    }

    fn gen_affixes(
        item: &mut EntityWorldMut,
        ilevel: ItemLevel,
        rarity: ItemRarity,
        rng: &mut ThreadRng,
    ) {
        assert!(item.contains::<Self>());
        let _ = Self::generate_affixes(item, rarity, *ilevel, rng);
    }
}

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
struct WandAffixProvider(AffixProvider<WandAffixKind>);

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
        WandAffixProvider(AffixProvider::new::<Wand>(ilevel, provider))
    }
}
