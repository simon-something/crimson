//! In-game HUD

use bevy::prelude::*;

use crate::creatures::components::{Creature, CreatureHealth};
use crate::perks::components::PerkInventory;
use crate::player::components::{Experience, Health, Invincibility, Player};
use crate::quests::systems::{ActiveQuest, QuestProgress};
use crate::rush::RushState;
use crate::survival::SurvivalState;
use crate::weapons::components::EquippedWeapon;

/// Marker for HUD root
#[derive(Component)]
pub struct HudRoot;

/// Marker for health bar
#[derive(Component)]
pub struct HealthBar;

/// Marker for health text
#[derive(Component)]
pub struct HealthText;

/// Marker for experience bar
#[derive(Component)]
pub struct ExperienceBar;

/// Marker for level text
#[derive(Component)]
pub struct LevelText;

/// Marker for ammo text
#[derive(Component)]
pub struct AmmoText;

/// Marker for weapon name text
#[derive(Component)]
pub struct WeaponText;

/// Marker for kill counter text
#[derive(Component)]
pub struct KillCounterText;

/// Marker for game timer text
#[derive(Component)]
pub struct GameTimerText;

/// Marker for wave/progress indicator
#[derive(Component)]
pub struct WaveProgressText;

/// Marker for perk count indicator
#[derive(Component)]
pub struct PerkCountText;

/// Marker for invincibility indicator
#[derive(Component)]
pub struct InvincibilityIndicator;

/// Marker for creature health bar (world-space sprite)
#[derive(Component)]
pub struct CreatureHealthBar {
    /// The creature entity this health bar belongs to
    pub creature: Entity,
}

/// Marker for health bar background
#[derive(Component)]
pub struct CreatureHealthBarBackground;

/// Sets up the HUD
pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            HudRoot,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            // Top bar (health, exp)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                    ..default()
                })
                .with_children(|parent| {
                    // Health section
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                HealthText,
                                TextBundle::from_section(
                                    "Health: 100/100",
                                    TextStyle {
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                            ));

                            // Health bar background
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(200.0),
                                        height: Val::Px(20.0),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgb(0.3, 0.0, 0.0)),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Health bar fill
                                    parent.spawn((
                                        HealthBar,
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::srgb(
                                                0.8, 0.1, 0.1,
                                            )),
                                            ..default()
                                        },
                                    ));
                                });
                        });

                    // Center stats section
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Game timer
                            parent.spawn((
                                GameTimerText,
                                TextBundle::from_section(
                                    "0:00",
                                    TextStyle {
                                        font_size: 28.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                            ));

                            // Wave/progress text
                            parent.spawn((
                                WaveProgressText,
                                TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font_size: 16.0,
                                        color: Color::srgb(0.8, 0.8, 0.5),
                                        ..default()
                                    },
                                ),
                            ));
                        });

                    // Level/XP section
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::End,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                LevelText,
                                TextBundle::from_section(
                                    "Level 1",
                                    TextStyle {
                                        font_size: 20.0,
                                        color: Color::srgb(0.5, 0.8, 1.0),
                                        ..default()
                                    },
                                ),
                            ));

                            // XP bar background
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(200.0),
                                        height: Val::Px(10.0),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.3)),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // XP bar fill
                                    parent.spawn((
                                        ExperienceBar,
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(0.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::srgb(
                                                0.3, 0.5, 1.0,
                                            )),
                                            ..default()
                                        },
                                    ));
                                });
                        });
                });

            // Bottom bar (weapon, ammo, kills, perks)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                    ..default()
                })
                .with_children(|parent| {
                    // Kill counter (left side)
                    parent.spawn((
                        KillCounterText,
                        TextBundle::from_section(
                            "Kills: 0",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgb(1.0, 0.5, 0.5),
                                ..default()
                            },
                        ),
                    ));

                    // Weapon section (center)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                WeaponText,
                                TextBundle::from_section(
                                    "Pistol",
                                    TextStyle {
                                        font_size: 24.0,
                                        color: Color::srgb(1.0, 0.8, 0.3),
                                        ..default()
                                    },
                                ),
                            ));

                            parent.spawn(TextBundle::from_section(
                                " - ",
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));

                            parent.spawn((
                                AmmoText,
                                TextBundle::from_section(
                                    "∞",
                                    TextStyle {
                                        font_size: 24.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                            ));
                        });

                    // Right side: perk count and power-up indicators
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Invincibility indicator (hidden by default)
                            parent.spawn((
                                InvincibilityIndicator,
                                TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font_size: 18.0,
                                        color: Color::srgb(1.0, 1.0, 0.3),
                                        ..default()
                                    },
                                ),
                            ));

                            // Perk count
                            parent.spawn((
                                PerkCountText,
                                TextBundle::from_section(
                                    "Perks: 0",
                                    TextStyle {
                                        font_size: 20.0,
                                        color: Color::srgb(0.6, 0.9, 0.6),
                                        ..default()
                                    },
                                ),
                            ));
                        });
                });
        });
}

