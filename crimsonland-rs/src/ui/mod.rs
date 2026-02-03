//! UI module
//!
//! Handles all user interface elements: menus, HUD, and overlays.

mod hud;
mod menus;
mod perk_select;

pub use hud::*;
pub use menus::*;
pub use perk_select::*;

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for UI functionality
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Main menu
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(
                Update,
                handle_main_menu_input.run_if(in_state(GameState::MainMenu)),
            )
            // HUD
            .add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(OnExit(GameState::Playing), (cleanup_hud, cleanup_creature_health_bars))
            .add_systems(
                Update,
                (
                    update_hud,
                    update_hud_perks,
                    update_hud_game_mode,
                    spawn_creature_health_bars,
                    update_creature_health_bars,
                    cleanup_creature_health_bars,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Pause menu
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_pause_menu)
            .add_systems(
                Update,
                handle_pause_menu_input.run_if(in_state(GameState::Paused)),
            )
            // Perk selection
            .add_systems(OnEnter(GameState::PerkSelect), setup_perk_select)
            .add_systems(OnExit(GameState::PerkSelect), cleanup_perk_select)
            .add_systems(
                Update,
                handle_perk_select_input.run_if(in_state(GameState::PerkSelect)),
            )
            // Game over
            .add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(
                Update,
                handle_game_over_input.run_if(in_state(GameState::GameOver)),
            )
            // Victory
            .add_systems(OnEnter(GameState::Victory), setup_victory)
            .add_systems(OnExit(GameState::Victory), cleanup_victory)
            .add_systems(
                Update,
                handle_victory_input.run_if(in_state(GameState::Victory)),
            );
    }
}

/// Marker component for UI elements that should be cleaned up with their state
#[derive(Component)]
pub struct StateUi;

/// Marker for main menu UI
#[derive(Component)]
pub struct MainMenuUi;

/// Marker for pause menu UI
#[derive(Component)]
pub struct PauseMenuUi;

/// Marker for game over UI
#[derive(Component)]
pub struct GameOverUi;

/// Marker for victory UI
#[derive(Component)]
pub struct VictoryUi;

/// Helper function to create a text style
pub fn text_style(font_size: f32, color: Color) -> TextStyle {
    TextStyle {
        font_size,
        color,
        ..default()
    }
}

/// Helper function to create centered text
pub fn centered_text(text: &str, font_size: f32, color: Color) -> TextBundle {
    TextBundle::from_section(text, text_style(font_size, color)).with_style(Style {
        margin: UiRect::all(Val::Px(10.0)),
        ..default()
    })
}
