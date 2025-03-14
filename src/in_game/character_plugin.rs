use crate::{
    components::{
        affix::{
            Armour, BaseArmour, IncreaseAreaOfEffect, IncreaseAttackSpeed, IncreaseDamage,
            IncreaseMaxLife, IncreaseMovementSpeed, LifeRegen, MoreArmour, MoreDamage, MoreLife,
            PierceChance,
        },
        animation::AnimationTimer,
        character::{
            BaseLife, BaseMovementSpeed, Character, CharacterAction, CharacterDiedEvent,
            CharacterDyingEvent, CharacterLevel, HitEvent, Life, LooseLifeEvent, MaxLife,
            MovementSpeed, Target,
        },
        damage::{BaseDamageOverTime, BaseHitDamageRange, DamageOverTime, HitDamageRange},
        equipment::{
            weapon::{AttackSpeed, AttackTimer, BaseAttackSpeed},
            Equipment,
        },
        inventory::TakeDroppedItemCommand,
        item::DroppedItem,
    },
    schedule::GameRunningSet,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BaseLife>()
            .register_type::<Life>()
            .register_type::<MaxLife>()
            .register_type::<MoreLife>()
            .register_type::<IncreaseMaxLife>()
            .register_type::<LifeRegen>()
            .register_type::<BaseMovementSpeed>()
            .register_type::<MovementSpeed>()
            .register_type::<IncreaseMovementSpeed>()
            .register_type::<IncreaseAttackSpeed>()
            .register_type::<PierceChance>()
            .register_type::<Armour>()
            .register_type::<MoreArmour>()
            .register_type::<MoreDamage>()
            .register_type::<IncreaseDamage>()
            .register_type::<IncreaseAreaOfEffect>()
            .register_type::<BaseAttackSpeed>()
            .register_type::<AttackSpeed>()
            .register_type::<BaseHitDamageRange>()
            .register_type::<HitDamageRange>()
            .register_type::<BaseDamageOverTime>()
            .register_type::<DamageOverTime>()
            .register_type::<AttackTimer>()
            .register_type::<Target>()
            .register_type::<CharacterAction>()
            .register_type::<AnimationTimer>()
            .register_type::<Equipment>()
            .register_type::<BaseArmour>()
            .register_type::<CharacterLevel>()
            .add_event::<HitEvent>()
            .add_event::<LooseLifeEvent>()
            .add_event::<CharacterDiedEvent>()
            .add_systems(
                Update,
                (regen_life, mitigate_damage_over_time, do_character_action)
                    .in_set(GameRunningSet::EntityUpdate),
            )
            .add_systems(
                Update,
                despawn_character_on_death.in_set(GameRunningSet::DespawnEntities),
            )
            .add_observer(init_life)
            .add_observer(add_life_observers);
    }
}

fn init_life(
    trigger: Trigger<OnAdd, (BaseLife, Life, MaxLife)>,
    mut lifes: Query<(&mut Life, &mut MaxLife, &BaseLife)>,
) {
    if let Ok((mut life, mut max_life, base_life)) = lifes.get_mut(trigger.entity()) {
        **life = **base_life;
        **max_life = **base_life;
    }
}

fn add_life_observers(trigger: Trigger<OnAdd, Character>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(mitigate_damage_on_hit)
        .observe(loose_life);
}

fn mitigate_damage_on_hit(
    trigger: Trigger<HitEvent>,
    mut commands: Commands,
    characters: Query<&Armour, With<Character>>,
) {
    if let Ok(armour) = characters.get(trigger.entity()) {
        let damage = armour.mitigate(trigger.damage);
        info!("trigger_take_hit: damage: {:.1}", *damage);
        if *damage > 0. {
            commands.trigger_targets(LooseLifeEvent(damage), trigger.entity());
        }
    }
}

fn loose_life(
    trigger: Trigger<LooseLifeEvent>,
    mut commands: Commands,
    mut characters: Query<&mut Life, With<Character>>,
) {
    if let Ok(mut life) = characters.get_mut(trigger.entity()) {
        life.damage(**trigger.event());
        if life.is_dead() {
            commands.trigger_targets(CharacterDyingEvent, trigger.entity());
        }
    }
}

fn despawn_character_on_death(mut events: EventReader<CharacterDiedEvent>, mut commands: Commands) {
    for event in events.read() {
        commands.entity(**event).despawn_recursive();
    }
}

/// [Character] is curently having [DamageOverTime]. Mitigate it whith [Armour]
fn mitigate_damage_over_time(
    mut commands: Commands,
    characters: Query<(Entity, &Armour, &DamageOverTime), With<Character>>,
    time: Res<Time>,
) {
    for (entity, armour, dot) in &characters {
        let damage = armour.mitigate(dot.damage(&time));
        commands.trigger_targets(LooseLifeEvent(damage), entity);
    }
}

/// Regenerate [Character]'s [Life]
fn regen_life(mut query: Query<(&mut Life, &MaxLife, &LifeRegen)>, time: Res<Time>) {
    for (mut life, max_life, regen) in &mut query {
        let life_per_sec = **max_life * (**regen / 100.);
        life.regenerate(life_per_sec * time.delta_secs());
        life.check(*max_life);
    }
}

fn do_character_action(
    mut commands: Commands,
    mut characters: Query<
        (
            &mut Transform,
            &MovementSpeed,
            &mut CharacterAction,
            &mut Velocity,
        ),
        With<Character>,
    >,
    items: Query<(Entity, &GlobalTransform), With<DroppedItem>>,
    time: Res<Time>,
) {
    for (mut transform, movement_speed, mut action, mut velocity) in &mut characters {
        let mut move_to = |target: Vec2| {
            let direction = target - transform.translation.xy();
            let linvel = direction.normalize_or_zero() * **movement_speed;
            if direction.length() > linvel.length() * time.delta_secs() {
                velocity.linvel = linvel;
                false
            } else {
                // Player is next to target, position it at the target and stop moving
                transform.translation.x = target.x;
                transform.translation.y = target.y;
                velocity.linvel = Vec2::ZERO;
                true
            }
        };

        match *action {
            CharacterAction::GoTo(target) => {
                if move_to(target) {
                    action.stop();
                }
            }
            CharacterAction::TakeItem(entity) => match items.get(entity) {
                Ok((entity, transform)) => {
                    if move_to(transform.translation().xy()) {
                        action.stop();
                        commands.queue(TakeDroppedItemCommand(entity));
                    }
                }
                _ => action.stop(),
            },
            CharacterAction::Stop => velocity.linvel = Vec2::ZERO,
        };
    }
}
