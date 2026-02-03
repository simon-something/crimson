//! Creature systems

use bevy::prelude::*;

use super::components::*;
use super::spawner::{calculate_spawn_position, SpawnConfig};
use crate::player::components::{Health, Player};
use crate::player::systems::PlayerDamageEvent;

/// Event to spawn a creature
#[derive(Event)]
pub struct SpawnCreatureEvent {
    pub creature_type: CreatureType,
    pub position: Option<Vec3>,
}

/// Event fired when a creature dies
#[derive(Event)]
pub struct CreatureDeathEvent {
    pub entity: Entity,
    pub creature_type: CreatureType,
    pub position: Vec3,
    pub experience: u32,
}

/// Handles creature spawn events
pub fn handle_creature_spawns(
    mut commands: Commands,
    mut events: EventReader<SpawnCreatureEvent>,
    player_query: Query<&Transform, With<Player>>,
) {
    let spawn_config = SpawnConfig::default();

    for event in events.read() {
        let position = if let Some(pos) = event.position {
            pos
        } else if let Ok(player_transform) = player_query.get_single() {
            calculate_spawn_position(player_transform.translation.truncate(), &spawn_config)
        } else {
            // No player, spawn at edge of arena
            calculate_spawn_position(Vec2::ZERO, &spawn_config)
        };

        commands.spawn(CreatureBundle::new(event.creature_type, position));
    }
}

/// Updates AI state for all creatures
pub fn creature_ai_update(
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut creature_query: Query<(&Transform, &mut AIState, &Creature)>,
    time: Res<Time>,
) {
    // Find the nearest player (for multiplayer support)
    let players: Vec<_> = player_query.iter().collect();

    for (creature_transform, mut ai_state, creature) in creature_query.iter_mut() {
        // Update attack cooldown
        ai_state.attack_cooldown = (ai_state.attack_cooldown - time.delta_seconds()).max(0.0);

        // Skip dead creatures
        if ai_state.mode == AIMode::Dead {
            continue;
        }

        // Find nearest player
        let creature_pos = creature_transform.translation.truncate();
        let mut nearest_player: Option<(Entity, f32)> = None;

        for (entity, player_transform) in &players {
            let player_pos = player_transform.translation.truncate();
            let distance = creature_pos.distance(player_pos);

            if nearest_player.is_none() || distance < nearest_player.unwrap().1 {
                nearest_player = Some((*entity, distance));
            }
        }

        // Update target
        ai_state.target = nearest_player.map(|(e, _)| e);

        // Update AI mode based on creature type
        match creature.creature_type {
            CreatureType::Turret | CreatureType::BossNest => {
                ai_state.mode = AIMode::Stationary;
            }
            CreatureType::AlienShooter => {
                if let Some((_, distance)) = nearest_player {
                    if distance < 200.0 {
                        ai_state.mode = AIMode::Flee;
                    } else if distance > 400.0 {
                        ai_state.mode = AIMode::Chase;
                    } else {
                        ai_state.mode = AIMode::Circle;
                    }
                }
            }
            _ => {
                ai_state.mode = AIMode::Chase;
            }
        }

        // Update wander timer
        ai_state.wander_timer -= time.delta_seconds();
        if ai_state.wander_timer <= 0.0 {
            ai_state.wander_timer = rand::random::<f32>() * 2.0 + 1.0;
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            ai_state.wander_direction = Vec2::new(angle.cos(), angle.sin());
        }
    }
}

