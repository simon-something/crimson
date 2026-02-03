//! Effect systems

use bevy::prelude::*;
use rand::Rng;

use super::components::{
    CameraBasePosition, Effect, EffectType, Particle, ParticleBundle, ScreenShake,
};
use crate::audio::{PlaySoundEvent, SoundEffect};
use crate::bonuses::systems::BonusCollectedEvent;
use crate::creatures::systems::CreatureDeathEvent;
use crate::player::components::Player;
use crate::player::systems::PlayerLevelUpEvent;
use crate::weapons::components::Explosive;
use crate::weapons::systems::{FireWeaponEvent, ProjectileHitEvent};

/// Event to spawn an effect
#[derive(Event)]
pub struct SpawnEffectEvent {
    pub effect_type: EffectType,
    pub position: Vec3,
    pub count: u32,
}

/// Handles effect spawn events
pub fn handle_effect_spawns(mut commands: Commands, mut events: EventReader<SpawnEffectEvent>) {
    let mut rng = rand::thread_rng();

    for event in events.read() {
        match event.effect_type {
            EffectType::BloodSplatter => {
                for _ in 0..event.count {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let speed = rng.gen_range(50.0..150.0);
                    let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                    commands.spawn(ParticleBundle::blood(event.position, velocity));
                }
            }
            EffectType::Explosion => {
                for _ in 0..event.count {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let speed = rng.gen_range(100.0..300.0);
                    let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                    commands.spawn(ParticleBundle::explosion(event.position, velocity));
                }
            }
            EffectType::MuzzleFlash => {
                commands.spawn(ParticleBundle::muzzle_flash(event.position));
            }
            EffectType::BulletImpact => {
                for _ in 0..event.count.min(5) {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let speed = rng.gen_range(30.0..80.0);
                    let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);

                    commands.spawn((
                        Effect {
                            effect_type: EffectType::BulletImpact,
                        },
                        Particle::new(velocity, 0.2),
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(1.0, 0.8, 0.3),
                                custom_size: Some(Vec2::splat(3.0)),
                                ..default()
                            },
                            transform: Transform::from_translation(event.position),
                            ..default()
                        },
                    ));
                }
            }
            EffectType::PickupCollect => {
                for i in 0..8 {
                    let angle = (i as f32 / 8.0) * std::f32::consts::TAU;
                    let velocity = Vec2::new(angle.cos() * 100.0, angle.sin() * 100.0);

                    commands.spawn((
                        Effect {
                            effect_type: EffectType::PickupCollect,
                        },
                        Particle::new(velocity, 0.3).with_fade(true),
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(1.0, 1.0, 0.5),
                                custom_size: Some(Vec2::splat(4.0)),
                                ..default()
                            },
                            transform: Transform::from_translation(event.position),
                            ..default()
                        },
                    ));
                }
            }
            EffectType::LevelUp => {
                for i in 0..16 {
                    let angle = (i as f32 / 16.0) * std::f32::consts::TAU;
                    let velocity = Vec2::new(angle.cos() * 150.0, angle.sin() * 150.0);

                    commands.spawn((
                        Effect {
                            effect_type: EffectType::LevelUp,
                        },
                        Particle::new(velocity, 0.5)
                            .with_fade(true)
                            .with_scale_change(1.5),
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.3, 0.8, 1.0),
                                custom_size: Some(Vec2::splat(6.0)),
                                ..default()
                            },
                            transform: Transform::from_translation(event.position),
                            ..default()
                        },
                    ));
                }
            }
            EffectType::Death => {
                // Combination of blood and explosion
                for _ in 0..15 {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let speed = rng.gen_range(80.0..200.0);
                    let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
                    commands.spawn(ParticleBundle::blood(event.position, velocity));
                }
            }
        }
    }
}

