use super::{
    common::{AffixProvider, EntityInserter, EquipmentUI},
    Equipment,
};
use crate::components::{
    affix::{Armour, MoreLife, PierceChance},
    item::{
        AffixConfigGenerator, ItemEntityInfo, ItemInfo, ItemLevel, ItemRarity, ItemRarityProvider,
    },
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Component)]
#[require(
    Name(|| Name::new("Amulet")),
    Equipment(|| Equipment::Amulet),
    Armour,
    MoreLife,
    PierceChance
)]
pub struct Amulet;

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

impl Amulet {
    pub fn spawn(commands: &mut Commands, ilevel: u16, rng: &mut ThreadRng) -> ItemEntityInfo {
        let rarity = ItemRarityProvider::gen(rng);
        let mut amulet_commands = commands.spawn((Amulet, rarity));
        let entity = amulet_commands.id();
        let info = Self::generate_affixes(&mut amulet_commands, rarity, ilevel, rng);
        ItemEntityInfo { entity, info }
    }

    fn generate_affixes<E: EntityInserter>(
        entity: &mut E,
        rarity: ItemRarity,
        ilevel: u16,
        rng: &mut ThreadRng,
    ) -> ItemInfo {
        let mut provider = AmuletAffixProvider::new(ilevel);
        for _ in 0..rarity.n_affix() {
            match provider.gen(rng) {
                Some(AmuletAffixKind::AddArmour) => {
                    let value_and_tier = AMULET_MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    provider.set::<Armour, _>(entity, value_and_tier);
                }
                Some(AmuletAffixKind::MoreLife) => {
                    let value_and_tier = AMULET_MORE_LIFE_RANGES.generate(ilevel, rng);
                    provider.set::<MoreLife, _>(entity, value_and_tier);
                }
                Some(AmuletAffixKind::PierceChance) => {
                    let value_and_tier = AMULET_PIERCE_CHANCE_RANGES.generate(ilevel, rng);
                    provider.set::<PierceChance, _>(entity, value_and_tier);
                }
                None => {}
            }
        }
        let info = ItemInfo {
            text: provider.item_text(),
            tile_index: Amulet::tile_index(rarity),
        };
        entity.insert(info.clone());
        info
    }
}

impl OrbAction for Amulet {
    fn reset(item: &mut EntityWorldMut) {
        assert!(item.contains::<Amulet>());
        item.insert((Armour(0.), MoreLife(0.), PierceChance(0.)));
    }

    fn gen_affixes(
        item: &mut EntityWorldMut,
        ilevel: ItemLevel,
        rarity: ItemRarity,
        rng: &mut ThreadRng,
    ) {
        assert!(item.contains::<Amulet>());
        let _ = Self::generate_affixes(item, rarity, *ilevel, rng);
    }
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
        let mut rng_provider = RngKindProvider::default();

        rng_provider.add(
            AmuletAffixKind::AddArmour,
            AMULET_MORE_ARMOUR_RANGES.weight(ilevel),
        );
        rng_provider.add(
            AmuletAffixKind::MoreLife,
            AMULET_MORE_LIFE_RANGES.weight(ilevel),
        );
        rng_provider.add(
            AmuletAffixKind::PierceChance,
            AMULET_PIERCE_CHANCE_RANGES.weight(ilevel),
        );

        AmuletAffixProvider(AffixProvider::new::<Amulet>(ilevel, rng_provider))
    }
}
