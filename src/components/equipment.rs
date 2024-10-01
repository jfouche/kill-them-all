use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use super::rng_provider::{Generator, RngKindProvider};

// ==================================================================
// EquipmentAssets

#[derive(Resource)]
pub struct EquipmentAssets {
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl EquipmentAssets {
    pub fn load(
        asset_server: &AssetServer,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let texture_atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
            UVec2::new(48, 48),
            16,
            22,
            None,
            None,
        ));

        let texture = asset_server
            .load("items/Kyrise's 16x16 RPG Icon Pack - V1.3/spritesheet/spritesheet_48x48.png");

        EquipmentAssets {
            texture,
            texture_atlas_layout,
        }
    }

    pub fn image(&self, equipment: &Equipment) -> (Handle<Image>, TextureAtlas) {
        match equipment {
            Equipment::Helmet(helmet) => self.helmet(helmet),
            Equipment::BodyArmour(body_armour) => self.body_armour(body_armour),
            Equipment::Boots(boots) => self.boots(boots),
        }
    }

    pub fn helmet(&self, helmet: &Helmet) -> (Handle<Image>, TextureAtlas) {
        let index = match helmet {
            Helmet::None => 351,
            Helmet::Normal(_) => 182,
            Helmet::Magic(_) => 184,
        };
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index,
            },
        )
    }

    pub fn body_armour(&self, body_armour: &BodyArmour) -> (Handle<Image>, TextureAtlas) {
        let index = match body_armour {
            BodyArmour::None => 351,
            BodyArmour::Normal(_) => 0,
            BodyArmour::Magic(_) => 2,
        };
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index,
            },
        )
    }

    pub fn boots(&self, boots: &Boots) -> (Handle<Image>, TextureAtlas) {
        let index = match boots {
            Boots::None => 351,
            Boots::Normal(_) => 63,
            Boots::Magic(_) => 65,
        };
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index,
            },
        )
    }
}

// ==================================================================
// Affixes traits

pub trait Armor {
    fn armor(&self) -> f32;
}

pub trait MoreLife {
    fn more_life(&self) -> f32;
}

pub trait AddMovementSpeed {
    fn more_movement_speed(&self) -> f32;
}

// ==================================================================
// Helmet

#[derive(Component, Clone, Reflect)]
pub enum Helmet {
    None,
    Normal(NormalHelmet),
    Magic(MagicHelmet),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalHelmet {
    pub armor: f32,
}

impl NormalHelmet {
    fn generate(rng: &mut ThreadRng) -> Self {
        NormalHelmet {
            armor: rng.gen_range(1..=2) as f32,
        }
    }
}

#[derive(Clone, Reflect)]
pub struct MagicHelmet {
    pub base: NormalHelmet,
    pub affix: HelmetAffix,
}

impl MagicHelmet {
    fn generate(rng: &mut ThreadRng) -> Self {
        let mut affix_provider = HelmetAffixProvider::new();
        MagicHelmet {
            base: NormalHelmet::generate(rng),
            affix: affix_provider
                .gen()
                .expect("HelmetAffixProvider should provide at least 1 affix"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum HelmetAffixKind {
    AddLife,
    AddArmour,
}

#[derive(Clone, Reflect)]
pub enum HelmetAffix {
    AddLife(f32),
    AddArmour(f32),
}

impl std::fmt::Display for Helmet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Helmet::None => Ok(()),
            Helmet::Normal(helmet) => write!(f, "Helmet : +{} armour", helmet.armor as u16),
            Helmet::Magic(helmet) => write!(
                f,
                "Helmet : +{} armour\n{}",
                helmet.base.armor as u16, helmet.affix
            ),
        }
    }
}
impl std::fmt::Display for HelmetAffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelmetAffix::AddArmour(val) => write!(f, "Item add +{} armour", *val as u16),
            HelmetAffix::AddLife(val) => write!(f, "Item add +{} life", *val as u16),
        }
    }
}

