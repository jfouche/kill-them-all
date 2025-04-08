use super::{
    common::{AffixProvider, EquipmentUI},
    Equipment,
};
use crate::components::{
    affix::{Armour, IncreaseMovementSpeed, MoreLife},
    common::EntityInserter,
    item::{AffixConfigGenerator, ItemInfo, ItemLevel, ItemRarity},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Component)]
#[require(
    Name(|| Name::new("Boots")),
    Equipment(|| Equipment::Boots),
    Armour,
    MoreLife,
    IncreaseMovementSpeed
)]
pub struct Boots;

impl EquipmentUI for Boots {
    fn title() -> String {
        "Boots".into()
    }

    fn tile_index(rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 63,
            ItemRarity::Magic => 65,
            ItemRarity::Rare => 66,
        }
    }
}

impl Boots {
    pub fn generate_affixes<E: EntityInserter>(
        entity: &mut E,
        rarity: ItemRarity,
        ilevel: u16,
        rng: &mut ThreadRng,
    ) -> String {
        let mut provider = BootsAffixProvider::new(ilevel);
        for _ in 0..rarity.n_affix() {
            match provider.gen(rng) {
                Some(BootsAffixKind::AddArmour) => {
                    let value_and_tier = BOOTS_MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    provider.set::<Armour, _>(entity, value_and_tier);
                }
                Some(BootsAffixKind::AddLife) => {
                    let value_and_tier = BOOTS_MORE_LIFE_RANGES.generate(ilevel, rng);
                    provider.set::<MoreLife, _>(entity, value_and_tier);
                }
                Some(BootsAffixKind::IncreaseMovementSpeed) => {
                    let value_and_tier = BOOTS_INCR_MOVEMENT_SPEED_RANGES.generate(ilevel, rng);
                    provider.set::<IncreaseMovementSpeed, _>(entity, value_and_tier);
                }
                None => {}
            }
        }
        provider.item_text()
    }
}

impl OrbAction for Boots {
    fn reset(item: &mut EntityWorldMut) {
        assert!(item.contains::<Self>());
        item.insert((Armour(0.), MoreLife(0.), IncreaseMovementSpeed(0.)));
    }

    fn gen_affixes(
        item: &mut EntityWorldMut,
        ilevel: ItemLevel,
        rarity: ItemRarity,
        rng: &mut ThreadRng,
    ) {
        assert!(item.contains::<Self>());
        let text = Self::generate_affixes(item, rarity, *ilevel, rng);
        item.insert(ItemInfo {
            tile_index: Self::tile_index(rarity),
            text,
        });
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum BootsAffixKind {
    AddLife,
    AddArmour,
    IncreaseMovementSpeed,
}

const BOOTS_MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const BOOTS_MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const BOOTS_INCR_MOVEMENT_SPEED_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

#[derive(Deref, DerefMut)]
struct BootsAffixProvider(AffixProvider<BootsAffixKind>);

impl BootsAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            BootsAffixKind::AddArmour,
            BOOTS_MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(
            BootsAffixKind::AddLife,
            BOOTS_MORE_LIFE_RANGES.weight(ilevel),
        );
        provider.add(
            BootsAffixKind::IncreaseMovementSpeed,
            BOOTS_INCR_MOVEMENT_SPEED_RANGES.weight(ilevel),
        );
        BootsAffixProvider(AffixProvider::new::<Boots>(ilevel, provider))
    }
}
