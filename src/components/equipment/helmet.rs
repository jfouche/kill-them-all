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
    Name(|| Name::new("Helmet")),
    Equipment(|| Equipment::Helmet),
    Armour,
    MoreLife
)]
pub struct Helmet;

impl EquipmentUI for Helmet {
    fn title() -> String {
        "Helmet".into()
    }

    fn tile_index(rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 182,
            ItemRarity::Magic => 184,
            ItemRarity::Rare => 185,
        }
    }
}

impl Helmet {
    pub fn generate_affixes<E: EntityInserter>(
        entity: &mut E,
        rarity: ItemRarity,
        ilevel: u16,
        rng: &mut ThreadRng,
    ) -> String {
        let mut provider = HelmetAffixProvider::new(ilevel);
        for _ in 0..rarity.n_affix() {
            match provider.gen(rng) {
                Some(HelmetAffixKind::AddArmour) => {
                    let value_and_tier = HELMET_MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    provider.set::<Armour, _>(entity, value_and_tier);
                }
                Some(HelmetAffixKind::MoreLife) => {
                    let value_and_tier = HELMET_MORE_LIFE_RANGES.generate(ilevel, rng);
                    provider.set::<MoreLife, _>(entity, value_and_tier);
                }
                None => {}
            }
        }
        provider.item_text()
    }
}

impl OrbAction for Helmet {
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
enum HelmetAffixKind {
    MoreLife,
    AddArmour,
}

const HELMET_MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const HELMET_MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

#[derive(Deref, DerefMut)]
struct HelmetAffixProvider(AffixProvider<HelmetAffixKind>);

impl HelmetAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            HelmetAffixKind::AddArmour,
            HELMET_MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(
            HelmetAffixKind::MoreLife,
            HELMET_MORE_LIFE_RANGES.weight(ilevel),
        );
        HelmetAffixProvider(AffixProvider::new::<Helmet>(ilevel, provider))
    }
}
