use crate::in_game::GROUP_PLAYER;
use crate::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::Rng;
use std::{cmp::min, time::Duration};

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub collision_groups: CollisionGroups,
    // pub gravity_scale: GravityScale,
    // pub friction: Friction,
    // pub density: ColliderMassProperties,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Dynamic,
                // friction: Friction {
                //     coefficient: 0.0,
                //     combine_rule: CoefficientCombineRule::Min,
                // },
                collision_groups: CollisionGroups::new(GROUP_PLAYER, Group::ALL),
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

// ==================================================================
// CharacterBundle

#[derive(Default, Bundle)]
pub struct CharacterBundle {
    name: Name,
    movement_speed: MovementSpeed,
    life: Life,
    attack_speed: AttackSpeed,
    weapon: Weapon,
}

impl From<EntityInstance> for CharacterBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Player" => CharacterBundle {
                name: "Player".into(),
                movement_speed: MovementSpeed::new(120.0),
                life: Life::new(24),
                attack_speed: AttackSpeed::default(),
                weapon: Weapon::new(1.2, 2, 4),
            },
            _ => CharacterBundle::default(),
        }
    }
}

// ==================================================================
// Player

#[derive(Default, Component)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[from_entity_instance]
    #[bundle]
    character: CharacterBundle,
    money: Money,
    experience: Experience,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    anim_timer: AnimationTimer,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

// ==================================================================
// Monster

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct SpawningMonster;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Damage(pub u16);

// ==================================================================
// AnimationTimer

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

// ==================================================================
// Weapon

#[derive(Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Weapon {
    attack_speed: f32,
    damage_min: u16,
    damage_max: u16,
    timer: Timer,
    ready: bool,
}

impl Default for Weapon {
    fn default() -> Self {
        const DEFAULT_ATTACK_PER_SEC: f32 = 1.0;
        Weapon {
            attack_speed: DEFAULT_ATTACK_PER_SEC,
            damage_min: 1,
            damage_max: 4,
            timer: Timer::from_seconds(1. / DEFAULT_ATTACK_PER_SEC, TimerMode::Repeating),
            ready: false,
        }
    }
}

impl Weapon {
    pub fn new(attack_per_second: f32, damage_min: u16, damage_max: u16) -> Self {
        Weapon {
            attack_speed: attack_per_second,
            damage_min,
            damage_max,
            timer: Timer::from_seconds(1. / attack_per_second, TimerMode::Repeating),
            ..Default::default()
        }
    }

    pub fn attack(&mut self) -> u16 {
        self.ready = false;
        rand::thread_rng().gen_range(self.damage_min..=self.damage_max)
    }

    pub fn tick(&mut self, delta: Duration, player_attack_speed_increases: f32) -> &Timer {
        let attack_speed = self.attack_speed * (1. + player_attack_speed_increases / 100.);
        self.timer
            .set_duration(Duration::from_secs_f32(1. / attack_speed));
        if self.timer.tick(delta).just_finished() {
            self.ready = true;
        }
        &self.timer
    }

    pub fn ready(&self) -> bool {
        self.ready
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.damage_min, self.damage_max)
    }
}

// ==================================================================
// MovementSpeed

#[derive(Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct MovementSpeed {
    speed: f32,
    increases: f32,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        MovementSpeed {
            speed: 150.0,
            increases: 0.0,
        }
    }
}

impl MovementSpeed {
    pub fn new(speed: f32) -> Self {
        MovementSpeed {
            speed,
            ..Default::default()
        }
    }

    pub fn value(&self) -> f32 {
        self.speed * (100.0 + self.increases) / 100.0
    }

    pub fn increases(&mut self, percent: f32) {
        self.increases += percent;
    }
}

impl std::fmt::Display for MovementSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}    {:.0}%", self.value(), self.increases)
    }
}

// ==================================================================
// Life

#[derive(Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Life {
    life: u16,
    max_life: u16,
    increases: f32,
}

impl Default for Life {
    fn default() -> Self {
        Life {
            life: 20,
            max_life: 20,
            increases: 0.,
        }
    }
}

impl Life {
    pub fn new(life: u16) -> Self {
        Life {
            life,
            max_life: life,
            ..Default::default()
        }
    }

    pub fn hit(&mut self, damage: u16) {
        if damage > self.life {
            self.life = 0;
        } else {
            self.life -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.life == 0
    }

    pub fn life(&self) -> u16 {
        self.life
    }

    pub fn max_life(&self) -> u16 {
        (self.max_life as f32 * (100.0 + self.increases) / 100.0) as u16
    }

    pub fn increases(&mut self, percent: f32) {
        self.increases += percent;
    }

    pub fn regenerate(&mut self, life: u16) {
        self.life = min(self.max_life(), self.life + life);
    }
}

impl std::fmt::Display for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}    +{}%",
            self.life(),
            self.max_life(),
            self.increases
        )
    }
}

// ==================================================================
// AttackSpeed

#[derive(Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct AttackSpeed {
    increases: f32,
}

impl Default for AttackSpeed {
    fn default() -> Self {
        AttackSpeed { increases: 0.0 }
    }
}

impl AttackSpeed {
    pub fn value(&self) -> f32 {
        self.increases
    }

    pub fn increases(&mut self, percent: f32) {
        self.increases += percent;
    }
}

impl std::fmt::Display for AttackSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " +{:.0}%", self.increases)
    }
}

// ==================================================================
// Bonus

#[derive(Component)]
pub struct Bonus;

// ==================================================================
// Money

#[derive(Default, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Money(pub u16);

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ==================================================================
// Experience

#[derive(Component, Default, Reflect, FromReflect)]
pub struct Experience(u32);

impl Experience {
    const LEVELS: [u32; 6] = [4, 10, 40, 100, 400, 1000];

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
