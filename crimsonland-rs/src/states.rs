//! Game state management
//!
//! Defines the main game states and transitions.

use bevy::prelude::*;

/// The main game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    /// Loading assets
    #[default]
    Loading,
    /// Main menu
    MainMenu,
    /// Quest selection screen (future feature)
    #[allow(dead_code)]
    QuestSelect,
    /// Actively playing
    Playing,
    /// Game is paused
    Paused,
    /// Perk selection screen (on level up)
    PerkSelect,
    /// Game over screen
    GameOver,
    /// Victory screen
    Victory,
}

/// Sub-states for the Playing state
#[derive(SubStates, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[source(GameState = GameState::Playing)]
pub enum PlayingState {
    /// Normal gameplay
    #[default]
    Active,
    /// Transitioning between waves (future feature)
    #[allow(dead_code)]
    WaveTransition,
    /// Boss encounter (future feature)
    #[allow(dead_code)]
    BossEncounter,
}

/// Plugin for game state management
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<PlayingState>()
            .add_systems(OnEnter(GameState::Loading), start_loading)
            .add_systems(
                Update,
                check_loading_complete.run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu_state)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu_state)
            .add_systems(OnEnter(GameState::Playing), setup_playing_state)
            .add_systems(OnExit(GameState::Playing), cleanup_playing_state)
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                handle_unpause_input.run_if(in_state(GameState::Paused)),
            );
    }
}

/// Resource to track loading progress
#[derive(Resource, Default)]
#[allow(dead_code)]
pub struct LoadingState {
    pub assets_loaded: bool,
    pub config_loaded: bool,
}

fn start_loading(mut commands: Commands) {
    commands.insert_resource(LoadingState::default());
    // TODO: Start loading assets here
}

fn check_loading_complete(
    loading_state: Option<Res<LoadingState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // For now, immediately transition to main menu
    // Later this will check actual loading progress
    if loading_state.is_some() {
        next_state.set(GameState::MainMenu);
    }
}

fn setup_main_menu_state() {
    // Main menu setup handled by UI module
}

fn cleanup_main_menu_state() {
    // Main menu cleanup handled by UI module
}

fn setup_playing_state() {
    // Playing state setup handled by various game modules
}

fn cleanup_playing_state() {
    // Playing state cleanup handled by various game modules
}

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn handle_unpause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_state_default_is_loading() {
        assert_eq!(GameState::default(), GameState::Loading);
    }

    #[test]
    fn playing_state_default_is_active() {
        assert_eq!(PlayingState::default(), PlayingState::Active);
    }

    #[test]
    fn game_states_are_distinct() {
        let states = [
            GameState::Loading,
            GameState::MainMenu,
            GameState::QuestSelect,
            GameState::Playing,
            GameState::Paused,
            GameState::PerkSelect,
            GameState::GameOver,
            GameState::Victory,
        ];

        for (i, a) in states.iter().enumerate() {
            for (j, b) in states.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }
}
