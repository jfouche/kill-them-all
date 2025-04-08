use super::{
    common::{AffixProvider, EquipmentUI},
    Equipment,
};
use crate::components::{
    affix::{Armour, MoreLife},
    common::EntityInserter,
    item::{AffixConfigGenerator, ItemInfo, ItemLevel, ItemRarity},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Component)]
#[require(
    Name(|| Name::new("BodyArmour")),
    Equipment(|| Equipment::BodyArmour),
    Armour,
    MoreLife
)]
pub struct BodyArmour;

impl EquipmentUI for BodyArmour {
    fn title() -> String {
        "Body armour".into()
    }
    fn tile_index(rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 0,
            ItemRarity::Magic => 2,
            ItemRarity::Rare => 3,
        }
    }
}

impl BodyArmour {
    pub fn generate_affixes<E: EntityInserter>(
        entity: &mut E,
        rarity: ItemRarity,
        ilevel: u16,
        rng: &mut ThreadRng,
    ) -> String {
        let mut provider = BodyArmourAffixProvider::new(ilevel);
        for _ in 0..rarity.n_affix() {
            match provider.gen(rng) {
                Some(BodyArmourAffixKind::AddArmour) => {
                    let value_and_tier = BODYARMOUR_MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    provider.set::<Armour, _>(entity, value_and_tier);
                }
                Some(BodyArmourAffixKind::AddLife) => {
                    let value_and_tier = BODYARMOUR_MORE_LIFE_RANGES.generate(ilevel, rng);
                    provider.set::<MoreLife, _>(entity, value_and_tier);
                }
                None => {}
            }
        }
        provider.item_text()
    }
}

impl OrbAction for BodyArmour {
    fn reset(item: &mut EntityWorldMut) {
        assert!(item.contains::<Self>());
        item.insert((Armour(0.), MoreLife(0.)));
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
enum BodyArmourAffixKind {
    AddLife,
    AddArmour,
}

const BODYARMOUR_MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const BODYARMOUR_MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

#[derive(Deref, DerefMut)]
struct BodyArmourAffixProvider(AffixProvider<BodyArmourAffixKind>);

impl BodyArmourAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            BodyArmourAffixKind::AddArmour,
            BODYARMOUR_MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(
            BodyArmourAffixKind::AddLife,
            BODYARMOUR_MORE_LIFE_RANGES.weight(ilevel),
        );
        BodyArmourAffixProvider(AffixProvider::new::<BodyArmour>(ilevel, provider))
    }
}