impl Generator<HelmetAffix> for HelmetAffixKind {
    fn generate(&self, rng: &mut ThreadRng) -> HelmetAffix {
        match self {
            HelmetAffixKind::AddArmour => HelmetAffix::AddArmour(rng.gen_range(2..=5) as f32),
            HelmetAffixKind::AddLife => HelmetAffix::AddLife(rng.gen_range(5..=20) as f32),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct HelmetAffixProvider(RngKindProvider<HelmetAffixKind, HelmetAffix>);

impl HelmetAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(HelmetAffixKind::AddArmour, 20);
        provider.add(HelmetAffixKind::AddLife, 20);
        HelmetAffixProvider(provider)
    }
}

impl Armor for Helmet {
    fn armor(&self) -> f32 {
        match self {
            Helmet::None => 0.,
            Helmet::Normal(helmet) => helmet.armor,
            Helmet::Magic(helmet) => helmet.base.armor,
        }
    }
}

// ==================================================================
// BodyArmour

#[derive(Component, Clone, Reflect)]
pub enum BodyArmour {
    None,
    Normal(NormalBodyArmour),
    Magic(MagicBodyArmour),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalBodyArmour {
    pub armor: f32,
}

impl NormalBodyArmour {
    fn generate(rng: &mut ThreadRng) -> Self {
        NormalBodyArmour {
            armor: rng.gen_range(1..=2) as f32,
        }
    }
}

#[derive(Clone, Reflect)]
pub struct MagicBodyArmour {
    pub base: NormalBodyArmour,
    pub affix: BodyArmourAffix,
}

impl MagicBodyArmour {
    fn generate(rng: &mut ThreadRng) -> Self {
        let mut affix_provider = BodyArmourAffixProvider::new();
        MagicBodyArmour {
            base: NormalBodyArmour::generate(rng),
            affix: affix_provider
                .gen()
                .expect("BodyArmourAffixProvider should provide at least 1 affix"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyArmourAffixKind {
    AddLife,
    AddArmour,
}

#[derive(Clone, Reflect)]
pub enum BodyArmourAffix {
    AddLife(f32),
    AddArmour(f32),
}

impl std::fmt::Display for BodyArmour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyArmour::None => Ok(()),
            BodyArmour::Normal(body_armour) => {
                write!(f, "Body armour : +{} armour", body_armour.armor as u16)
            }
            BodyArmour::Magic(body_armour) => write!(
                f,
                "Body armour : +{} armour\n{}",
                body_armour.base.armor as u16, body_armour.affix
            ),
        }
    }
}

impl std::fmt::Display for BodyArmourAffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyArmourAffix::AddArmour(val) => write!(f, "Item add +{} armour", *val as u16),
            BodyArmourAffix::AddLife(val) => write!(f, "Item add +{} life", *val as u16),
        }
    }
}

impl Generator<BodyArmourAffix> for BodyArmourAffixKind {
    fn generate(&self, rng: &mut ThreadRng) -> BodyArmourAffix {
        match self {
            BodyArmourAffixKind::AddArmour => {
                BodyArmourAffix::AddArmour(rng.gen_range(2..=5) as f32)
            }
            BodyArmourAffixKind::AddLife => BodyArmourAffix::AddLife(rng.gen_range(5..=20) as f32),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct BodyArmourAffixProvider(RngKindProvider<BodyArmourAffixKind, BodyArmourAffix>);

impl BodyArmourAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(BodyArmourAffixKind::AddArmour, 20);
        provider.add(BodyArmourAffixKind::AddLife, 20);
        BodyArmourAffixProvider(provider)
    }
}

impl Armor for BodyArmour {
    fn armor(&self) -> f32 {
        match self {
            BodyArmour::None => 0.,
            BodyArmour::Normal(body_armour) => body_armour.armor,
            BodyArmour::Magic(body_armour) => body_armour.base.armor,
        }
    }
}

// ==================================================================
// Boot

#[derive(Component, Clone, Reflect)]
pub enum Boots {
    None,
    Normal(NormalBoots),
    Magic(MagicBoots),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalBoots {
    pub armor: f32,
}

impl NormalBoots {
    fn generate(rng: &mut ThreadRng) -> Self {
        NormalBoots {
            armor: rng.gen_range(1..=2) as f32,
        }
    }
}

#[derive(Clone, Reflect)]
pub struct MagicBoots {
    pub base: NormalBoots,
    pub affix: BootsAffix,
}

impl MagicBoots {
    fn generate(rng: &mut ThreadRng) -> Self {
        let mut affix_provider = BootsAffixProvider::new();
        MagicBoots {
            base: NormalBoots::generate(rng),
            affix: affix_provider
                .gen()
                .expect("BootsAffixProvider should provide at least 1 affix"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BootsAffixKind {
    AddLife,
    AddArmour,
}

#[derive(Clone, Reflect)]
pub enum BootsAffix {
    AddLife(f32),
    AddArmour(f32),
}

impl std::fmt::Display for Boots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Boots::None => Ok(()),
            Boots::Normal(boots) => {
                write!(f, "Boots : +{} armour", boots.armor as u16)
            }
            Boots::Magic(boots) => write!(
                f,
                "Boots : +{} armour\n{}",
                boots.base.armor as u16, boots.affix
            ),
        }
    }
}

impl std::fmt::Display for BootsAffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootsAffix::AddArmour(val) => write!(f, "Item add +{} armour", *val as u16),
            BootsAffix::AddLife(val) => write!(f, "Item add +{} life", *val as u16),
        }
    }
}

