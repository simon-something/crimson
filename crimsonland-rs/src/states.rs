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
    /// Quest selection screen
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
    /// Transitioning between waves
    WaveTransition,
    /// Boss encounter
    BossEncounter,
}

/// Plugin for game state management
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<PlayingState>()
            .insert_resource(LoadingState::default())
            .add_systems(OnEnter(GameState::Loading), start_loading)
            .add_systems(
                Update,
                check_loading_complete.run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu_state)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu_state)
            .add_systems(OnEnter(GameState::QuestSelect), setup_quest_select)
            .add_systems(OnExit(GameState::QuestSelect), cleanup_quest_select)
            .add_systems(OnEnter(GameState::Playing), setup_playing_state)
            .add_systems(OnExit(GameState::Playing), cleanup_playing_state)
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                handle_unpause_input.run_if(in_state(GameState::Paused)),
            )
            // Sub-state systems
            .add_systems(OnEnter(PlayingState::WaveTransition), on_wave_transition_enter)
            .add_systems(OnExit(PlayingState::WaveTransition), on_wave_transition_exit)
            .add_systems(OnEnter(PlayingState::BossEncounter), on_boss_encounter_enter)
            .add_systems(OnExit(PlayingState::BossEncounter), on_boss_encounter_exit)
            .add_systems(
                Update,
                update_wave_transition.run_if(in_state(PlayingState::WaveTransition)),
            )
            .add_systems(
                Update,
                update_boss_encounter.run_if(in_state(PlayingState::BossEncounter)),
            );
    }
}

/// Resource to track loading progress
#[derive(Resource, Default)]
pub struct LoadingState {
    /// Whether game assets (sprites, sounds) are loaded
    pub assets_loaded: bool,
    /// Whether game configuration (weapons, creatures, perks) is loaded
    pub config_loaded: bool,
    /// Frame counter for simulated loading (real games would track actual asset handles)
    frames_waited: u32,
}

impl LoadingState {
    /// Check if all loading is complete
    pub fn is_complete(&self) -> bool {
        self.assets_loaded && self.config_loaded
    }

    /// Mark assets as loaded
    pub fn mark_assets_loaded(&mut self) {
        self.assets_loaded = true;
    }

    /// Mark config as loaded
    pub fn mark_config_loaded(&mut self) {
        self.config_loaded = true;
    }
}

/// Resource for wave transition state
#[derive(Resource, Default)]
pub struct WaveTransitionState {
    /// Timer for transition duration
    pub timer: f32,
    /// Next wave number
    pub next_wave: u32,
    /// Whether transition is complete
    pub complete: bool,
}

/// Resource for boss encounter state
#[derive(Resource)]
pub struct BossEncounterState {
    /// Name of the boss for display
    pub boss_name: String,
    /// Whether the boss intro has played
    pub intro_complete: bool,
}

impl Default for BossEncounterState {
    fn default() -> Self {
        Self {
            boss_name: "Unknown Boss".to_string(),
            intro_complete: false,
        }
    }
}

fn start_loading(mut loading_state: ResMut<LoadingState>) {
    // Reset loading state
    loading_state.assets_loaded = false;
    loading_state.config_loaded = false;
    loading_state.frames_waited = 0;
    info!("Starting game loading...");
}

fn check_loading_complete(
    mut loading_state: ResMut<LoadingState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Simulate loading progress (real game would check actual asset handles)
    loading_state.frames_waited += 1;

    // Assets "load" after 2 frames
    if loading_state.frames_waited >= 2 && !loading_state.assets_loaded {
        loading_state.mark_assets_loaded();
        info!("Assets loaded");
    }

    // Config "loads" after 3 frames
    if loading_state.frames_waited >= 3 && !loading_state.config_loaded {
        loading_state.mark_config_loaded();
        info!("Config loaded");
    }

    // Transition when complete
    if loading_state.is_complete() {
        info!("Loading complete, transitioning to main menu");
        next_state.set(GameState::MainMenu);
    }
}

fn setup_main_menu_state() {
    info!("Entering main menu");
}

fn cleanup_main_menu_state() {
    info!("Leaving main menu");
}

fn setup_quest_select(quest_db: Res<crate::quests::QuestDatabase>) {
    info!("Entering quest selection");
    // Display available quests using get_by_index
    for i in 0..10 {
        if let Some(quest) = quest_db.get_by_index(i) {
            info!("  Quest {}: {} - {}", i + 1, quest.name, quest.description);
        }
    }
}

