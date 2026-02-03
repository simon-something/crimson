//! Bonus systems

use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use crate::creatures::components::{Creature, CreatureHealth, MarkedForDespawn};
use crate::player::components::{Experience, Health, Player};
use crate::weapons::components::{EquippedWeapon, WeaponId};

/// Event to spawn a bonus
#[derive(Event)]
pub struct SpawnBonusEvent {
    pub bonus_type: BonusType,
    pub position: Vec3,
}

/// Event fired when a bonus is collected
#[derive(Event)]
pub struct BonusCollectedEvent {
    pub player_entity: Entity,
    pub bonus_type: BonusType,
}

/// Handles bonus spawn events
pub fn handle_bonus_spawns(mut commands: Commands, mut events: EventReader<SpawnBonusEvent>) {
    for event in events.read() {
        commands.spawn(BonusBundle::new(event.bonus_type, event.position));
    }
}

/// Attracts bonuses toward nearby players
#[allow(clippy::type_complexity)]
pub fn bonus_attraction(
    time: Res<Time>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut bonus_query: Query<(&mut Transform, &mut BonusAttraction), (With<Bonus>, Without<Player>)>,
) {
    const ATTRACTION_DISTANCE: f32 = 100.0;

    for (player_entity, player_transform) in player_query.iter() {
        let player_pos = player_transform.translation.truncate();

        for (mut bonus_transform, mut attraction) in bonus_query.iter_mut() {
            let bonus_pos = bonus_transform.translation.truncate();
            let distance = player_pos.distance(bonus_pos);

            if distance < ATTRACTION_DISTANCE {
                attraction.target = Some(player_entity);

                // Move toward player
                let direction = (player_pos - bonus_pos).normalize_or_zero();
                let movement = direction * attraction.speed * time.delta_seconds();
                bonus_transform.translation.x += movement.x;
                bonus_transform.translation.y += movement.y;
            }
        }
    }
}

/// Handles bonus collection when player touches a bonus
pub fn bonus_collection(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    bonus_query: Query<(Entity, &Transform, &Bonus)>,
    mut collected_events: EventWriter<BonusCollectedEvent>,
) {
    const COLLECTION_RADIUS: f32 = 24.0;

    for (player_entity, player_transform) in player_query.iter() {
        let player_pos = player_transform.translation.truncate();

        for (bonus_entity, bonus_transform, bonus) in bonus_query.iter() {
            let bonus_pos = bonus_transform.translation.truncate();
            let distance = player_pos.distance(bonus_pos);

            if distance < COLLECTION_RADIUS {
                collected_events.send(BonusCollectedEvent {
                    player_entity,
                    bonus_type: bonus.bonus_type,
                });
                commands.entity(bonus_entity).despawn_recursive();
            }
        }
    }
}

/// Updates bonus lifetimes and despawns expired bonuses
pub fn bonus_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BonusLifetime), With<Bonus>>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.remaining -= time.delta_seconds();
        if lifetime.remaining <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Applies the effects of collected bonuses
pub fn apply_bonus_effects(
    mut events: EventReader<BonusCollectedEvent>,
    mut player_query: Query<
        (
            &mut Health,
            &mut Experience,
            &mut EquippedWeapon,
            Option<&mut ActiveBonusEffects>,
        ),
        With<Player>,
    >,
    _commands: Commands,
    creatures: Query<Entity, (With<Creature>, Without<MarkedForDespawn>)>,
    mut creature_health: Query<&mut CreatureHealth>,
) {
    for event in events.read() {
        let Ok((mut health, mut exp, mut weapon, active_effects)) =
            player_query.get_mut(event.player_entity)
        else {
            continue;
        };

        match event.bonus_type {
            // Health bonuses
            BonusType::SmallHealth => {
                health.heal(25.0);
            }
            BonusType::LargeHealth => {
                health.heal(50.0);
            }
            BonusType::FullHealth => {
                health.current = health.max;
            }

            // Experience bonuses
            BonusType::SmallExp => {
                exp.add(25);
            }
            BonusType::LargeExp => {
                exp.add(100);
            }

            // Weapon pickup (random weapon)
            BonusType::WeaponPickup => {
                let weapons = [
                    WeaponId::Shotgun,
                    WeaponId::Uzi,
                    WeaponId::AssaultRifle,
                    WeaponId::PlasmaRifle,
                    WeaponId::RocketLauncher,
                    WeaponId::Flamethrower,
                    WeaponId::Minigun,
                ];
                let mut rng = rand::thread_rng();
                let idx = rng.gen_range(0..weapons.len());
                weapon.weapon_id = weapons[idx];
                weapon.ammo = Some(100); // Give some ammo
            }

            // Temporary effects
            BonusType::SpeedBoost => {
                if let Some(mut effects) = active_effects {
                    effects.speed_boost_timer = BonusType::SpeedBoost.duration().unwrap_or(10.0);
                }
            }
            BonusType::FireRateBoost => {
                if let Some(mut effects) = active_effects {
                    effects.fire_rate_boost_timer =
                        BonusType::FireRateBoost.duration().unwrap_or(10.0);
                }
            }
            BonusType::DamageBoost => {
                if let Some(mut effects) = active_effects {
                    effects.damage_boost_timer = BonusType::DamageBoost.duration().unwrap_or(10.0);
                }
            }
            BonusType::Invincibility => {
                if let Some(mut effects) = active_effects {
                    effects.invincibility_timer =
                        BonusType::Invincibility.duration().unwrap_or(5.0);
                }
            }
            BonusType::Shield => {
                if let Some(mut effects) = active_effects {
                    effects.shield_timer = BonusType::Shield.duration().unwrap_or(15.0);
                }
            }
            BonusType::SlowMotion => {
                if let Some(mut effects) = active_effects {
                    effects.slow_motion_timer = BonusType::SlowMotion.duration().unwrap_or(5.0);
                }
            }

            // Special effects
            BonusType::Nuke => {
                // Kill all enemies on screen
                for entity in creatures.iter() {
                    if let Ok(mut ch) = creature_health.get_mut(entity) {
                        ch.damage(10000.0); // Massive damage
                    }
                }
            }
            BonusType::Freeze => {
                // Freeze is handled by the creatures module looking at a global state
                // For now, we'll skip implementation
            }
        }
    }
}

/// Despawns all bonuses when leaving Playing state
pub fn despawn_all_bonuses(mut commands: Commands, query: Query<Entity, With<Bonus>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_bonus_event_can_be_created() {
        let event = SpawnBonusEvent {
            bonus_type: BonusType::SmallHealth,
            position: Vec3::new(100.0, 200.0, 0.0),
        };
        assert_eq!(event.bonus_type, BonusType::SmallHealth);
    }

    #[test]
    fn bonus_collected_event_can_be_created() {
        let event = BonusCollectedEvent {
            player_entity: Entity::PLACEHOLDER,
            bonus_type: BonusType::LargeExp,
        };
        assert_eq!(event.bonus_type, BonusType::LargeExp);
    }
}
