//! Player systems

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use super::components::*;
use super::resources::*;
use crate::creatures::systems::CreatureDeathEvent;
use crate::perks::components::{PerkBonuses, PerkInventory};
use crate::states::GameState;
use crate::weapons::components::EquippedWeapon;

/// Event fired when a player takes damage
#[derive(Event)]
pub struct PlayerDamageEvent {
    pub player_entity: Entity,
    pub damage: f32,
    pub source: Option<Entity>,
}

/// Event fired when a player dies
#[derive(Event)]
pub struct PlayerDeathEvent {
    pub player_entity: Entity,
}

/// Event fired when a player levels up
#[derive(Event)]
pub struct PlayerLevelUpEvent {
    pub player_entity: Entity,
    pub new_level: u32,
}

/// Spawns the player entity when entering Playing state
pub fn spawn_player(mut commands: Commands, config: Res<PlayerConfig>) {
    commands.spawn((
        PlayerBundle {
            player: Player::default(),
            health: Health::new(config.base_health),
            experience: Experience::new(),
            move_speed: MoveSpeed(config.base_move_speed),
            aim_direction: AimDirection::default(),
            firing: Firing::default(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.2, 0.6, 1.0),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
        },
        Invincibility::new(config.spawn_invincibility_duration),
        EquippedWeapon::default(),
        // Perk system components
        PerkInventory::new(),
        PerkBonuses::default(),
    ));
}

/// Despawns all player entities
pub fn despawn_players(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles player movement input
pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    for (mut transform, speed) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation.x += direction.x * speed.0 * time.delta_seconds();
            transform.translation.y += direction.y * speed.0 * time.delta_seconds();
        }
    }
}

/// Handles player aiming based on mouse position
pub fn player_aim(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut player_query: Query<(&Transform, &mut AimDirection), With<Player>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    else {
        return;
    };

    for (transform, mut aim) in player_query.iter_mut() {
        let player_pos = transform.translation.truncate();
        let direction = world_position - player_pos;
        if direction.length_squared() > 0.01 {
            *aim = AimDirection::from_direction(direction);
        }
    }
}

/// Handles player shooting input
pub fn player_shooting(
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<&mut Firing, With<Player>>,
) {
    for mut firing in query.iter_mut() {
        firing.is_firing = mouse.pressed(MouseButton::Left);
        firing.cooldown_timer = (firing.cooldown_timer - time.delta_seconds()).max(0.0);
    }
}

/// Applies damage to players from damage events
/// Integrates perk bonuses: damage_reduction reduces incoming damage, dodge_chance can avoid hits entirely
pub fn apply_player_damage(
    mut events: EventReader<PlayerDamageEvent>,
    mut query: Query<(&mut Health, Option<&mut Invincibility>, &PerkBonuses), With<Player>>,
    config: Res<PlayerConfig>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();

    for event in events.read() {
        if let Ok((mut health, invincibility, perk_bonuses)) = query.get_mut(event.player_entity) {
            // Skip if invincible
            if let Some(inv) = &invincibility {
                if inv.is_active() {
                    continue;
                }
            }

            // Dodge check - chance to completely avoid damage (Dodger perk)
            if perk_bonuses.dodge_chance > 0.0 && rng.gen::<f32>() < perk_bonuses.dodge_chance {
                continue; // Dodged!
            }

            // Apply damage reduction (ThickSkin perk)
            let reduced_damage = event.damage * (1.0 - perk_bonuses.damage_reduction);
            health.damage(reduced_damage);

            // Grant invincibility after taking damage
            commands
                .entity(event.player_entity)
                .insert(Invincibility::new(config.damage_invincibility_duration));
        }
    }
}

/// Checks for player death and fires death events
pub fn check_player_death(
    query: Query<(Entity, &Health), With<Player>>,
    mut death_events: EventWriter<PlayerDeathEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, health) in query.iter() {
        if health.is_dead() {
            death_events.send(PlayerDeathEvent {
                player_entity: entity,
            });
            next_state.set(GameState::GameOver);
        }
    }
}

/// Updates player experience and handles level ups
pub fn update_player_experience(
    mut query: Query<(Entity, &mut Experience), With<Player>>,
    mut level_up_events: EventWriter<PlayerLevelUpEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, mut exp) in query.iter_mut() {
        // Experience is added externally, we just check for level ups
        if exp.current >= exp.to_next_level {
            let leveled = exp.add(0); // Process level up
            if leveled {
                level_up_events.send(PlayerLevelUpEvent {
                    player_entity: entity,
                    new_level: exp.level,
                });
                next_state.set(GameState::PerkSelect);
            }
        }
    }
}

/// Ticks down invincibility timers
pub fn player_invincibility_timer(time: Res<Time>, mut query: Query<&mut Invincibility>) {
    for mut inv in query.iter_mut() {
        inv.tick(time.delta_seconds());
    }
}

/// Grants experience to players when creatures die
/// Applies exp_multiplier from perks (FastLearner)
pub fn grant_experience_on_kill(
    mut death_events: EventReader<CreatureDeathEvent>,
    mut player_query: Query<(&mut Experience, &PerkBonuses), With<Player>>,
    mut level_up_events: EventWriter<PlayerLevelUpEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in death_events.read() {
        // Grant experience to all players (for potential multiplayer support)
        for (mut exp, perk_bonuses) in player_query.iter_mut() {
            // Apply exp multiplier from FastLearner perk
            let exp_amount = (event.experience as f32 * perk_bonuses.exp_multiplier) as u32;
            let leveled_up = exp.add(exp_amount);

            if leveled_up {
                level_up_events.send(PlayerLevelUpEvent {
                    player_entity: Entity::PLACEHOLDER, // TODO: Get actual player entity
                    new_level: exp.level,
                });
                next_state.set(GameState::PerkSelect);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_damage_event_can_be_created() {
        let event = PlayerDamageEvent {
            player_entity: Entity::PLACEHOLDER,
            damage: 10.0,
            source: None,
        };
        assert_eq!(event.damage, 10.0);
    }

    #[test]
    fn player_death_event_can_be_created() {
        let event = PlayerDeathEvent {
            player_entity: Entity::PLACEHOLDER,
        };
        assert_eq!(event.player_entity, Entity::PLACEHOLDER);
    }

    #[test]
    fn player_level_up_event_can_be_created() {
        let event = PlayerLevelUpEvent {
            player_entity: Entity::PLACEHOLDER,
            new_level: 5,
        };
        assert_eq!(event.new_level, 5);
    }
}
