//! In-game HUD

use bevy::prelude::*;

use crate::player::components::{Experience, Health, Player};
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

            // Bottom bar (weapon, ammo)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
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
        });
}

/// Cleans up the HUD
pub fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Updates HUD elements based on player state
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

    // Update ammo text
    if let Ok(mut text) = ammo_text_query.get_single_mut() {
        text.sections[0].value = match weapon.ammo {
            Some(ammo) => format!("{}", ammo),
            None => "∞".into(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hud_root_is_component() {
        let _root = HudRoot;
    }
}
