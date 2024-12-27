use crate::components::*;
use bevy::prelude::*;
use skills::Skill;

#[derive(Resource)]
pub struct ShurikenAssets {
    pub shuriken: Handle<Image>,
}

impl FromWorld for ShurikenAssets {
    fn from_world(world: &mut World) -> Self {
        ShurikenAssets {
            shuriken: world.load_asset("shuriken.png"),
        }
    }
}

///
/// Skill that launch [Shuriken]s
///
#[derive(Component)]
#[require(
    Skill,
    Name(|| Name::new("ShurikenLauncher")),
    BaseHitDamageRange(|| BaseHitDamageRange::new(2., 4.)),
    BaseAttackSpeed(|| BaseAttackSpeed(1.5)),
)]
pub struct ShurikenLauncher {
    pub dir: Dir2,
}

impl Default for ShurikenLauncher {
    fn default() -> Self {
        ShurikenLauncher { dir: Dir2::NORTH }
    }
}

impl SkillUI for ShurikenLauncher {
    fn title() -> String {
        "Shuriken launcher".into()
    }

    fn label() -> String {
        "Launch shurikens".into()
    }

    fn tile_index() -> usize {
        153
    }
}
///
/// A shuriken projectile
///
#[derive(Component)]
#[require(
    Name(|| Name::new("Shuriken")),
    Projectile,
    Sprite,
    Collider(|| Collider::ball(8.))
)]
pub struct Shuriken;