fn cleanup_quest_select() {
    info!("Leaving quest selection");
}

fn setup_playing_state() {
    info!("Starting gameplay");
}

fn cleanup_playing_state() {
    info!("Ending gameplay");
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

// Wave transition systems
fn on_wave_transition_enter(mut commands: Commands) {
    commands.insert_resource(WaveTransitionState::default());
    info!("Wave transition started");
}

fn on_wave_transition_exit(mut commands: Commands) {
    commands.remove_resource::<WaveTransitionState>();
    info!("Wave transition ended");
}

fn update_wave_transition(
    time: Res<Time>,
    mut transition: ResMut<WaveTransitionState>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    transition.timer += time.delta_seconds();

    // 3 second transition between waves
    const TRANSITION_DURATION: f32 = 3.0;

    if transition.timer >= TRANSITION_DURATION && !transition.complete {
        transition.complete = true;
        next_state.set(PlayingState::Active);
        info!("Wave {} starting!", transition.next_wave);
    }
}

// Boss encounter systems
fn on_boss_encounter_enter(
    mut commands: Commands,
    pending_boss: Option<Res<PendingBossEncounter>>,
) {
    let boss_name = pending_boss
        .map(|p| p.boss_name.clone())
        .unwrap_or_else(|| "Unknown Boss".to_string());

    commands.insert_resource(BossEncounterState {
        boss_name: boss_name.clone(),
        intro_complete: false,
    });
    commands.remove_resource::<PendingBossEncounter>();
    info!("Boss encounter started: {}", boss_name);
}

fn on_boss_encounter_exit(mut commands: Commands, boss_state: Option<Res<BossEncounterState>>) {
    if let Some(state) = boss_state {
        info!("Boss encounter ended: {} (intro completed: {})", state.boss_name, state.intro_complete);
    }
    commands.remove_resource::<BossEncounterState>();
}

/// Update boss encounter intro animation
pub fn update_boss_encounter(
    time: Res<Time>,
    mut boss_state: ResMut<BossEncounterState>,
) {
    // Boss intro takes 2 seconds
    const INTRO_DURATION: f32 = 2.0;

    if !boss_state.intro_complete {
        // In a real game, this would be checked against an animation timer
        // For now, we mark it complete after a brief delay
        static mut INTRO_TIMER: f32 = 0.0;
        // SAFETY: Single-threaded Bevy systems
        unsafe {
            INTRO_TIMER += time.delta_seconds();
            if INTRO_TIMER >= INTRO_DURATION {
                boss_state.intro_complete = true;
                INTRO_TIMER = 0.0;
                info!("Boss intro complete for: {}", boss_state.boss_name);
            }
        }
    }
}

/// Resource for pending boss encounter (set before transitioning)
#[derive(Resource)]
pub struct PendingBossEncounter {
    pub boss_name: String,
}

/// Trigger a wave transition (call from quest/survival systems)
pub fn trigger_wave_transition(
    commands: &mut Commands,
    next_state: &mut ResMut<NextState<PlayingState>>,
    wave_number: u32,
) {
    commands.insert_resource(WaveTransitionState {
        timer: 0.0,
        next_wave: wave_number,
        complete: false,
    });
    next_state.set(PlayingState::WaveTransition);
    info!("Triggering transition to wave {}", wave_number);
}

/// Trigger a boss encounter (call from quest systems)
pub fn trigger_boss_encounter(
    commands: &mut Commands,
    next_state: &mut ResMut<NextState<PlayingState>>,
    boss_name: &str,
) {
    commands.insert_resource(PendingBossEncounter {
        boss_name: boss_name.to_string(),
    });
    next_state.set(PlayingState::BossEncounter);
    info!("Triggering boss encounter: {}", boss_name);
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
    fn loading_state_tracks_progress() {
        let mut state = LoadingState::default();
        assert!(!state.is_complete());

        state.mark_assets_loaded();
        assert!(!state.is_complete());

        state.mark_config_loaded();
        assert!(state.is_complete());
    }

    #[test]
    fn wave_transition_state_defaults() {
        let state = WaveTransitionState::default();
        assert_eq!(state.timer, 0.0);
        assert!(!state.complete);
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
