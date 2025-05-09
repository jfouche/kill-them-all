use super::{
    common::{AffixProvider, EquipmentUI},
    weapon::{BaseAttackSpeed, Weapon},
};
use crate::components::{
    affix::{IncreaseAttackSpeed, IncreaseDamage, MoreDamage, PierceChance},
    damage::BaseHitDamageRange,
    item::{AffixConfigGenerator, ItemInfo, ItemRarity},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

/// A [Wand]
#[derive(Component)]
#[require(
    Name::new("Wand"),
    Weapon,
    BaseHitDamageRange::new(1., 2.),
    BaseAttackSpeed(1.2),
    MoreDamage,
    IncreaseDamage,
    PierceChance,
    IncreaseAttackSpeed
)]
pub struct Wand {
    affix_provider: WandAffixProvider,
}

impl Wand {
    pub fn new(ilevel: u16) -> Self {
        Wand {
            affix_provider: WandAffixProvider::new(ilevel),
        }
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

impl OrbAction for Wand {
    fn affix_reset(&mut self, ecommands: &mut EntityCommands) {
        self.affix_provider.reset();
        ecommands.insert((
            MoreDamage(0.),
            IncreaseDamage(0.),
            PierceChance(0.),
            IncreaseAttackSpeed(0.),
        ));
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
                Some(WandAffixKind::MoreDamage) => {
                    let value_and_tier = WAND_MORE_DAMAGE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreDamage, _>(ecommands, value_and_tier);
                }
                Some(WandAffixKind::IncreaseDamage) => {
                    let value_and_tier = WAND_INCR_DAMAGE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<IncreaseDamage, _>(ecommands, value_and_tier);
                }
                Some(WandAffixKind::PierceChance) => {
                    let value_and_tier = WAND_PIERCE_CHANCE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<PierceChance, _>(ecommands, value_and_tier);
                }
                Some(WandAffixKind::IncreaseAttackSpeed) => {
                    let value_and_tier = WAND_INCR_ATTACK_SPEED_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<IncreaseAttackSpeed, _>(ecommands, value_and_tier);
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
