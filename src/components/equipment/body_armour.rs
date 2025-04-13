use super::{
    common::{AffixProvider, EquipmentUI},
    Equipment,
};
use crate::components::{
    affix::{Armour, MoreLife},
    item::{AffixConfigGenerator, ItemInfo, ItemRarity},
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
pub struct BodyArmour {
    affix_provider: BodyArmourAffixProvider,
}

impl BodyArmour {
    pub fn new(ilevel: u16) -> Self {
        BodyArmour {
            affix_provider: BodyArmourAffixProvider::new(ilevel),
        }
    }
}

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

impl OrbAction for BodyArmour {
    fn affix_reset(&mut self, ecommands: &mut EntityCommands) {
        self.affix_provider.reset();
        ecommands.insert((Armour(0.), MoreLife(0.)));
    }

    fn affix_gen(
        &mut self,
        ecommands: &mut EntityCommands,
        count: u16,
        rarity: ItemRarity,
        rng: &mut ThreadRng,
    ) {
        let ilevel = self.affix_provider.ilevel();
        for _ in 0..count {
            match self.affix_provider.gen(rng) {
                Some(BodyArmourAffixKind::AddArmour) => {
                    let value_and_tier = BODYARMOUR_MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<Armour, _>(ecommands, value_and_tier);
                }
                Some(BodyArmourAffixKind::AddLife) => {
                    let value_and_tier = BODYARMOUR_MORE_LIFE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreLife, _>(ecommands, value_and_tier);
                }
                None => {}
            }
        }
        ecommands.insert(ItemInfo {
            tile_index: Self::tile_index(rarity),
            text: self.affix_provider.item_text(),
        });
    }

    fn affix_text(&self) -> String {
        self.affix_provider.item_text()
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