/// Moves creatures based on their AI state
pub fn creature_movement(
    player_query: Query<&Transform, With<Player>>,
    mut creature_query: Query<(&mut Transform, &AIState, &CreatureSpeed), With<Creature>>,
    time: Res<Time>,
) {
    for (mut transform, ai_state, speed) in creature_query.iter_mut() {
        if speed.0 <= 0.0 || ai_state.mode == AIMode::Dead {
            continue;
        }

        let creature_pos = transform.translation.truncate();
        let mut direction = Vec2::ZERO;

        match ai_state.mode {
            AIMode::Chase => {
                if let Some(target) = ai_state.target {
                    if let Ok(player_transform) = player_query.get(target) {
                        let player_pos = player_transform.translation.truncate();
                        direction = (player_pos - creature_pos).normalize_or_zero();
                    }
                }
            }
            AIMode::Flee => {
                if let Some(target) = ai_state.target {
                    if let Ok(player_transform) = player_query.get(target) {
                        let player_pos = player_transform.translation.truncate();
                        direction = (creature_pos - player_pos).normalize_or_zero();
                    }
                }
            }
            AIMode::Circle => {
                if let Some(target) = ai_state.target {
                    if let Ok(player_transform) = player_query.get(target) {
                        let player_pos = player_transform.translation.truncate();
                        let to_player = player_pos - creature_pos;
                        // Move perpendicular to player
                        direction = Vec2::new(-to_player.y, to_player.x).normalize_or_zero();
                    }
                }
            }
            AIMode::Wander => {
                direction = ai_state.wander_direction;
            }
            AIMode::Stationary | AIMode::Dead => {}
        }

        if direction != Vec2::ZERO {
            let movement = direction * speed.0 * time.delta_seconds();
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

/// Handles creature attacks on players
pub fn creature_attack(
    creature_query: Query<
        (&Transform, &AIState, &ContactDamage, &Creature),
        Without<MarkedForDespawn>,
    >,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut damage_events: EventWriter<PlayerDamageEvent>,
) {
    const ATTACK_RANGE: f32 = 32.0; // Contact distance
    const ATTACK_COOLDOWN: f32 = 1.0;

    for (creature_transform, ai_state, damage, _creature) in creature_query.iter() {
        if ai_state.mode == AIMode::Dead || ai_state.attack_cooldown > 0.0 {
            continue;
        }

        let creature_pos = creature_transform.translation.truncate();

        for (player_entity, player_transform) in player_query.iter() {
            let player_pos = player_transform.translation.truncate();
            let distance = creature_pos.distance(player_pos);

            if distance < ATTACK_RANGE {
                damage_events.send(PlayerDamageEvent {
                    player_entity,
                    damage: damage.0,
                    source: None,
                });
                // Note: We'd need to update attack_cooldown here, but it requires
                // mutable access to AIState which we'd handle with a separate component
                break;
            }
        }
    }
}

/// Checks for dead creatures and marks them for despawn
pub fn check_creature_death(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &CreatureHealth,
            &Creature,
            &Transform,
            &ExperienceValue,
        ),
        Without<MarkedForDespawn>,
    >,
    mut death_events: EventWriter<CreatureDeathEvent>,
) {
    for (entity, health, creature, transform, exp) in query.iter() {
        if health.is_dead() {
            death_events.send(CreatureDeathEvent {
                entity,
                creature_type: creature.creature_type,
                position: transform.translation,
                experience: exp.0,
            });
            commands.entity(entity).insert(MarkedForDespawn);
        }
    }
}

/// Removes creatures marked for despawn
pub fn cleanup_dead_creatures(
    mut commands: Commands,
    query: Query<Entity, With<MarkedForDespawn>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Despawns all creatures when leaving Playing state
pub fn despawn_all_creatures(mut commands: Commands, query: Query<Entity, With<Creature>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_creature_event_can_be_created() {
        let event = SpawnCreatureEvent {
            creature_type: CreatureType::Zombie,
            position: Some(Vec3::new(100.0, 200.0, 0.0)),
        };
        assert_eq!(event.creature_type, CreatureType::Zombie);
    }

    #[test]
    fn creature_death_event_contains_position() {
        let event = CreatureDeathEvent {
            entity: Entity::PLACEHOLDER,
            creature_type: CreatureType::Spider,
            position: Vec3::new(50.0, 75.0, 0.0),
            experience: 10,
        };
        assert_eq!(event.position.x, 50.0);
        assert_eq!(event.experience, 10);
    }
}
