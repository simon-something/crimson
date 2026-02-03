//! Bonus components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Types of bonuses that can spawn
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BonusType {
    // Health
    SmallHealth,
    LargeHealth,
    FullHealth,

    // Experience
    SmallExp,
    LargeExp,

    // Weapons (random weapon pickup)
    WeaponPickup,

    // Temporary Effects
    SpeedBoost,
    FireRateBoost,
    DamageBoost,
    Invincibility,
    Shield,

    // Special
    Nuke,
    Freeze,
    SlowMotion,
}

impl BonusType {
    pub fn duration(&self) -> Option<f32> {
        match self {
            BonusType::SpeedBoost => Some(10.0),
            BonusType::FireRateBoost => Some(10.0),
            BonusType::DamageBoost => Some(10.0),
            BonusType::Invincibility => Some(5.0),
            BonusType::Shield => Some(15.0),
            BonusType::SlowMotion => Some(5.0),
            _ => None,
        }
    }

    pub fn spawn_weight(&self) -> u32 {
        match self {
            BonusType::SmallHealth => 20,
            BonusType::LargeHealth => 10,
            BonusType::FullHealth => 2,
            BonusType::SmallExp => 25,
            BonusType::LargeExp => 5,
            BonusType::WeaponPickup => 15,
            BonusType::SpeedBoost => 8,
            BonusType::FireRateBoost => 8,
            BonusType::DamageBoost => 8,
            BonusType::Invincibility => 3,
            BonusType::Shield => 5,
            BonusType::Nuke => 1,
            BonusType::Freeze => 4,
            BonusType::SlowMotion => 3,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            BonusType::SmallHealth | BonusType::LargeHealth | BonusType::FullHealth => {
                Color::srgb(1.0, 0.2, 0.2)
            }
            BonusType::SmallExp | BonusType::LargeExp => Color::srgb(1.0, 1.0, 0.2),
            BonusType::WeaponPickup => Color::srgb(0.8, 0.5, 0.2),
            BonusType::SpeedBoost => Color::srgb(0.2, 0.8, 1.0),
            BonusType::FireRateBoost => Color::srgb(1.0, 0.5, 0.0),
            BonusType::DamageBoost => Color::srgb(1.0, 0.0, 0.5),
            BonusType::Invincibility => Color::srgb(1.0, 1.0, 1.0),
            BonusType::Shield => Color::srgb(0.3, 0.3, 1.0),
            BonusType::Nuke => Color::srgb(1.0, 0.8, 0.0),
            BonusType::Freeze => Color::srgb(0.5, 0.8, 1.0),
            BonusType::SlowMotion => Color::srgb(0.6, 0.3, 0.8),
        }
    }
}

/// Marker component for bonus entities
#[derive(Component, Debug, Clone)]
pub struct Bonus {
    pub bonus_type: BonusType,
}

/// Lifetime for bonuses (they despawn after a while)
#[derive(Component, Debug, Clone)]
pub struct BonusLifetime {
    pub remaining: f32,
}

impl Default for BonusLifetime {
    fn default() -> Self {
        Self { remaining: 15.0 } // 15 seconds default
    }
}

/// Component for bonuses being attracted to the player
#[derive(Component, Debug, Clone)]
pub struct BonusAttraction {
    pub speed: f32,
    pub target: Option<Entity>,
}

impl Default for BonusAttraction {
    fn default() -> Self {
        Self {
            speed: 200.0,
            target: None,
        }
    }
}

/// Bundle for spawning bonuses
#[derive(Bundle)]
pub struct BonusBundle {
    pub bonus: Bonus,
    pub lifetime: BonusLifetime,
    pub attraction: BonusAttraction,
    pub sprite: SpriteBundle,
}

impl BonusBundle {
    pub fn new(bonus_type: BonusType, position: Vec3) -> Self {
        Self {
            bonus: Bonus { bonus_type },
            lifetime: BonusLifetime::default(),
            attraction: BonusAttraction::default(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: bonus_type.color(),
                    custom_size: Some(Vec2::splat(16.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
        }
    }
}

/// Component for active temporary bonus effects on a player
#[derive(Component, Debug, Clone, Default)]
pub struct ActiveBonusEffects {
    pub speed_boost_timer: f32,
    pub fire_rate_boost_timer: f32,
    pub damage_boost_timer: f32,
    pub invincibility_timer: f32,
    pub shield_timer: f32,
    pub slow_motion_timer: f32,
}

impl ActiveBonusEffects {
    pub fn tick(&mut self, delta: f32) {
        self.speed_boost_timer = (self.speed_boost_timer - delta).max(0.0);
        self.fire_rate_boost_timer = (self.fire_rate_boost_timer - delta).max(0.0);
        self.damage_boost_timer = (self.damage_boost_timer - delta).max(0.0);
        self.invincibility_timer = (self.invincibility_timer - delta).max(0.0);
        self.shield_timer = (self.shield_timer - delta).max(0.0);
        self.slow_motion_timer = (self.slow_motion_timer - delta).max(0.0);
    }

    pub fn has_speed_boost(&self) -> bool {
        self.speed_boost_timer > 0.0
    }

    pub fn has_fire_rate_boost(&self) -> bool {
        self.fire_rate_boost_timer > 0.0
    }

    pub fn has_damage_boost(&self) -> bool {
        self.damage_boost_timer > 0.0
    }

    pub fn has_invincibility(&self) -> bool {
        self.invincibility_timer > 0.0
    }

    pub fn has_shield(&self) -> bool {
        self.shield_timer > 0.0
    }

    pub fn has_slow_motion(&self) -> bool {
        self.slow_motion_timer > 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_type_duration_returns_some_for_timed() {
        assert!(BonusType::SpeedBoost.duration().is_some());
        assert!(BonusType::Invincibility.duration().is_some());
    }

    #[test]
    fn bonus_type_duration_returns_none_for_instant() {
        assert!(BonusType::SmallHealth.duration().is_none());
        assert!(BonusType::Nuke.duration().is_none());
    }

    #[test]
    fn bonus_type_all_have_spawn_weights() {
        let types = [
            BonusType::SmallHealth,
            BonusType::LargeHealth,
            BonusType::WeaponPickup,
            BonusType::Nuke,
        ];

        for bt in types {
            assert!(bt.spawn_weight() > 0);
        }
    }

    #[test]
    fn active_bonus_effects_tick_down() {
        let mut effects = ActiveBonusEffects {
            speed_boost_timer: 5.0,
            ..default()
        };

        assert!(effects.has_speed_boost());
        effects.tick(3.0);
        assert!(effects.has_speed_boost());
        effects.tick(3.0);
        assert!(!effects.has_speed_boost());
    }

    #[test]
    fn active_bonus_effects_clamp_to_zero() {
        let mut effects = ActiveBonusEffects {
            speed_boost_timer: 1.0,
            ..default()
        };

        effects.tick(10.0);
        assert_eq!(effects.speed_boost_timer, 0.0);
    }
}