/// Cleans up the HUD
pub fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Updates basic HUD elements (health, XP, level, weapon)
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn update_hud(
    player_query: Query<(&Health, &Experience, &EquippedWeapon), With<Player>>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
    mut health_text_query: Query<&mut Text, (With<HealthText>, Without<LevelText>)>,
    mut exp_bar_query: Query<&mut Style, (With<ExperienceBar>, Without<HealthBar>)>,
    mut level_text_query: Query<
        &mut Text,
        (
            With<LevelText>,
            Without<HealthText>,
            Without<AmmoText>,
            Without<WeaponText>,
        ),
    >,
    mut ammo_text_query: Query<
        &mut Text,
        (
            With<AmmoText>,
            Without<HealthText>,
            Without<LevelText>,
            Without<WeaponText>,
        ),
    >,
    mut weapon_text_query: Query<
        &mut Text,
        (
            With<WeaponText>,
            Without<HealthText>,
            Without<LevelText>,
            Without<AmmoText>,
        ),
    >,
    weapon_registry: Res<crate::weapons::registry::WeaponRegistry>,
) {
    let Ok((health, experience, weapon)) = player_query.get_single() else {
        return;
    };

    // Update health bar
    if let Ok(mut style) = health_bar_query.get_single_mut() {
        let percent = health.percentage() * 100.0;
        style.width = Val::Percent(percent);
    }

    // Update health text
    if let Ok(mut text) = health_text_query.get_single_mut() {
        text.sections[0].value = format!("Health: {:.0}/{:.0}", health.current, health.max);
    }

    // Update XP bar
    if let Ok(mut style) = exp_bar_query.get_single_mut() {
        let percent = experience.progress() * 100.0;
        style.width = Val::Percent(percent);
    }

    // Update level text
    if let Ok(mut text) = level_text_query.get_single_mut() {
        text.sections[0].value = format!("Level {}", experience.level);
    }

    // Update weapon name
    if let Ok(mut text) = weapon_text_query.get_single_mut() {
        if let Some(weapon_data) = weapon_registry.get(weapon.weapon_id) {
            text.sections[0].value = weapon_data.name.clone();
        }
    }

    // Update ammo text - use has_ammo() to check and color accordingly
    if let Ok(mut text) = ammo_text_query.get_single_mut() {
        let has_ammo = weapon.has_ammo();
        text.sections[0].value = match weapon.ammo {
            Some(ammo) => format!("{}", ammo),
            None => "∞".into(),
        };
        // Red text when out of ammo
        text.sections[0].style.color = if has_ammo {
            Color::WHITE
        } else {
            Color::srgb(1.0, 0.3, 0.3)
        };
    }
}

