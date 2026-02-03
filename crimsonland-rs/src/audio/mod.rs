//! Audio module
//!
//! Handles sound effects and music.

pub mod systems;

pub use systems::*;

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for audio functionality
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioSettings>()
            .init_resource::<CurrentMusic>()
            .add_systems(OnEnter(GameState::MainMenu), start_menu_music)
            .add_systems(OnExit(GameState::MainMenu), stop_menu_music)
            .add_systems(OnEnter(GameState::Playing), start_game_music)
            .add_systems(OnExit(GameState::Playing), stop_game_music)
            .add_systems(Update, play_sound_effects.run_if(in_state(GameState::Playing)));
    }
}

/// Audio settings
#[derive(Resource, Debug, Clone)]
pub struct AudioSettings {
    pub master_volume: f64,
    pub music_volume: f64,
    pub sfx_volume: f64,
    pub music_enabled: bool,
    pub sfx_enabled: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 1.0,
            music_enabled: true,
            sfx_enabled: true,
        }
    }
}

impl AudioSettings {
    pub fn effective_music_volume(&self) -> f64 {
        if self.music_enabled {
            self.master_volume * self.music_volume
        } else {
            0.0
        }
    }

    pub fn effective_sfx_volume(&self) -> f64 {
        if self.sfx_enabled {
            self.master_volume * self.sfx_volume
        } else {
            0.0
        }
    }
}

/// Sound effect types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEffect {
    // Weapons
    PistolFire,
    ShotgunFire,
    RifleFire,
    RocketFire,
    PlasmaFire,

    // Impacts
    BulletHit,
    Explosion,

    // Creatures
    CreatureDeath,
    CreatureSpawn,

    // Player
    PlayerHurt,
    PlayerDeath,
    LevelUp,

    // Pickups
    HealthPickup,
    WeaponPickup,
    BonusPickup,

    // UI
    MenuSelect,
    MenuBack,
}

/// Event to play a sound effect
#[derive(Event)]
pub struct PlaySoundEvent {
    pub sound: SoundEffect,
    pub position: Option<Vec2>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_settings_default_has_sound() {
        let settings = AudioSettings::default();
        assert!(settings.music_enabled);
        assert!(settings.sfx_enabled);
        assert!(settings.master_volume > 0.0);
    }

    #[test]
    fn audio_settings_effective_volume_respects_mute() {
        let mut settings = AudioSettings::default();
        assert!(settings.effective_music_volume() > 0.0);

        settings.music_enabled = false;
        assert_eq!(settings.effective_music_volume(), 0.0);
    }

    #[test]
    fn audio_settings_effective_volume_respects_master() {
        let mut settings = AudioSettings::default();
        settings.master_volume = 0.5;
        settings.music_volume = 1.0;

        assert!((settings.effective_music_volume() - 0.5).abs() < 0.001);
    }
}
