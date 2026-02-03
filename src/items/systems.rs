//! Item systems

use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use crate::creatures::{Creature, CreatureHealth};
use crate::creatures::systems::CreatureDeathEvent;
use crate::player::components::Player;
use crate::player::resources::PlayerInputMapping;
use crate::bonuses::ActiveBonusEffects;

/// Event fired when a player uses their carried item
#[derive(Event)]
pub struct ItemUsedEvent {
    pub player_entity: Entity,
    pub item_type: ItemType,
    pub position: Vec3,
}

/// Event fired when a player picks up an item
#[derive(Event)]
pub struct ItemPickedUpEvent {
    pub player_entity: Entity,
    pub item_type: ItemType,
    pub replaced: Option<ItemType>,
}

/// Handles space key press to use carried item
pub fn handle_item_use(
    keyboard: Res<ButtonInput<KeyCode>>,
    input_mapping: Res<PlayerInputMapping>,
    mut player_query: Query<(Entity, &Transform, &mut CarriedItem), With<Player>>,
    mut item_events: EventWriter<ItemUsedEvent>,
) {
    for (entity, transform, mut carried) in player_query.iter_mut() {
        if keyboard.just_pressed(input_mapping.use_item) {
            if let Some(item_type) = carried.take_item() {
                info!("Player used item: {:?}", item_type);
                item_events.send(ItemUsedEvent {
                    player_entity: entity,
                    item_type,
                    position: transform.translation,
                });
            }
        }
    }
}

/// Applies the effects of used items
pub fn apply_item_effects(
    mut commands: Commands,
    mut item_events: EventReader<ItemUsedEvent>,
    mut creatures: Query<(Entity, &Transform, &mut CreatureHealth), With<Creature>>,
    mut player_query: Query<&mut ActiveBonusEffects, With<Player>>,
) {
    for event in item_events.read() {
        match event.item_type {
            ItemType::Nuke => {
                // Kill all creatures on screen
                info!("NUKE! Killing all creatures");
                for (entity, _, _) in creatures.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }

            ItemType::Freeze => {
                // Damage and slow all creatures (simplified: just damage)
                info!("FREEZE! Damaging all creatures");
                for (_, _, mut health) in creatures.iter_mut() {
                    health.damage(20.0);
                }
                // TODO: Add frozen status effect to creatures
            }

            ItemType::Shield => {
                // Grant shield to player
                if let Ok(mut effects) = player_query.get_mut(event.player_entity) {
                    effects.shield_timer = 15.0;
                    info!("Shield activated for 15 seconds");
                }
            }

            ItemType::PlasmaBlast => {
                // Damage all creatures based on distance
                info!("PLASMA BLAST!");
                let player_pos = event.position.truncate();
                for (_, transform, mut health) in creatures.iter_mut() {
                    let creature_pos = transform.translation.truncate();
                    let distance = player_pos.distance(creature_pos);
                    // More damage the closer they are
                    let damage = (300.0 - distance).max(0.0) * 0.5;
                    if damage > 0.0 {
                        health.damage(damage);
                    }
                }
            }

            ItemType::TimeWarp => {
                // Slow motion effect (handled via bonus effects)
                if let Ok(mut effects) = player_query.get_mut(event.player_entity) {
                    effects.slow_motion_timer = 8.0;
                    info!("Time Warp activated for 8 seconds");
                }
            }

            ItemType::Invincibility => {
                // Grant invincibility
                if let Ok(mut effects) = player_query.get_mut(event.player_entity) {
                    effects.invincibility_timer = 10.0;
                    info!("Invincibility activated for 10 seconds");
                }
            }

            ItemType::MissileSalvo => {
                // Damage all creatures (simplified from actual homing missiles)
                info!("MISSILE SALVO!");
                for (_, _, mut health) in creatures.iter_mut() {
                    health.damage(50.0);
                }
            }

            ItemType::Shockwave => {
                // Damage nearby creatures
                info!("SHOCKWAVE!");
                let player_pos = event.position.truncate();
                for (_, transform, mut health) in creatures.iter_mut() {
                    let creature_pos = transform.translation.truncate();
                    let distance = player_pos.distance(creature_pos);
                    if distance < 200.0 {
                        health.damage(100.0);
                    }
                }
            }

            ItemType::ToxicCloud => {
                // Poison nearby creatures (simplified: instant damage)
                info!("TOXIC CLOUD!");
                let player_pos = event.position.truncate();
                for (_, transform, mut health) in creatures.iter_mut() {
                    let creature_pos = transform.translation.truncate();
                    let distance = player_pos.distance(creature_pos);
                    if distance < 250.0 {
                        health.damage(30.0);
                    }
                }
                // TODO: Add poison status effect
            }

            ItemType::Overdrive => {
                // Double fire rate temporarily
                if let Ok(mut effects) = player_query.get_mut(event.player_entity) {
                    effects.fire_rate_boost_timer = 10.0;
                    info!("Overdrive activated for 10 seconds");
                }
            }
        }
    }
}