impl Generator<BootsAffix> for BootsAffixKind {
    fn generate(&self, rng: &mut ThreadRng) -> BootsAffix {
        match self {
            BootsAffixKind::AddArmour => BootsAffix::AddArmour(rng.gen_range(2..=5) as f32),
            BootsAffixKind::AddLife => BootsAffix::AddLife(rng.gen_range(5..=20) as f32),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct BootsAffixProvider(RngKindProvider<BootsAffixKind, BootsAffix>);

impl BootsAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(BootsAffixKind::AddArmour, 20);
        provider.add(BootsAffixKind::AddLife, 20);
        BootsAffixProvider(provider)
    }
}

impl Armor for Boots {
    fn armor(&self) -> f32 {
        match self {
            Boots::None => 0.,
            Boots::Normal(boot) => boot.armor,
            Boots::Magic(boot) => boot.base.armor,
        }
    }
}

// ==================================================================
// EquipmentProvider

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentKind {
    NormalHelmet,
    MagicHelmet,
    NormalBodyArmour,
    MagicBodyArmour,
    NormalBoots,
    MagicBoots,
}

impl Generator<Equipment> for EquipmentKind {
    fn generate(&self, rng: &mut ThreadRng) -> Equipment {
        match self {
            EquipmentKind::NormalHelmet => {
                Equipment::Helmet(Helmet::Normal(NormalHelmet::generate(rng)))
            }
            EquipmentKind::MagicHelmet => {
                Equipment::Helmet(Helmet::Magic(MagicHelmet::generate(rng)))
            }
            EquipmentKind::NormalBodyArmour => {
                Equipment::BodyArmour(BodyArmour::Normal(NormalBodyArmour::generate(rng)))
            }
            EquipmentKind::MagicBodyArmour => {
                Equipment::BodyArmour(BodyArmour::Magic(MagicBodyArmour::generate(rng)))
            }
            EquipmentKind::NormalBoots => {
                Equipment::Boots(Boots::Normal(NormalBoots::generate(rng)))
            }
            EquipmentKind::MagicBoots => Equipment::Boots(Boots::Magic(MagicBoots::generate(rng))),
        }
    }
}

#[derive(Component, Reflect)]
pub enum Equipment {
    Helmet(Helmet),
    BodyArmour(BodyArmour),
    Boots(Boots),
}

impl std::fmt::Display for Equipment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Equipment::Helmet(helmet) => helmet.fmt(f),
            Equipment::BodyArmour(body_armour) => body_armour.fmt(f),
            Equipment::Boots(boots) => boots.fmt(f),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct EquipmentProvider(RngKindProvider<EquipmentKind, Equipment>);

impl EquipmentProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::<EquipmentKind, Equipment>::default();
        provider.add(EquipmentKind::NormalHelmet, 40);
        provider.add(EquipmentKind::MagicHelmet, 20);
        provider.add(EquipmentKind::NormalBodyArmour, 40);
        provider.add(EquipmentKind::MagicBodyArmour, 20);
        provider.add(EquipmentKind::NormalBoots, 40);
        provider.add(EquipmentKind::MagicBoots, 20);

        EquipmentProvider(provider)
    }
}
