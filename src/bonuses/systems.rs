//! Bonus systems

use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use crate::creatures::components::{Creature, CreatureHealth, MarkedForDespawn};
use crate::creatures::systems::CreatureDeathEvent;
use crate::perks::components::PerkBonuses;
use crate::player::components::{Experience, Health, MoveSpeed, Player};
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
#[allow(clippy::type_complexity)]
pub fn apply_bonus_effects(
    mut events: EventReader<BonusCollectedEvent>,
    mut player_query: Query<
        (
            &mut Health,
            &mut Experience,
            &mut EquippedWeapon,
            Option<&mut ActiveBonusEffects>,
            &PerkBonuses,
        ),
        With<Player>,
    >,
    _commands: Commands,
    creatures: Query<Entity, (With<Creature>, Without<MarkedForDespawn>)>,
    mut creature_health: Query<&mut CreatureHealth>,
) {
    for event in events.read() {
        let Ok((mut health, mut exp, mut weapon, active_effects, perk_bonuses)) =
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
                let new_weapon_id = weapons[idx];
                // Apply ammo multiplier from perks
                let base_ammo = 100;
                let bonus_ammo = (base_ammo as f32 * perk_bonuses.ammo_multiplier) as u32;
                // Use EquippedWeapon::new to create new weapon with proper initialization
                *weapon = EquippedWeapon::new(new_weapon_id, Some(bonus_ammo));
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

/// Spawns bonuses when creatures die (chance-based with weighted selection)
pub fn spawn_bonus_on_death(
    mut death_events: EventReader<CreatureDeathEvent>,
    mut spawn_events: EventWriter<SpawnBonusEvent>,
) {
    let mut rng = rand::thread_rng();
    const DROP_CHANCE: f32 = 0.15; // 15% chance to drop a bonus

    // All bonus types for weighted selection
    let bonus_types = [
        BonusType::SmallHealth,
        BonusType::LargeHealth,
        BonusType::FullHealth,
        BonusType::SmallExp,
        BonusType::LargeExp,
        BonusType::WeaponPickup,
        BonusType::SpeedBoost,
        BonusType::FireRateBoost,
        BonusType::DamageBoost,
        BonusType::Invincibility,
        BonusType::Shield,
        BonusType::Nuke,
        BonusType::Freeze,
        BonusType::SlowMotion,
    ];

    // Calculate total weight
    let total_weight: u32 = bonus_types.iter().map(|b| b.spawn_weight()).sum();

    for event in death_events.read() {
        // Roll for drop
        if rng.gen::<f32>() > DROP_CHANCE {
            continue;
        }

        // Weighted random selection
        let roll = rng.gen_range(0..total_weight);
        let mut cumulative = 0;
        let mut selected = BonusType::SmallHealth;

        for bonus_type in &bonus_types {
            cumulative += bonus_type.spawn_weight();
            if roll < cumulative {
                selected = *bonus_type;
                break;
            }
        }

        spawn_events.send(SpawnBonusEvent {
            bonus_type: selected,
            position: event.position,
        });
    }
}

/// Updates active bonus effect timers
pub fn update_active_bonus_effects(
    time: Res<Time>,
    mut query: Query<&mut ActiveBonusEffects, With<Player>>,
) {
    for mut effects in query.iter_mut() {
        effects.tick(time.delta_seconds());
    }
}

/// Applies speed boost to player movement
pub fn apply_speed_boost(
    mut query: Query<(&mut MoveSpeed, &ActiveBonusEffects), With<Player>>,
    base_speed: Res<crate::player::resources::PlayerConfig>,
) {
    for (mut speed, effects) in query.iter_mut() {
        if effects.has_speed_boost() {
            speed.0 = base_speed.base_move_speed * 1.5; // 50% speed boost
        } else {
            speed.0 = base_speed.base_move_speed;
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
