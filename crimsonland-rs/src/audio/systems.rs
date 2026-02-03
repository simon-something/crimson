//! Audio systems

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::{AudioSettings, PlaySoundEvent, SoundEffect};
use crate::bonuses::systems::BonusCollectedEvent;
use crate::bonuses::BonusType;
use crate::creatures::systems::CreatureDeathEvent;
use crate::player::systems::{PlayerDamageEvent, PlayerDeathEvent, PlayerLevelUpEvent};
use crate::weapons::components::WeaponId;
use crate::weapons::systems::{FireWeaponEvent, ProjectileHitEvent};

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
#[allow(clippy::too_many_arguments)]
pub fn play_sound_effects(
    audio: Res<Audio>,
    settings: Res<AudioSettings>,
    asset_server: Res<AssetServer>,
    mut creature_deaths: EventReader<CreatureDeathEvent>,
    mut player_damage: EventReader<PlayerDamageEvent>,
    mut player_deaths: EventReader<PlayerDeathEvent>,
    mut player_levelups: EventReader<PlayerLevelUpEvent>,
    mut weapon_fires: EventReader<FireWeaponEvent>,
    mut projectile_hits: EventReader<ProjectileHitEvent>,
    mut bonus_collected: EventReader<BonusCollectedEvent>,
    mut sound_events: EventReader<PlaySoundEvent>,
) {
    // Process weapon fire events
    for event in weapon_fires.read() {
        let sound = weapon_fire_sound(event.weapon_id);
        play_sfx(&audio, &settings, &asset_server, sound);
    }

    // Process creature deaths
    for _event in creature_deaths.read() {
        play_sfx(&audio, &settings, &asset_server, SoundEffect::CreatureDeath);
    }

    // Process player damage
    for _event in player_damage.read() {
        play_sfx(&audio, &settings, &asset_server, SoundEffect::PlayerHurt);
    }

    // Process player deaths
    for _event in player_deaths.read() {
        play_sfx(&audio, &settings, &asset_server, SoundEffect::PlayerDeath);
    }

    // Process level ups
    for _event in player_levelups.read() {
        play_sfx(&audio, &settings, &asset_server, SoundEffect::LevelUp);
    }

    // Process projectile hits
    for _event in projectile_hits.read() {
        play_sfx(&audio, &settings, &asset_server, SoundEffect::BulletHit);
    }

    // Process bonus pickups
    for event in bonus_collected.read() {
        let sound = bonus_pickup_sound(event.bonus_type);
        play_sfx(&audio, &settings, &asset_server, sound);
    }

    // Process direct sound effect events
    for event in sound_events.read() {
        play_sfx(&audio, &settings, &asset_server, event.sound);
    }
}

/// Maps weapon ID to sound effect
fn weapon_fire_sound(weapon_id: WeaponId) -> SoundEffect {
    match weapon_id {
        WeaponId::Pistol | WeaponId::Magnum | WeaponId::PocketRocket => SoundEffect::PistolFire,
        WeaponId::AssaultRifle | WeaponId::MachineGun | WeaponId::Minigun | WeaponId::Uzi
        | WeaponId::Smg | WeaponId::DualSmg => SoundEffect::RifleFire,
        WeaponId::Shotgun | WeaponId::DoubleBarrel | WeaponId::Jackhammer | WeaponId::GaussShotgun => {
            SoundEffect::ShotgunFire
        }
        WeaponId::RocketLauncher | WeaponId::GrenadeLauncher => SoundEffect::RocketFire,
        WeaponId::PlasmaRifle | WeaponId::IonRifle | WeaponId::PulseGun | WeaponId::GaussGun => {
            SoundEffect::PlasmaFire
        }
        _ => SoundEffect::RifleFire, // Default for other weapons
    }
}

/// Maps bonus type to sound effect
fn bonus_pickup_sound(bonus_type: BonusType) -> SoundEffect {
    match bonus_type {
        BonusType::SmallHealth | BonusType::LargeHealth | BonusType::FullHealth => {
            SoundEffect::HealthPickup
        }
        BonusType::WeaponPickup => SoundEffect::WeaponPickup,
        _ => SoundEffect::BonusPickup,
    }
}

/// Helper to play a sound effect
fn play_sfx(audio: &Audio, settings: &AudioSettings, asset_server: &AssetServer, sound: SoundEffect) {
    if !settings.sfx_enabled {
        return;
    }

    // Map sound effect to file path
    // NOTE: Audio files need to be placed in assets/audio/
    let path = match sound {
        SoundEffect::PistolFire => "audio/pistol.ogg",
        SoundEffect::ShotgunFire => "audio/shotgun.ogg",
        SoundEffect::RifleFire => "audio/rifle.ogg",
        SoundEffect::RocketFire => "audio/rocket.ogg",
        SoundEffect::PlasmaFire => "audio/plasma.ogg",
        SoundEffect::BulletHit => "audio/hit.ogg",
        SoundEffect::Explosion => "audio/explosion.ogg",
        SoundEffect::CreatureDeath => "audio/creature_death.ogg",
        SoundEffect::CreatureSpawn => "audio/creature_spawn.ogg",
        SoundEffect::PlayerHurt => "audio/player_hurt.ogg",
        SoundEffect::PlayerDeath => "audio/player_death.ogg",
        SoundEffect::LevelUp => "audio/levelup.ogg",
        SoundEffect::HealthPickup => "audio/health.ogg",
        SoundEffect::WeaponPickup => "audio/weapon.ogg",
        SoundEffect::BonusPickup => "audio/bonus.ogg",
        SoundEffect::MenuSelect => "audio/menu_select.ogg",
        SoundEffect::MenuBack => "audio/menu_back.ogg",
    };

    // Only play if file exists (gracefully handle missing audio files)
    let handle = asset_server.load(path);
    audio
        .play(handle)
        .with_volume(settings.effective_sfx_volume());
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