/// Spawns item pickups when creatures die (rare drops)
pub fn spawn_item_on_death(
    mut commands: Commands,
    mut death_events: EventReader<CreatureDeathEvent>,
) {
    let mut rng = rand::thread_rng();
    // Items are rarer than bonuses - 3% base chance
    const BASE_DROP_CHANCE: f32 = 0.03;

    for event in death_events.read() {
        // Boss creatures have higher drop chance
        let drop_chance = if event.experience >= 100 {
            BASE_DROP_CHANCE * 5.0 // 15% for bosses
        } else if event.experience >= 30 {
            BASE_DROP_CHANCE * 2.0 // 6% for strong creatures
        } else {
            BASE_DROP_CHANCE
        };

        if rng.gen::<f32>() < drop_chance {
            let item_type = ItemType::random();
            spawn_item_at(&mut commands, item_type, event.position);
            info!("Dropped item {:?} at {:?}", item_type, event.position);
        }
    }
}

/// Handles player collecting item pickups
pub fn collect_items(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut CarriedItem), With<Player>>,
    pickup_query: Query<(Entity, &Transform, &ItemPickup)>,
    mut pickup_events: EventWriter<ItemPickedUpEvent>,
) {
    const PICKUP_RADIUS: f32 = 30.0;

    for (player_entity, player_transform, mut carried) in player_query.iter_mut() {
        let player_pos = player_transform.translation.truncate();

        for (pickup_entity, pickup_transform, pickup) in pickup_query.iter() {
            let pickup_pos = pickup_transform.translation.truncate();
            let distance = player_pos.distance(pickup_pos);

            if distance < PICKUP_RADIUS {
                // Collect the item (replaces current item if any)
                let replaced = carried.item;
                carried.set_item(pickup.item_type);

                pickup_events.send(ItemPickedUpEvent {
                    player_entity,
                    item_type: pickup.item_type,
                    replaced,
                });

                info!(
                    "Picked up {:?}{}",
                    pickup.item_type,
                    if replaced.is_some() {
                        format!(" (replaced {:?})", replaced.unwrap())
                    } else {
                        String::new()
                    }
                );

                commands.entity(pickup_entity).despawn_recursive();
            }
        }
    }
}

/// Updates item pickup lifetimes and despawns expired ones
pub fn update_item_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ItemLifetime), With<ItemPickup>>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.remaining -= time.delta_seconds();
        if lifetime.remaining <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Helper function to spawn an item pickup (called by other systems)
pub fn spawn_item_at(commands: &mut Commands, item_type: ItemType, position: Vec3) {
    commands.spawn(ItemPickupBundle::new(item_type, position));
}

/// Helper function to spawn a random item pickup
pub fn spawn_random_item_at(commands: &mut Commands, position: Vec3) {
    let item_type = ItemType::random();
    spawn_item_at(commands, item_type, position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_used_event_can_be_created() {
        let event = ItemUsedEvent {
            player_entity: Entity::PLACEHOLDER,
            item_type: ItemType::Nuke,
            position: Vec3::ZERO,
        };
        assert_eq!(event.item_type, ItemType::Nuke);
    }

    #[test]
    fn item_picked_up_event_can_be_created() {
        let event = ItemPickedUpEvent {
            player_entity: Entity::PLACEHOLDER,
            item_type: ItemType::Shield,
            replaced: Some(ItemType::Freeze),
        };
        assert_eq!(event.item_type, ItemType::Shield);
        assert_eq!(event.replaced, Some(ItemType::Freeze));
    }
}
