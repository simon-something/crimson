//! Player systems

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use super::components::*;
use super::resources::*;
use crate::bonuses::ActiveBonusEffects;
use crate::creatures::CreatureDeathEvent;
use crate::perks::{PerkBonuses, PerkInventory};
use crate::states::GameState;
use crate::weapons::EquippedWeapon;

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
    // Player index for multiplayer support (0 = first player)
    let player_index: u8 = 0;

    // Different colors for different player indices in multiplayer
    let player_colors = [
        Color::srgb(0.2, 0.6, 1.0), // Blue - Player 1
        Color::srgb(1.0, 0.4, 0.2), // Orange - Player 2
        Color::srgb(0.2, 1.0, 0.4), // Green - Player 3
        Color::srgb(1.0, 0.8, 0.2), // Yellow - Player 4
    ];
    let color = player_colors.get(player_index as usize).copied().unwrap_or(player_colors[0]);

    commands.spawn((
        PlayerBundle {
            player: Player { index: player_index },
            health: Health::new(config.base_health),
            experience: Experience::new(),
            move_speed: MoveSpeed(config.base_move_speed),
            // Use from_angle to start facing right (angle 0)
            aim_direction: AimDirection::from_angle(0.0),
            firing: Firing::default(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color,
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
        // Active bonus effects (from pickups)
        ActiveBonusEffects::default(),
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
    input_mapping: Res<PlayerInputMapping>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    for (mut transform, speed) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        // Use input mapping for customizable keybindings, with arrow key fallbacks
        if keyboard.pressed(input_mapping.move_up) || keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(input_mapping.move_down) || keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(input_mapping.move_left) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(input_mapping.move_right) || keyboard.pressed(KeyCode::ArrowRight) {
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
    keyboard: Res<ButtonInput<KeyCode>>,
    input_mapping: Res<PlayerInputMapping>,
    time: Res<Time>,
    mut query: Query<(&mut Firing, &mut EquippedWeapon), With<Player>>,
) {
    for (mut firing, mut weapon) in query.iter_mut() {
        // Use configurable fire button
        firing.is_firing = mouse.pressed(input_mapping.fire);
        firing.cooldown_timer = (firing.cooldown_timer - time.delta_seconds()).max(0.0);

        // Handle reload input (2 second base reload time)
        if keyboard.just_pressed(input_mapping.reload) && !weapon.is_reloading() {
            weapon.start_reload(2.0);
        }

        // Handle use item input (currently just logs)
        if keyboard.just_pressed(input_mapping.use_item) {
            info!("Use item pressed");
        }
    }
}

/// Applies damage to players from damage events
/// Integrates perk bonuses: damage_reduction reduces incoming damage, dodge_chance can avoid hits entirely
/// Also respects ActiveBonusEffects: invincibility and shield
pub fn apply_player_damage(
    mut events: EventReader<PlayerDamageEvent>,
    mut query: Query<
        (
            &Player,
            &mut Health,
            Option<&mut Invincibility>,
            &PerkBonuses,
            &ActiveBonusEffects,
        ),
    >,
    config: Res<PlayerConfig>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();

    for event in events.read() {
        if let Ok((player, mut health, invincibility, perk_bonuses, bonus_effects)) =
            query.get_mut(event.player_entity)
        {
            // Skip if invincible (perk or pickup)
            if let Some(inv) = &invincibility {
                if inv.is_active() {
                    continue;
                }
            }
            if bonus_effects.has_invincibility() {
                continue;
            }

            // Shield absorbs damage completely
            if bonus_effects.has_shield() {
                continue;
            }

            // Dodge check - chance to completely avoid damage (Dodger perk)
            if perk_bonuses.dodge_chance > 0.0 && rng.gen::<f32>() < perk_bonuses.dodge_chance {
                continue; // Dodged!
            }

            // Apply damage reduction (ThickSkin perk)
            let reduced_damage = event.damage * (1.0 - perk_bonuses.damage_reduction);
            health.damage(reduced_damage);

            // Log damage for multiplayer support (uses player.index)
            info!("Player {} took {:.1} damage (reduced from {:.1})",
                player.index + 1, reduced_damage, event.damage);

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

/// Updates player experience display (level ups are handled by grant_experience_on_kill)
pub fn update_player_experience(
    _query: Query<(Entity, &Experience), With<Player>>,
) {
    // Experience updates and level ups are handled by grant_experience_on_kill
    // This system exists for potential future UI updates or experience decay mechanics
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
    mut player_query: Query<(Entity, &mut Experience, &PerkBonuses), With<Player>>,
    mut level_up_events: EventWriter<PlayerLevelUpEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in death_events.read() {
        // Grant experience to all players (for potential multiplayer support)
        for (player_entity, mut exp, perk_bonuses) in player_query.iter_mut() {
            // Apply exp multiplier from FastLearner perk
            let exp_amount = (event.experience as f32 * perk_bonuses.exp_multiplier) as u32;
            let leveled_up = exp.add(exp_amount);

            if leveled_up {
                level_up_events.send(PlayerLevelUpEvent {
                    player_entity,
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
