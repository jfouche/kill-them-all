use super::{
    common::{AffixProvider, EquipmentUI},
    Equipment,
};
use crate::components::{
    affix::{Armour, MoreLife, PierceChance},
    item::{AffixConfigGenerator, ItemInfo, ItemRarity},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Component)]
#[require(Name::new("Amulet"), Equipment::Amulet, Armour, MoreLife, PierceChance)]
pub struct Amulet {
    affix_provider: AmuletAffixProvider,
}

impl Amulet {
    pub fn new(ilevel: u16) -> Self {
        Amulet {
            affix_provider: AmuletAffixProvider::new(ilevel),
        }
    }
}

impl EquipmentUI for Amulet {
    fn title() -> String {
        "Amulet".into()
    }

    fn tile_index(rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 213,
            ItemRarity::Magic => 215,
            ItemRarity::Rare => 216,
        }
    }
}

impl OrbAction for Amulet {
    fn affix_reset(&mut self, ecommands: &mut EntityCommands) {
        self.affix_provider.reset();
        ecommands.insert((Armour(0.), MoreLife(0.), PierceChance(0.)));
    }

    fn affix_gen(
        &mut self,
        ecommands: &mut EntityCommands,
        count: u16,
        rarity: ItemRarity,
        rng: &mut ThreadRng,
    ) -> ItemInfo {
        let ilevel = self.affix_provider.ilevel();
        for _ in 0..count {
            match self.affix_provider.gen(rng) {
                Some(AmuletAffixKind::AddArmour) => {
                    let value_and_tier = AMULET_MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<Armour, _>(ecommands, value_and_tier);
                }
                Some(AmuletAffixKind::MoreLife) => {
                    let value_and_tier = AMULET_MORE_LIFE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreLife, _>(ecommands, value_and_tier);
                }
                Some(AmuletAffixKind::PierceChance) => {
                    let value_and_tier = AMULET_PIERCE_CHANCE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<PierceChance, _>(ecommands, value_and_tier);
                }
                None => {}
            }
        }
        // TODO: don't do this here, but when calling this, adding or updating ItemInfo
        let item_info = ItemInfo {
            tile_index: Self::tile_index(rarity),
            title: "Amulet".into(),
            text: self.affix_provider.item_text(),
        };
        ecommands.insert(item_info.clone());
        item_info
    }

    // fn gen_affixes(
    //     item: &mut EntityWorldMut,
    //     ilevel: ItemLevel,
    //     rarity: ItemRarity,
    //     rng: &mut ThreadRng,
    // ) {
    //     assert!(item.contains::<Self>());
    //     let text = Self::generate_affixes(item, rarity, *ilevel, rng);
    //     item.insert(ItemInfo {
    //         tile_index: Self::tile_index(rarity),
    //         text,
    //     });
    // }
}

/// All available affixes for [Amulet]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AmuletAffixKind {
    MoreLife,
    AddArmour,
    PierceChance,
}

const AMULET_MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const AMULET_MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const AMULET_PIERCE_CHANCE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 10), (10, (10, 24), 10), (17, (25, 29), 10)];

#[derive(Deref, DerefMut)]
struct AmuletAffixProvider(AffixProvider<AmuletAffixKind>);

impl AmuletAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();

        provider.add(
            AmuletAffixKind::AddArmour,
            AMULET_MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(
            AmuletAffixKind::MoreLife,
            AMULET_MORE_LIFE_RANGES.weight(ilevel),
        );
        provider.add(
            AmuletAffixKind::PierceChance,
            AMULET_PIERCE_CHANCE_RANGES.weight(ilevel),
        );

        AmuletAffixProvider(AffixProvider::new::<Amulet>(ilevel, provider))
    }
}