/// Updates perk count and invincibility indicator
#[allow(clippy::type_complexity)]
pub fn update_hud_perks(
    player_query: Query<(&PerkInventory, Option<&Invincibility>), With<Player>>,
    mut perk_text_query: Query<&mut Text, With<PerkCountText>>,
    mut invincibility_text_query: Query<
        &mut Text,
        (With<InvincibilityIndicator>, Without<PerkCountText>),
    >,
) {
    let Ok((perk_inventory, invincibility)) = player_query.get_single() else {
        return;
    };

    // Update perk count
    if let Ok(mut text) = perk_text_query.get_single_mut() {
        text.sections[0].value = format!("Perks: {}", perk_inventory.total_perks());
    }

    // Update invincibility indicator
    if let Ok(mut text) = invincibility_text_query.get_single_mut() {
        if let Some(inv) = invincibility {
            if inv.is_active() {
                text.sections[0].value = format!("SHIELD {:.1}s", inv.timer);
            } else {
                text.sections[0].value.clear();
            }
        } else {
            text.sections[0].value.clear();
        }
    }
}

/// Updates game mode specific HUD elements (timer, kills, wave)
#[allow(clippy::type_complexity)]
pub fn update_hud_game_mode(
    survival_state: Option<Res<SurvivalState>>,
    rush_state: Option<Res<RushState>>,
    quest_progress: Option<Res<QuestProgress>>,
    active_quest: Option<Res<ActiveQuest>>,
    mut kill_text_query: Query<&mut Text, With<KillCounterText>>,
    mut timer_text_query: Query<&mut Text, (With<GameTimerText>, Without<KillCounterText>)>,
    mut wave_text_query: Query<
        &mut Text,
        (
            With<WaveProgressText>,
            Without<GameTimerText>,
            Without<KillCounterText>,
        ),
    >,
) {
    // Update kill counter based on game mode
    if let Ok(mut text) = kill_text_query.get_single_mut() {
        if let Some(ref survival) = survival_state {
            text.sections[0].value = format!("Kills: {}", survival.kills);
        } else if let Some(ref rush) = rush_state {
            text.sections[0].value = format!("Kills: {} | Score: {}", rush.total_kills, rush.score);
        } else if let Some(ref progress) = quest_progress {
            text.sections[0].value = format!("Kills: {}", progress.kills);
        } else {
            text.sections[0].value = "Kills: 0".to_string();
        }
    }

    // Update game timer based on game mode
    if let Ok(mut text) = timer_text_query.get_single_mut() {
        if let Some(ref survival) = survival_state {
            let mins = (survival.game_time / 60.0) as u32;
            let secs = (survival.game_time % 60.0) as u32;
            text.sections[0].value = format!("{}:{:02}", mins, secs);
        } else if let Some(ref rush) = rush_state {
            let mins = (rush.time_remaining / 60.0) as u32;
            let secs = (rush.time_remaining % 60.0) as u32;
            // Change color based on time remaining
            text.sections[0].style.color = if rush.time_remaining < 10.0 {
                Color::srgb(1.0, 0.3, 0.3) // Red when low
            } else if rush.time_remaining < 30.0 {
                Color::srgb(1.0, 0.8, 0.3) // Yellow when medium
            } else {
                Color::WHITE
            };
            text.sections[0].value = format!("{}:{:02}", mins, secs);
        } else if let Some(ref progress) = quest_progress {
            let mins = (progress.total_time / 60.0) as u32;
            let secs = (progress.total_time % 60.0) as u32;
            text.sections[0].value = format!("{}:{:02}", mins, secs);
        } else {
            text.sections[0].value = "0:00".to_string();
        }
    }

    // Update wave/progress text based on game mode
    if let Ok(mut text) = wave_text_query.get_single_mut() {
        if survival_state.is_some() {
            text.sections[0].value = "SURVIVAL".to_string();
        } else if let Some(ref rush) = rush_state {
            let streak_text = if rush.kill_streak >= 5 {
                format!(" | x{:.1} STREAK", rush.streak_multiplier())
            } else {
                String::new()
            };
            text.sections[0].value = format!("RUSH{}", streak_text);
        } else if let Some(ref progress) = quest_progress {
            if active_quest
                .as_ref()
                .map(|q| q.quest_id.is_some())
                .unwrap_or(false)
            {
                text.sections[0].value = format!("Wave {}", progress.current_wave + 1);
            } else {
                text.sections[0].value.clear();
            }
        } else {
            text.sections[0].value.clear();
        }
    }
}