/// Updates particle positions and lifetimes
pub fn update_particles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Particle, &mut Sprite, &Effect)>,
) {
    for (mut transform, mut particle, mut sprite, effect) in query.iter_mut() {
        // Apply velocity
        transform.translation.x += particle.velocity.x * time.delta_seconds();
        transform.translation.y += particle.velocity.y * time.delta_seconds();

        // Apply gravity
        particle.velocity.y -= particle.gravity * time.delta_seconds();

        // Update lifetime
        particle.lifetime -= time.delta_seconds();

        // Apply type-specific color changes based on effect_type
        let progress = particle.progress();
        match effect.effect_type {
            EffectType::Explosion => {
                // Explosions shift from yellow to red as they fade
                let r = 1.0;
                let g = 0.6 * (1.0 - progress);
                let b = 0.1 * (1.0 - progress);
                sprite.color = Color::srgba(r, g, b, 1.0 - progress);
            }
            EffectType::BloodSplatter | EffectType::Death => {
                // Blood darkens as it ages
                let darkness = 0.6 - 0.3 * progress;
                if particle.fade_out {
                    sprite.color = Color::srgba(darkness, 0.0, 0.0, 1.0 - progress);
                } else {
                    sprite.color = Color::srgb(darkness, 0.0, 0.0);
                }
            }
            _ => {
                // Apply fade out using progress() for smoother interpolation
                if particle.fade_out {
                    let alpha = 1.0 - progress;
                    sprite.color = sprite.color.with_alpha(alpha.max(0.0));
                }
            }
        }

        // Apply scale change
        if let Some(scale_delta) = particle.scale_over_time {
            let scale_change = 1.0 + scale_delta * time.delta_seconds();
            transform.scale *= scale_change;
        }
    }
}

/// Updates camera to follow the player
pub fn update_camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut base_pos: ResMut<CameraBasePosition>,
) {
    // Camera follows the average player position
    let mut total_pos = Vec2::ZERO;
    let mut count = 0;

    for transform in player_query.iter() {
        total_pos += transform.translation.truncate();
        count += 1;
    }

    if count > 0 {
        base_pos.position = total_pos / count as f32;
    }
}

/// Updates screen shake effect and applies to camera
pub fn update_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    base_pos: Res<CameraBasePosition>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    shake.update(time.delta_seconds());

    let offset = shake.get_offset();

    for mut transform in camera_query.iter_mut() {
        // Apply base position plus shake offset
        transform.translation.x = base_pos.position.x + offset.x;
        transform.translation.y = base_pos.position.y + offset.y;
    }
}

/// Triggers screen shake when projectiles hit creatures
pub fn trigger_screen_shake_on_hit(
    mut hit_events: EventReader<ProjectileHitEvent>,
    mut shake: ResMut<ScreenShake>,
) {
    for _event in hit_events.read() {
        // Small shake on each hit
        shake.add(1.5, 0.1);
    }
}

