use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
#[require(
    Name(|| Name::new("Player")),
    Character,
    Target(|| Target::Monster),
    BaseLife(|| BaseLife(10.)),
    BaseMovementSpeed(|| BaseMovementSpeed(130.)),
    Money,
    Experience,
    Sprite,
    Transform(|| Transform::from_xyz(0., 0., 10.)),
    AnimationTimer,
    CollisionGroups(|| CollisionGroups::new(GROUP_PLAYER, Group::ALL)),
    ActiveEvents(|| ActiveEvents::COLLISION_EVENTS)
)]
pub struct Player;

impl Player {
    pub fn sprite(assets: &PlayerAssets) -> Sprite {
        Sprite {
            image: assets.texture.clone(),
            texture_atlas: Some(assets.texture_atlas_layout.clone().into()),
            custom_size: Some(PLAYER_SIZE),
            ..Default::default()
        }
    }
}

pub const PLAYER_SIZE: Vec2 = Vec2::new(16.0, 16.0);

#[derive(Resource)]
pub struct PlayerAssets {
    pub texture: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

/// Event to notify the player died
#[derive(Event)]
pub struct PlayerDeathEvent;

/// Event to notify a player level up
#[derive(Event)]
pub struct LevelUpEvent;

// ==================================================================
// Experience

#[derive(Component, Default, Debug, Reflect)]
pub struct Experience(u32);

impl Experience {
    const LEVELS: [u32; 6] = [4, 10, 30, 80, 170, 300];

    pub fn add(&mut self, xp: u32) {
        self.0 += xp;
    }

    pub fn current(&self) -> u32 {
        self.0
    }

    /// Level starting at 0
    pub fn level(&self) -> u8 {
        let mut level = 0;
        for xp in Experience::LEVELS.iter() {
            if self.0 >= *xp {
                level += 1;
            } else {
                break;
            }
        }
        level
    }

    pub fn get_current_level_min_max_exp(&self) -> (u32, u32) {
        let level = self.level();
        let min = match level {
            0 => &0,
            _ => Experience::LEVELS.get(level as usize - 1).unwrap_or(&100),
        };
        let max = Experience::LEVELS
            .get(level as usize)
            .unwrap_or(Experience::LEVELS.last().unwrap());
        (*min, *max)
    }
}

impl std::fmt::Display for Experience {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} (level {})",
            self.0,
            self.get_current_level_min_max_exp().1,
            self.level() + 1,
        )
    }
}

///
/// [Money]
///
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct Money(pub u16);

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

///
/// Resource to store the score
///
#[derive(Default, Resource, Reflect)]
pub struct Score(pub u16);
