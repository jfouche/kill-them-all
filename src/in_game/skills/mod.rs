mod death_aura_plugin;
mod fireball_plugin;
mod mine_plugin;
mod shuriken_plugin;

pub use plugin::SkillsPlugin;

mod plugin {
    use super::*;
    use crate::{
        components::{
            affix::{IncreaseAreaOfEffect, PierceChance},
            character::{Character, HitEvent},
            damage::Projectile,
            equipment::weapon::AttackTimer,
            skills::{
                death_aura::DeathAuraBook, fireball::FireBallLauncherBook, mine::MineDropperBook,
                shuriken::ShurikenLauncherBook, AffectedByAreaOfEffect, Skill, SkillOfBook,
            },
        },
        schedule::{GameRunningSet, GameState},
    };
    use bevy::prelude::*;

    pub struct SkillsPlugin;

    impl Plugin for SkillsPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                fireball_plugin::FireballPlugin,
                shuriken_plugin::ShurikenPlugin,
                mine_plugin::MinePlugin,
                death_aura_plugin::DeathAuraPlugin,
            ))
            .add_systems(
                PreUpdate,
                tick_attack_timer.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                update_skills_affected_by_aoe.in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(update_character_observers)
            .add_observer(equip_skill_book::<DeathAuraBook>)
            .add_observer(unequip_skill_book::<DeathAuraBook>)
            .add_observer(equip_skill_book::<FireBallLauncherBook>)
            .add_observer(unequip_skill_book::<FireBallLauncherBook>)
            .add_observer(equip_skill_book::<MineDropperBook>)
            .add_observer(unequip_skill_book::<MineDropperBook>)
            .add_observer(equip_skill_book::<ShurikenLauncherBook>)
            .add_observer(unequip_skill_book::<ShurikenLauncherBook>);
        }
    }

    fn tick_attack_timer(mut timers: Query<&mut AttackTimer, With<Skill>>, time: Res<Time>) {
        for mut timer in &mut timers {
            timer.tick(time.delta());
        }
    }

    fn update_character_observers(trigger: Trigger<OnAdd, Character>, mut commands: Commands) {
        commands.entity(trigger.target()).observe(try_pierce);
    }

    fn equip_skill_book<B>(
        trigger: Trigger<OnAdd, ChildOf>,
        mut commands: Commands,
        books: Query<&ChildOf, With<B>>,
        characters: Query<(), With<Character>>,
    ) where
        B: Component + SkillOfBook,
        <B as SkillOfBook>::Skill: Component + Default,
    {
        if let Ok(child_of) = books.get(trigger.target()) {
            if characters.get(child_of.parent()).is_ok() {
                info!("Equip {} - ", std::any::type_name::<B>());
                commands
                    .entity(child_of.parent())
                    .insert(children![B::Skill::default()]);
            }
        }
    }

    fn unequip_skill_book<B>(
        trigger: Trigger<OnRemove, ChildOf>,
        mut commands: Commands,
        books: Query<&ChildOf, With<B>>,
        characters: Query<&Children, With<Character>>,
        skills: Query<(Entity, &ChildOf), With<<B as SkillOfBook>::Skill>>,
    ) where
        B: Component + SkillOfBook,
        <B as SkillOfBook>::Skill: Component,
    {
        if let Ok(child_of) = books.get(trigger.target()) {
            if characters.get(child_of.parent()).is_ok() {
                if let Some(skill_entity) = skills
                    .iter()
                    .filter_map(|(e, co)| (co.parent() == child_of.parent()).then_some(e))
                    .next()
                {
                    info!("Unequip {} - ", std::any::type_name::<B>());
                    commands.entity(skill_entity).despawn();
                }
            }
        }
    }

    fn try_pierce(
        trigger: Trigger<HitEvent>,
        mut commands: Commands,
        mut projectiles: Query<&mut PierceChance, With<Projectile>>,
    ) {
        if let Ok(mut pierce_chance) = projectiles.get_mut(trigger.damager) {
            let mut rng = rand::rng();
            if !pierce_chance.try_pierce(&mut rng) {
                // Didn't pierce => despawn projectile
                commands.entity(trigger.damager).despawn();
            } else {
                info!("Projectile {} pierced", trigger.damager);
            }
        }
    }

    fn update_skills_affected_by_aoe(
        mut skills: Query<(&mut Transform, &ChildOf), (With<Skill>, With<AffectedByAreaOfEffect>)>,
        characters: Query<&IncreaseAreaOfEffect, With<Character>>,
    ) {
        for (mut transform, child_of) in &mut skills {
            if let Ok(incr) = characters.get(child_of.parent()) {
                let scale = 1. + **incr / 100.;
                transform.scale = Vec3::splat(scale);
            }
        }
    }
}
