//! Menu screens

use bevy::prelude::*;

use super::{centered_text, text_style, GameOverUi, MainMenuUi, PauseMenuUi, StateUi, VictoryUi};
use crate::audio::{PlaySoundEvent, SoundEffect};
use crate::quests::database::QuestId;
use crate::quests::systems::{ActiveQuest, QuestProgress};
use crate::rush::RushState;
use crate::states::GameState;
use crate::survival::SurvivalState;

/// Marker for stats text on end screens
#[derive(Component)]
pub struct EndScreenStats;

/// Sets up the main menu
pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenuUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.1, 0.05, 0.05)),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "CRIMSONLAND",
                TextStyle {
                    font_size: 72.0,
                    color: Color::srgb(0.8, 0.1, 0.1),
                    ..default()
                },
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(50.0),
                    ..default()
                },
                ..default()
            });

            // Menu options
            parent.spawn(TextBundle::from_section(
                "[ENTER] Quest Mode - Story missions",
                text_style(24.0, Color::WHITE),
            ));

            parent.spawn(TextBundle::from_section(
                "[S] Survival Mode - Endless waves",
                text_style(24.0, Color::srgb(0.7, 0.9, 0.7)),
            ));

            parent.spawn(TextBundle::from_section(
                "[R] Rush Mode - Timed challenge",
                text_style(24.0, Color::srgb(0.9, 0.7, 0.7)),
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                "[ESC] Quit",
                text_style(20.0, Color::srgb(0.5, 0.5, 0.5)),
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(100.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                "Rust/Bevy Port",
                text_style(16.0, Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

/// Cleans up the main menu
pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles main menu input
pub fn handle_main_menu_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut active_quest: ResMut<ActiveQuest>,
    mut exit: EventWriter<AppExit>,
    mut sound_events: EventWriter<PlaySoundEvent>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        // Start quest mode with first quest using ActiveQuest::new
        sound_events.send(PlaySoundEvent {
            sound: SoundEffect::MenuSelect,
            position: None,
        });
        *active_quest = ActiveQuest::new(QuestId::Q01LandHostile);
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        // Survival mode (no specific quest)
        sound_events.send(PlaySoundEvent {
            sound: SoundEffect::MenuSelect,
            position: None,
        });
        active_quest.quest_id = None;
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        // Rush mode - 2 minute timed challenge
        sound_events.send(PlaySoundEvent {
            sound: SoundEffect::MenuSelect,
            position: None,
        });
        active_quest.quest_id = None;

        // Select a loadout from available_loadouts (use first one for now)
        // In a full implementation, this would go to a loadout selection screen
        let loadouts = crate::rush::available_loadouts();
        let selected_loadout = loadouts.into_iter().next().unwrap_or_default();

        // Log the loadout selection
        info!("Starting Rush mode with loadout: {} (weapon: {:?}, perks: {:?})",
            selected_loadout.name, selected_loadout.weapon, selected_loadout.perks);

        commands.insert_resource(RushState::new(120.0, selected_loadout));
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        sound_events.send(PlaySoundEvent {
            sound: SoundEffect::MenuBack,
            position: None,
        });
        exit.send(AppExit::Success);
    }
}

/// Sets up the pause menu
pub fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            PauseMenuUi,
            StateUi, // Generic marker for state-based UI cleanup
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font_size: 48.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(centered_text(
                "Press ESC to Resume",
                24.0,
                Color::srgb(0.7, 0.7, 0.7),
            ));

            parent.spawn(TextBundle::from_section(
                "Press Q to Quit to Menu",
                text_style(24.0, Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

/// Cleans up the pause menu
pub fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles pause menu input
pub fn handle_pause_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // ESC to unpause is handled in states module

    if keyboard.just_pressed(KeyCode::KeyQ) {
        next_state.set(GameState::MainMenu);
    }
}

/// Sets up the game over screen
pub fn setup_game_over(
    mut commands: Commands,
    survival_state: Option<Res<SurvivalState>>,
    rush_state: Option<Res<RushState>>,
    quest_progress: Option<Res<QuestProgress>>,
) {
    // Gather stats from the current game mode
    let (time_str, kills_str, extra_str) = if let Some(ref rush) = rush_state {
        let mins = (rush.round_duration - rush.time_remaining) as u32 / 60;
        let secs = (rush.round_duration - rush.time_remaining) as u32 % 60;
        (
            format!("Time: {}:{:02}", mins, secs),
            format!("Kills: {}", rush.total_kills),
            format!("Score: {}", rush.score),
        )
    } else if let Some(ref survival) = survival_state {
        let mins = survival.game_time as u32 / 60;
        let secs = survival.game_time as u32 % 60;
        (
            format!("Time: {}:{:02}", mins, secs),
            format!("Kills: {}", survival.kills),
            String::new(),
        )
    } else if let Some(ref progress) = quest_progress {
        let mins = progress.total_time as u32 / 60;
        let secs = progress.total_time as u32 % 60;
        (
            format!("Time: {}:{:02}", mins, secs),
            format!("Kills: {}", progress.kills),
            format!("Wave: {}", progress.current_wave + 1),
        )
    } else {
        (String::new(), String::new(), String::new())
    };

    commands
        .spawn((
            GameOverUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.2, 0.0, 0.0, 0.9)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font_size: 64.0,
                    color: Color::srgb(0.8, 0.1, 0.1),
                    ..default()
                },
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            // Stats
            if !time_str.is_empty() {
                parent.spawn((
                    EndScreenStats,
                    TextBundle::from_section(&time_str, text_style(24.0, Color::srgb(0.8, 0.8, 0.8))),
                ));
            }
            if !kills_str.is_empty() {
                parent.spawn((
                    EndScreenStats,
                    TextBundle::from_section(&kills_str, text_style(24.0, Color::srgb(0.8, 0.8, 0.8))),
                ));
            }
            if !extra_str.is_empty() {
                parent.spawn((
                    EndScreenStats,
                    TextBundle::from_section(&extra_str, text_style(24.0, Color::srgb(1.0, 0.9, 0.5))),
                ));
            }

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                "[ENTER] Retry",
                text_style(24.0, Color::WHITE),
            ));

            parent.spawn(TextBundle::from_section(
                "[ESC] Return to Menu",
                text_style(20.0, Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}

/// Cleans up the game over screen
pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles game over input
pub fn handle_game_over_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sound_events: EventWriter<PlaySoundEvent>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        sound_events.send(PlaySoundEvent {
            sound: SoundEffect::MenuSelect,
            position: None,
        });
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        sound_events.send(PlaySoundEvent {
            sound: SoundEffect::MenuBack,
            position: None,
        });
        next_state.set(GameState::MainMenu);
    }
}

/// Sets up the victory screen
pub fn setup_victory(
    mut commands: Commands,
    quest_progress: Option<Res<QuestProgress>>,
    rush_state: Option<Res<RushState>>,
) {
    // Gather stats
    let (title, time_str, kills_str, extra_str) = if let Some(ref rush) = rush_state {
        let mins = rush.round_duration as u32 / 60;
        let secs = rush.round_duration as u32 % 60;
        (
            "RUSH COMPLETE!",
            format!("Time: {}:{:02}", mins, secs),
            format!("Kills: {}", rush.total_kills),
            format!("Final Score: {}", rush.score),
        )
    } else if let Some(ref progress) = quest_progress {
        let mins = progress.total_time as u32 / 60;
        let secs = progress.total_time as u32 % 60;
        (
            "QUEST COMPLETE!",
            format!("Time: {}:{:02}", mins, secs),
            format!("Total Kills: {}", progress.kills),
            String::new(),
        )
    } else {
        ("VICTORY!", String::new(), String::new(), String::new())
    };

    commands
        .spawn((
            VictoryUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.1, 0.0, 0.9)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font_size: 64.0,
                    color: Color::srgb(0.2, 0.9, 0.2),
                    ..default()
                },
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            // Stats
            if !time_str.is_empty() {
                parent.spawn((
                    EndScreenStats,
                    TextBundle::from_section(&time_str, text_style(24.0, Color::srgb(0.8, 0.8, 0.8))),
                ));
            }
            if !kills_str.is_empty() {
                parent.spawn((
                    EndScreenStats,
                    TextBundle::from_section(&kills_str, text_style(24.0, Color::srgb(0.8, 0.8, 0.8))),
                ));
            }
            if !extra_str.is_empty() {
                parent.spawn((
                    EndScreenStats,
                    TextBundle::from_section(
                        &extra_str,
                        text_style(28.0, Color::srgb(1.0, 0.9, 0.3)),
                    ),
                ));
            }

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                "[ENTER] Continue",
                text_style(24.0, Color::WHITE),
            ));

            parent.spawn(TextBundle::from_section(
                "[ESC] Return to Menu",
                text_style(20.0, Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}

/// Cleans up the victory screen
pub fn cleanup_victory(mut commands: Commands, query: Query<Entity, With<VictoryUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles victory screen input
pub fn handle_victory_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sound_events: EventWriter<PlaySoundEvent>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        // Progress to next quest (or replay)
        sound_events.send(PlaySoundEvent {
            sound: SoundEffect::MenuSelect,
            position: None,
        });
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        sound_events.send(PlaySoundEvent {
            sound: SoundEffect::MenuBack,
            position: None,
        });
        next_state.set(GameState::MainMenu);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_menu_ui_is_component() {
        let _ui = MainMenuUi;
    }
}
