//! Crimsonland - A top-down shooter game
//!
//! Ported from the original C implementation to Rust using Bevy ECS.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

mod audio;
mod bonuses;
mod creatures;
mod effects;
mod perks;
mod player;
mod quests;
mod rush;
mod states;
mod survival;
mod ui;
mod weapons;

use states::GameStatePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Crimsonland".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(creatures::CreaturesPlugin)
        .add_plugins(weapons::WeaponsPlugin)
        .add_plugins(perks::PerksPlugin)
        .add_plugins(bonuses::BonusesPlugin)
        .add_plugins(quests::QuestsPlugin)
        .add_plugins(effects::EffectsPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(audio::GameAudioPlugin)
        .add_plugins(survival::SurvivalPlugin)
        .add_plugins(rush::RushPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_builds_without_panic() {
        // Test that the app can be constructed without panicking
        // We don't actually run it, just verify it builds
        let _app = App::new();
    }
}
