//! Effect components

use bevy::prelude::*;

/// Types of visual effects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    /// Blood splatter when creatures die
    BloodSplatter,
    /// Explosion effect
    Explosion,
    /// Muzzle flash from weapon
    MuzzleFlash,
    /// Bullet impact on creature
    BulletImpact,
    /// Pickup collected
    PickupCollect,
    /// Level up effect
    LevelUp,
    /// Death effect
    Death,
}

/// Marker component for effect entities
#[derive(Component, Debug)]
pub struct Effect {
    pub effect_type: EffectType,
}

/// Component for particle effects
#[derive(Component, Debug)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub gravity: f32,
    pub fade_out: bool,
    pub scale_over_time: Option<f32>,
}

impl Particle {
    pub fn new(velocity: Vec2, lifetime: f32) -> Self {
        Self {
            velocity,
            lifetime,
            max_lifetime: lifetime,
            gravity: 0.0,
            fade_out: true,
            scale_over_time: None,
        }
    }

    pub fn with_gravity(mut self, gravity: f32) -> Self {
        self.gravity = gravity;
        self
    }

    pub fn with_fade(mut self, fade: bool) -> Self {
        self.fade_out = fade;
        self
    }

    pub fn with_scale_change(mut self, scale_delta: f32) -> Self {
        self.scale_over_time = Some(scale_delta);
        self
    }

    pub fn progress(&self) -> f32 {
        if self.max_lifetime > 0.0 {
            1.0 - (self.lifetime / self.max_lifetime)
        } else {
            1.0
        }
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime <= 0.0
    }
}

/// Component for screen shake effects
#[derive(Resource, Debug, Default)]
pub struct ScreenShake {
    pub intensity: f32,
    pub duration: f32,
    pub decay: f32,
}

/// Tracks the base camera position before shake offset is applied
#[derive(Resource, Debug, Default)]
pub struct CameraBasePosition {
    pub position: Vec2,
}

impl ScreenShake {
    pub fn add(&mut self, intensity: f32, duration: f32) {
        // Stack shakes but cap intensity
        self.intensity = (self.intensity + intensity).min(20.0);
        self.duration = self.duration.max(duration);
        self.decay = intensity / duration;
    }

    pub fn update(&mut self, delta: f32) {
        if self.duration > 0.0 {
            self.duration -= delta;
            self.intensity = (self.intensity - self.decay * delta).max(0.0);
        } else {
            self.intensity = 0.0;
        }
    }

    pub fn get_offset(&self) -> Vec2 {
        if self.intensity <= 0.0 {
            return Vec2::ZERO;
        }

        let angle = rand::random::<f32>() * std::f32::consts::TAU;
        Vec2::new(angle.cos(), angle.sin()) * self.intensity * rand::random::<f32>()
    }
}

/// Bundle for spawning particle effects
#[derive(Bundle)]
pub struct ParticleBundle {
    pub effect: Effect,
    pub particle: Particle,
    pub sprite: SpriteBundle,
}

impl ParticleBundle {
    pub fn blood(position: Vec3, velocity: Vec2) -> Self {
        Self {
            effect: Effect {
                effect_type: EffectType::BloodSplatter,
            },
            particle: Particle::new(velocity, 0.5).with_gravity(200.0),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.6, 0.0, 0.0),
                    custom_size: Some(Vec2::splat(4.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
        }
    }

    pub fn explosion(position: Vec3, velocity: Vec2) -> Self {
        Self {
            effect: Effect {
                effect_type: EffectType::Explosion,
            },
            particle: Particle::new(velocity, 0.3)
                .with_fade(true)
                .with_scale_change(2.0),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.6, 0.1),
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
        }
    }

    pub fn muzzle_flash(position: Vec3) -> Self {
        Self {
            effect: Effect {
                effect_type: EffectType::MuzzleFlash,
            },
            particle: Particle::new(Vec2::ZERO, 0.05).with_fade(true),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.9, 0.5),
                    custom_size: Some(Vec2::new(16.0, 8.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn particle_progress_starts_at_zero() {
        let particle = Particle::new(Vec2::ZERO, 1.0);
        assert!((particle.progress() - 0.0).abs() < 0.001);
    }

    #[test]
    fn particle_expires_when_lifetime_zero() {
        let mut particle = Particle::new(Vec2::ZERO, 1.0);
        assert!(!particle.is_expired());
        particle.lifetime = 0.0;
        assert!(particle.is_expired());
    }

    #[test]
    fn screen_shake_adds_and_caps() {
        let mut shake = ScreenShake::default();
        shake.add(10.0, 0.5);
        assert!(shake.intensity > 0.0);

        shake.add(100.0, 0.5);
        assert!(shake.intensity <= 20.0); // Capped
    }

    #[test]
    fn screen_shake_decays() {
        let mut shake = ScreenShake::default();
        shake.add(10.0, 1.0);

        let initial = shake.intensity;
        shake.update(0.5);
        assert!(shake.intensity < initial);
    }

    #[test]
    fn screen_shake_offset_zero_when_no_shake() {
        let shake = ScreenShake::default();
        assert_eq!(shake.get_offset(), Vec2::ZERO);
    }
}