/// Removes expired particle effects
pub fn cleanup_expired_effects(mut commands: Commands, query: Query<(Entity, &Particle)>) {
    for (entity, particle) in query.iter() {
        if particle.is_expired() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Cleans up all effects when leaving Playing state
pub fn cleanup_all_effects(mut commands: Commands, query: Query<Entity, With<Effect>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Spawns blood effects when creatures die
pub fn spawn_blood_on_death(
    mut death_events: EventReader<CreatureDeathEvent>,
    mut effect_events: EventWriter<SpawnEffectEvent>,
    mut shake: ResMut<ScreenShake>,
) {
    for event in death_events.read() {
        // Use entity for logging/debugging purposes
        let _dead_entity = event.entity;

        // Bosses get bigger effects
        let is_boss = event.creature_type.is_boss();
        let blood_count = if is_boss { 20 } else { 8 };

        // Spawn blood splatter
        effect_events.send(SpawnEffectEvent {
            effect_type: EffectType::BloodSplatter,
            position: event.position,
            count: blood_count,
        });

        // Also spawn death effect for larger impact
        effect_events.send(SpawnEffectEvent {
            effect_type: EffectType::Death,
            position: event.position,
            count: 1,
        });

        // Bosses cause screen shake on death
        if is_boss {
            shake.add(8.0, 0.5);
            // Spawn explosion effect for boss deaths
            effect_events.send(SpawnEffectEvent {
                effect_type: EffectType::Explosion,
                position: event.position,
                count: 30,
            });
        }
    }
}

/// Spawns level up effect at player position
/// Uses new_level to scale effect intensity at milestone levels
pub fn spawn_levelup_effect(
    mut levelup_events: EventReader<PlayerLevelUpEvent>,
    mut effect_events: EventWriter<SpawnEffectEvent>,
    mut shake: ResMut<ScreenShake>,
    player_query: Query<&Transform, With<Player>>,
) {
    for event in levelup_events.read() {
        if let Ok(transform) = player_query.get(event.player_entity) {
            // Bigger effect at milestone levels (5, 10, 15, etc.)
            let is_milestone = event.new_level % 5 == 0;
            let particle_count = if is_milestone { 3 } else { 1 };

            for _ in 0..particle_count {
                effect_events.send(SpawnEffectEvent {
                    effect_type: EffectType::LevelUp,
                    position: transform.translation,
                    count: 1,
                });
            }

            // Screen shake at milestone levels
            if is_milestone {
                shake.add(3.0, 0.3);
            }
        }
    }
}

/// Spawns pickup effect when bonuses are collected
pub fn spawn_pickup_effect(
    mut bonus_events: EventReader<BonusCollectedEvent>,
    mut effect_events: EventWriter<SpawnEffectEvent>,
    player_query: Query<&Transform, With<Player>>,
) {
    for event in bonus_events.read() {
        if let Ok(transform) = player_query.get(event.player_entity) {
            effect_events.send(SpawnEffectEvent {
                effect_type: EffectType::PickupCollect,
                position: transform.translation,
                count: 1,
            });
        }
    }
}

/// Spawns muzzle flash when weapons fire
pub fn spawn_muzzle_flash(
    mut fire_events: EventReader<FireWeaponEvent>,
    mut effect_events: EventWriter<SpawnEffectEvent>,
) {
    for event in fire_events.read() {
        effect_events.send(SpawnEffectEvent {
            effect_type: EffectType::MuzzleFlash,
            position: event.position,
            count: 1,
        });
    }
}

/// Spawns bullet impact effect when projectiles hit
pub fn spawn_hit_effect(
    mut hit_events: EventReader<ProjectileHitEvent>,
    mut effect_events: EventWriter<SpawnEffectEvent>,
) {
    for event in hit_events.read() {
        effect_events.send(SpawnEffectEvent {
            effect_type: EffectType::BulletImpact,
            position: event.position,
            count: 3,
        });
    }
}

/// Plays explosion sound and effects for explosive projectiles
pub fn spawn_explosion_effects(
    query: Query<(&Transform, &Explosive)>,
    despawned: Query<Entity, Added<crate::weapons::components::ProjectileDespawn>>,
    explosive_query: Query<(&Transform, &Explosive)>,
    mut effect_events: EventWriter<SpawnEffectEvent>,
    mut sound_events: EventWriter<PlaySoundEvent>,
) {
    // When an explosive projectile is despawned, spawn explosion effects
    for entity in despawned.iter() {
        if let Ok((transform, _explosive)) = explosive_query.get(entity) {
            effect_events.send(SpawnEffectEvent {
                effect_type: EffectType::Explosion,
                position: transform.translation,
                count: 20,
            });
            sound_events.send(PlaySoundEvent {
                sound: SoundEffect::Explosion,
                position: Some(transform.translation.truncate()),
            });
        }
    }

    // Suppress unused warning for query
    let _ = query;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_effect_event_can_be_created() {
        let event = SpawnEffectEvent {
            effect_type: EffectType::BloodSplatter,
            position: Vec3::new(100.0, 200.0, 0.0),
            count: 10,
        };
        assert_eq!(event.count, 10);
    }
}
