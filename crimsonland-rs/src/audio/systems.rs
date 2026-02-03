//! Audio systems

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::{AudioSettings, PlaySoundEvent, SoundEffect};
use crate::creatures::systems::CreatureDeathEvent;
use crate::player::systems::{PlayerDamageEvent, PlayerLevelUpEvent};
use crate::weapons::systems::ProjectileHitEvent;

/// Resource to track current music
#[derive(Resource, Default)]
pub struct CurrentMusic {
    pub handle: Option<Handle<AudioInstance>>,
}

/// Starts menu music
pub fn start_menu_music(
    _audio: Res<Audio>,
    _settings: Res<AudioSettings>,
    mut _current: ResMut<CurrentMusic>,
) {
    // TODO: Load and play menu music
    // let handle = audio.play(asset_server.load("audio/menu_music.ogg"))
    //     .with_volume(settings.effective_music_volume())
    //     .looped()
    //     .handle();
    // current.handle = Some(handle);
}

/// Stops menu music
pub fn stop_menu_music(mut current: ResMut<CurrentMusic>, mut audio_instances: ResMut<Assets<AudioInstance>>) {
    if let Some(handle) = current.handle.take() {
        if let Some(instance) = audio_instances.get_mut(&handle) {
            instance.stop(AudioTween::default());
        }
    }
}

/// Starts game music
pub fn start_game_music(
    _audio: Res<Audio>,
    _settings: Res<AudioSettings>,
    mut _current: ResMut<CurrentMusic>,
) {
    // TODO: Load and play game music
}

/// Stops game music
pub fn stop_game_music(mut current: ResMut<CurrentMusic>, mut audio_instances: ResMut<Assets<AudioInstance>>) {
    if let Some(handle) = current.handle.take() {
        if let Some(instance) = audio_instances.get_mut(&handle) {
            instance.stop(AudioTween::default());
        }
    }
}

/// Plays sound effects based on game events
pub fn play_sound_effects(
    _audio: Res<Audio>,
    _settings: Res<AudioSettings>,
    mut creature_deaths: EventReader<CreatureDeathEvent>,
    mut player_damage: EventReader<PlayerDamageEvent>,
    mut player_levelups: EventReader<PlayerLevelUpEvent>,
    mut projectile_hits: EventReader<ProjectileHitEvent>,
    mut _sound_events: EventReader<PlaySoundEvent>,
) {
    // Process creature deaths
    for _event in creature_deaths.read() {
        // TODO: Play creature death sound
        // play_sfx(&audio, &settings, SoundEffect::CreatureDeath);
    }

    // Process player damage
    for _event in player_damage.read() {
        // TODO: Play player hurt sound
        // play_sfx(&audio, &settings, SoundEffect::PlayerHurt);
    }

    // Process level ups
    for _event in player_levelups.read() {
        // TODO: Play level up sound
        // play_sfx(&audio, &settings, SoundEffect::LevelUp);
    }

    // Process projectile hits
    for _event in projectile_hits.read() {
        // TODO: Play bullet hit sound
        // play_sfx(&audio, &settings, SoundEffect::BulletHit);
    }
}

/// Helper to play a sound effect
#[allow(dead_code)]
fn play_sfx(_audio: &Audio, _settings: &AudioSettings, _sound: SoundEffect) {
    // TODO: Load and play sound effect
    // let path = match sound {
    //     SoundEffect::PistolFire => "audio/pistol.ogg",
    //     SoundEffect::ShotgunFire => "audio/shotgun.ogg",
    //     // ... etc
    // };
    // audio.play(asset_server.load(path))
    //     .with_volume(settings.effective_sfx_volume());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_music_default_is_none() {
        let music = CurrentMusic::default();
        assert!(music.handle.is_none());
    }
}
