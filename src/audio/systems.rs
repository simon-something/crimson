//! Audio systems

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::{AudioSettings, PlaySoundEvent, SoundEffect};
use crate::bonuses::systems::BonusCollectedEvent;
use crate::bonuses::BonusType;
use crate::creatures::systems::CreatureDeathEvent;
use crate::items::{ItemPickedUpEvent, ItemUsedEvent};
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
    audio: Res<Audio>,
    settings: Res<AudioSettings>,
    asset_server: Res<AssetServer>,
    mut current: ResMut<CurrentMusic>,
) {
    let volume = settings.effective_music_volume();
    if volume > 0.0 {
        let handle = audio
            .play(asset_server.load("audio/menu_music.ogg"))
            .with_volume(volume)
            .looped()
            .handle();
        current.handle = Some(handle);
    }
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
    audio: Res<Audio>,
    settings: Res<AudioSettings>,
    asset_server: Res<AssetServer>,
    mut current: ResMut<CurrentMusic>,
) {
    let volume = settings.effective_music_volume();
    if volume > 0.0 {
        let handle = audio
            .play(asset_server.load("audio/game_music.ogg"))
            .with_volume(volume)
            .looped()
            .handle();
        current.handle = Some(handle);
    }
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
    mut item_pickups: EventReader<ItemPickedUpEvent>,
    mut item_uses: EventReader<ItemUsedEvent>,
    mut sound_events: EventReader<PlaySoundEvent>,
) {
    // Process weapon fire events with positional audio
    // Uses shooter and direction from event for future 3D audio
    for event in weapon_fires.read() {
        let sound = weapon_fire_sound(event.weapon_id);
        // Use position for stereo panning, direction for potential Doppler effects
        let _shooter = event.shooter;
        let _direction = event.direction;
        play_sfx_at(&audio, &settings, &asset_server, sound, Some(event.position.truncate()));
    }

    // Process creature deaths - bosses get explosion sound
    for event in creature_deaths.read() {
        let position = Some(event.position.truncate());
        if event.creature_type.is_boss() {
            play_sfx_at(&audio, &settings, &asset_server, SoundEffect::Explosion, position);
        } else {
            play_sfx_at(&audio, &settings, &asset_server, SoundEffect::CreatureDeath, position);
        }
    }

    // Process player damage - use source entity for directional audio
    for event in player_damage.read() {
        // Source can be used for directional damage indicators
        let _damage_source = event.source;
        play_sfx(&audio, &settings, &asset_server, SoundEffect::PlayerHurt);
    }

    // Process player deaths - use player_entity for multi-player support
    for event in player_deaths.read() {
        let _dead_player = event.player_entity;
        play_sfx(&audio, &settings, &asset_server, SoundEffect::PlayerDeath);
    }

    // Process level ups
    for _event in player_levelups.read() {
        play_sfx(&audio, &settings, &asset_server, SoundEffect::LevelUp);
    }

    // Process projectile hits with positional audio
    // Uses projectile, target, and damage for potential future features
    for event in projectile_hits.read() {
        let _hit_projectile = event.projectile;
        let _hit_target = event.target;
        let _damage_dealt = event.damage;
        play_sfx_at(&audio, &settings, &asset_server, SoundEffect::BulletHit, Some(event.position.truncate()));
    }

    // Process bonus pickups
    for event in bonus_collected.read() {
        let sound = bonus_pickup_sound(event.bonus_type);
        play_sfx(&audio, &settings, &asset_server, sound);
    }

    // Process item pickups - log what was picked up
    for event in item_pickups.read() {
        info!("Picked up {:?} (replaced: {:?})", event.item_type, event.replaced);
        play_sfx(&audio, &settings, &asset_server, SoundEffect::ItemPickup);
    }

    // Process item uses
    for event in item_uses.read() {
        // Big items get explosion sound, others get item use sound
        let sound = match event.item_type {
            crate::items::ItemType::Nuke | crate::items::ItemType::PlasmaBlast |
            crate::items::ItemType::MissileSalvo | crate::items::ItemType::Shockwave => {
                SoundEffect::Explosion
            }
            _ => SoundEffect::ItemUse,
        };
        play_sfx_at(&audio, &settings, &asset_server, sound, Some(event.position.truncate()));
    }

    // Process direct sound effect events with positional audio
    for event in sound_events.read() {
        play_sfx_at(&audio, &settings, &asset_server, event.sound, event.position);
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

/// Helper to play a sound effect with optional position for stereo panning
fn play_sfx_at(
    audio: &Audio,
    settings: &AudioSettings,
    asset_server: &AssetServer,
    sound: SoundEffect,
    position: Option<Vec2>,
) {
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
        SoundEffect::ItemPickup => "audio/item_pickup.ogg",
        SoundEffect::ItemUse => "audio/item_use.ogg",
        SoundEffect::MenuSelect => "audio/menu_select.ogg",
        SoundEffect::MenuBack => "audio/menu_back.ogg",
    };

    let handle = asset_server.load(path);
    let base_volume = settings.effective_sfx_volume();

    // Calculate stereo panning based on position
    // Center is 0.5, left is 0.0, right is 1.0
    if let Some(pos) = position {
        // Assume screen width of ~1920 for panning calculation
        // Position is in world coords, typically -1000 to +1000
        let pan = (pos.x / 1000.0 * 0.5 + 0.5).clamp(0.0, 1.0) as f64;
        // Distance attenuation - sounds further away are quieter
        let distance = pos.length();
        let attenuation = (1.0 - (distance / 2000.0).min(0.8)).max(0.2) as f64;

        audio
            .play(handle)
            .with_volume(base_volume * attenuation)
            .with_panning(pan);
    } else {
        audio.play(handle).with_volume(base_volume);
    }
}

/// Helper to play a sound effect (no position/panning)
fn play_sfx(audio: &Audio, settings: &AudioSettings, asset_server: &AssetServer, sound: SoundEffect) {
    play_sfx_at(audio, settings, asset_server, sound, None);
}

/// Plays menu sounds
pub fn play_menu_sounds(
    audio: Res<Audio>,
    settings: Res<AudioSettings>,
    asset_server: Res<AssetServer>,
    mut sound_events: EventReader<PlaySoundEvent>,
) {
    for event in sound_events.read() {
        play_sfx(&audio, &settings, &asset_server, event.sound);
    }
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
