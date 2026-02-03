//! Menu screens

use bevy::prelude::*;

use super::{text_style, GameOverUi, MainMenuUi, PauseMenuUi, VictoryUi};
use crate::quests::database::QuestId;
use crate::quests::systems::ActiveQuest;
use crate::states::GameState;

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
                "Press ENTER to Start Quest Mode",
                text_style(28.0, Color::WHITE),
            ));

            parent.spawn(TextBundle::from_section(
                "Press S for Survival Mode",
                text_style(28.0, Color::srgb(0.7, 0.7, 0.7)),
            ));

            parent.spawn(TextBundle::from_section(
                "Press ESC to Quit",
                text_style(28.0, Color::srgb(0.7, 0.7, 0.7)),
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
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut active_quest: ResMut<ActiveQuest>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        // Start quest mode with first quest
        active_quest.quest_id = Some(QuestId::Q01LandHostile);
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        // Survival mode (no specific quest)
        active_quest.quest_id = None;
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

/// Sets up the pause menu
pub fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            PauseMenuUi,
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

            parent.spawn(TextBundle::from_section(
                "Press ESC to Resume",
                text_style(24.0, Color::srgb(0.7, 0.7, 0.7)),
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
pub fn setup_game_over(mut commands: Commands) {
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
                    height: Val::Px(50.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                "Press ENTER to Retry",
                text_style(28.0, Color::WHITE),
            ));

            parent.spawn(TextBundle::from_section(
                "Press ESC to Return to Menu",
                text_style(28.0, Color::srgb(0.7, 0.7, 0.7)),
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
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

/// Sets up the victory screen
pub fn setup_victory(mut commands: Commands) {
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
                "VICTORY!",
                TextStyle {
                    font_size: 64.0,
                    color: Color::srgb(0.2, 0.9, 0.2),
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

            parent.spawn(TextBundle::from_section(
                "Quest Complete!",
                text_style(28.0, Color::WHITE),
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                "Press ENTER to Continue",
                text_style(24.0, Color::srgb(0.7, 0.7, 0.7)),
            ));

            parent.spawn(TextBundle::from_section(
                "Press ESC to Return to Menu",
                text_style(24.0, Color::srgb(0.7, 0.7, 0.7)),
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
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        // TODO: Progress to next quest
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
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
