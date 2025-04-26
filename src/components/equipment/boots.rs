use super::{
    common::{AffixProvider, EquipmentUI},
    Equipment,
};
use crate::components::{
    affix::{Armour, IncreaseMovementSpeed, MoreLife},
    item::{AffixConfigGenerator, ItemInfo, ItemRarity},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Component)]
#[require(
    Name::new("Boots"),
    Equipment::Boots,
    Armour,
    MoreLife,
    IncreaseMovementSpeed
)]
pub struct Boots {
    affix_provider: BootsAffixProvider,
}

impl Boots {
    pub fn new(ilevel: u16) -> Self {
        Boots {
            affix_provider: BootsAffixProvider::new(ilevel),
        }
    }
}

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

impl OrbAction for Boots {
    fn affix_reset(&mut self, ecommands: &mut EntityCommands) {
        self.affix_provider.reset();
        ecommands.insert((Armour(0.), MoreLife(0.), IncreaseMovementSpeed(0.)));
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
                Some(BootsAffixKind::AddArmour) => {
                    let value_and_tier = BOOTS_MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<Armour, _>(ecommands, value_and_tier);
                }
                Some(BootsAffixKind::AddLife) => {
                    let value_and_tier = BOOTS_MORE_LIFE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreLife, _>(ecommands, value_and_tier);
                }
                Some(BootsAffixKind::IncreaseMovementSpeed) => {
                    let value_and_tier = BOOTS_INCR_MOVEMENT_SPEED_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<IncreaseMovementSpeed, _>(ecommands, value_and_tier);
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
