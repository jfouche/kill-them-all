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
#[require(Name::new("Helmet"), Equipment::Helmet, Armour, MoreLife)]
pub struct Helmet {
    affix_provider: HelmetAffixProvider,
}

impl Helmet {
    pub fn new(ilevel: u16) -> Self {
        Helmet {
            affix_provider: HelmetAffixProvider::new(ilevel),
        }
    }
}

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

impl OrbAction for Helmet {
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
    ) -> ItemInfo {
        let ilevel = self.affix_provider.ilevel();
        for _ in 0..count {
            match self.affix_provider.gen(rng) {
                Some(HelmetAffixKind::AddArmour) => {
                    let value_and_tier = HELMET_MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<Armour, _>(ecommands, value_and_tier);
                }
                Some(HelmetAffixKind::MoreLife) => {
                    let value_and_tier = HELMET_MORE_LIFE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreLife, _>(ecommands, value_and_tier);
                }
                None => {}
            }
        }
        let item_info = ItemInfo {
            tile_index: Self::tile_index(rarity),
            title: "Helmet".into(),
            text: self.affix_provider.item_text(),
        };
        ecommands.insert(item_info.clone());
        item_info
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