/// Spawns health bars above damaged creatures
#[allow(clippy::type_complexity)]
pub fn spawn_creature_health_bars(
    mut commands: Commands,
    creatures: Query<(Entity, &CreatureHealth), (With<Creature>, Without<CreatureHealthBar>)>,
    existing_bars: Query<&CreatureHealthBar>,
) {
    for (entity, health) in creatures.iter() {
        // Only spawn health bar if creature has taken damage
        if health.current < health.max {
            // Check if this creature already has a health bar
            let has_bar = existing_bars.iter().any(|bar| bar.creature == entity);
            if !has_bar {
                // Spawn health bar background (dark)
                commands.spawn((
                    CreatureHealthBarBackground,
                    CreatureHealthBar { creature: entity },
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgba(0.1, 0.1, 0.1, 0.8),
                            custom_size: Some(Vec2::new(32.0, 4.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 20.0, 10.0)),
                        ..default()
                    },
                ));

                // Spawn health bar fill (red/green based on percentage)
                commands.spawn((
                    CreatureHealthBar { creature: entity },
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.8, 0.2, 0.2),
                            custom_size: Some(Vec2::new(32.0 * health.percentage(), 4.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 20.0, 11.0)),
                        ..default()
                    },
                ));
            }
        }
    }
}

/// Updates creature health bar positions and sizes
#[allow(clippy::type_complexity)]
pub fn update_creature_health_bars(
    creatures: Query<(&Transform, &CreatureHealth), With<Creature>>,
    mut health_bars: Query<
        (&CreatureHealthBar, &mut Transform, &mut Sprite),
        (Without<Creature>, Without<CreatureHealthBarBackground>),
    >,
    mut backgrounds: Query<
        (&CreatureHealthBar, &mut Transform),
        (
            With<CreatureHealthBarBackground>,
            Without<Creature>,
        ),
    >,
) {
    // Update health bar fills
    for (bar, mut transform, mut sprite) in health_bars.iter_mut() {
        if let Ok((creature_transform, health)) = creatures.get(bar.creature) {
            // Position above creature
            transform.translation.x = creature_transform.translation.x;
            transform.translation.y = creature_transform.translation.y + 20.0;

            // Update width based on health percentage
            let percentage = health.percentage();
            if let Some(ref mut size) = sprite.custom_size {
                size.x = 32.0 * percentage;
            }

            // Color: green when healthy, yellow mid, red low
            sprite.color = if percentage > 0.6 {
                Color::srgb(0.2, 0.8, 0.2)
            } else if percentage > 0.3 {
                Color::srgb(0.8, 0.8, 0.2)
            } else {
                Color::srgb(0.8, 0.2, 0.2)
            };
        }
    }

    // Update background positions
    for (bar, mut transform) in backgrounds.iter_mut() {
        if let Ok((creature_transform, _)) = creatures.get(bar.creature) {
            transform.translation.x = creature_transform.translation.x;
            transform.translation.y = creature_transform.translation.y + 20.0;
        }
    }
}

/// Cleans up health bars when creatures die
pub fn cleanup_creature_health_bars(
    mut commands: Commands,
    creatures: Query<Entity, With<Creature>>,
    health_bars: Query<(Entity, &CreatureHealthBar)>,
) {
    for (bar_entity, bar) in health_bars.iter() {
        // If the creature no longer exists, despawn the health bar
        if creatures.get(bar.creature).is_err() {
            commands.entity(bar_entity).despawn_recursive();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hud_root_is_component() {
        let _root = HudRoot;
    }

    #[test]
    fn creature_health_bar_tracks_entity() {
        let bar = CreatureHealthBar {
            creature: Entity::PLACEHOLDER,
        };
        assert_eq!(bar.creature, Entity::PLACEHOLDER);
    }
}
