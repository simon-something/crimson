//! Creature components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Types of creatures in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreatureType {
    // Basic enemies
    Zombie,
    Spider,
    Lizard,
    Beetle,

    // Medium enemies
    AlienSpider,
    Giant,
    Necromancer,
    GiantSpider,

    // Fast enemies
    Dog,
    Runner,

    // Ranged enemies
    AlienShooter,
    Turret,

    // Special enemies
    Ghost,
    Exploder,
    Splitter,

    // Bosses
    BossSpider,
    BossAlien,
    BossNest,
}

impl CreatureType {
    pub fn base_health(&self) -> f32 {
        match self {
            CreatureType::Zombie => 30.0,
            CreatureType::Spider => 15.0,
            CreatureType::Lizard => 25.0,
            CreatureType::Beetle => 20.0,
            CreatureType::AlienSpider => 40.0,
            CreatureType::Giant => 100.0,
            CreatureType::Necromancer => 80.0,
            CreatureType::GiantSpider => 120.0,
            CreatureType::Dog => 20.0,
            CreatureType::Runner => 25.0,
            CreatureType::AlienShooter => 35.0,
            CreatureType::Turret => 60.0,
            CreatureType::Ghost => 50.0,
            CreatureType::Exploder => 15.0,
            CreatureType::Splitter => 40.0,
            CreatureType::BossSpider => 500.0,
            CreatureType::BossAlien => 800.0,
            CreatureType::BossNest => 1000.0,
        }
    }

    pub fn base_speed(&self) -> f32 {
        match self {
            CreatureType::Zombie => 40.0,
            CreatureType::Spider => 80.0,
            CreatureType::Lizard => 60.0,
            CreatureType::Beetle => 50.0,
            CreatureType::AlienSpider => 90.0,
            CreatureType::Giant => 30.0,
            CreatureType::Necromancer => 35.0,
            CreatureType::GiantSpider => 45.0,
            CreatureType::Dog => 120.0,
            CreatureType::Runner => 150.0,
            CreatureType::AlienShooter => 50.0,
            CreatureType::Turret => 0.0,
            CreatureType::Ghost => 70.0,
            CreatureType::Exploder => 100.0,
            CreatureType::Splitter => 60.0,
            CreatureType::BossSpider => 40.0,
            CreatureType::BossAlien => 50.0,
            CreatureType::BossNest => 0.0,
        }
    }

    pub fn base_damage(&self) -> f32 {
        match self {
            CreatureType::Zombie => 10.0,
            CreatureType::Spider => 8.0,
            CreatureType::Lizard => 12.0,
            CreatureType::Beetle => 8.0,
            CreatureType::AlienSpider => 15.0,
            CreatureType::Giant => 25.0,
            CreatureType::Necromancer => 20.0,
            CreatureType::GiantSpider => 30.0,
            CreatureType::Dog => 12.0,
            CreatureType::Runner => 10.0,
            CreatureType::AlienShooter => 15.0,
            CreatureType::Turret => 20.0,
            CreatureType::Ghost => 15.0,
            CreatureType::Exploder => 50.0,
            CreatureType::Splitter => 15.0,
            CreatureType::BossSpider => 40.0,
            CreatureType::BossAlien => 50.0,
            CreatureType::BossNest => 0.0,
        }
    }

    pub fn experience_value(&self) -> u32 {
        match self {
            CreatureType::Zombie => 10,
            CreatureType::Spider => 8,
            CreatureType::Lizard => 12,
            CreatureType::Beetle => 8,
            CreatureType::AlienSpider => 20,
            CreatureType::Giant => 50,
            CreatureType::Necromancer => 40,
            CreatureType::GiantSpider => 60,
            CreatureType::Dog => 15,
            CreatureType::Runner => 15,
            CreatureType::AlienShooter => 25,
            CreatureType::Turret => 30,
            CreatureType::Ghost => 35,
            CreatureType::Exploder => 20,
            CreatureType::Splitter => 25,
            CreatureType::BossSpider => 500,
            CreatureType::BossAlien => 800,
            CreatureType::BossNest => 1000,
        }
    }

    pub fn is_boss(&self) -> bool {
        matches!(
            self,
            CreatureType::BossSpider | CreatureType::BossAlien | CreatureType::BossNest
        )
    }
}

/// AI behavior modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AIMode {
    /// Chase the nearest player
    #[default]
    Chase,
    /// Wander randomly
    Wander,
    /// Flee from the player
    Flee,
    /// Circle around the player (ranged enemies)
    Circle,
    /// Stay stationary (turrets)
    Stationary,
    /// Dead, waiting for cleanup
    Dead,
}

/// Marker component for creature entities
#[derive(Component, Debug, Clone)]
pub struct Creature {
    pub creature_type: CreatureType,
}

/// AI state component
#[derive(Component, Debug, Clone, Default)]
pub struct AIState {
    pub mode: AIMode,
    pub target: Option<Entity>,
    pub wander_direction: Vec2,
    pub wander_timer: f32,
    /// Time since last attack
    pub attack_cooldown: f32,
}

/// Creature health (separate from player health for potential different behavior)
#[derive(Component, Debug, Clone)]
pub struct CreatureHealth {
    pub current: f32,
    pub max: f32,
}

impl CreatureHealth {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn percentage(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
}

/// Movement speed for creatures
#[derive(Component, Debug, Clone)]
pub struct CreatureSpeed(pub f32);

/// Damage dealt on contact
#[derive(Component, Debug, Clone)]
pub struct ContactDamage(pub f32);

/// Experience granted when killed
#[derive(Component, Debug, Clone)]
pub struct ExperienceValue(pub u32);

/// Marker for creatures that should be despawned
#[derive(Component)]
pub struct MarkedForDespawn;

/// Bundle for spawning creatures
#[derive(Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub health: CreatureHealth,
    pub ai_state: AIState,
    pub speed: CreatureSpeed,
    pub contact_damage: ContactDamage,
    pub experience_value: ExperienceValue,
    pub sprite: SpriteBundle,
}

impl CreatureBundle {
    pub fn new(creature_type: CreatureType, position: Vec3) -> Self {
        let color = match creature_type {
            CreatureType::Zombie => Color::srgb(0.3, 0.5, 0.3),
            CreatureType::Spider => Color::srgb(0.2, 0.2, 0.2),
            CreatureType::Dog | CreatureType::Runner => Color::srgb(0.6, 0.3, 0.1),
            CreatureType::Ghost => Color::srgba(0.8, 0.8, 1.0, 0.5),
            CreatureType::Exploder => Color::srgb(1.0, 0.3, 0.1),
            _ if creature_type.is_boss() => Color::srgb(0.8, 0.1, 0.1),
            _ => Color::srgb(0.5, 0.3, 0.3),
        };

        let size = if creature_type.is_boss() {
            64.0
        } else {
            match creature_type {
                CreatureType::Giant | CreatureType::GiantSpider => 48.0,
                CreatureType::Spider | CreatureType::Beetle => 20.0,
                _ => 28.0,
            }
        };

        Self {
            creature: Creature { creature_type },
            health: CreatureHealth::new(creature_type.base_health()),
            ai_state: AIState::default(),
            speed: CreatureSpeed(creature_type.base_speed()),
            contact_damage: ContactDamage(creature_type.base_damage()),
            experience_value: ExperienceValue(creature_type.experience_value()),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(size)),
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
    fn creature_type_base_stats_are_positive() {
        let types = [
            CreatureType::Zombie,
            CreatureType::Spider,
            CreatureType::Giant,
            CreatureType::BossSpider,
        ];

        for ct in types {
            assert!(ct.base_health() > 0.0);
            assert!(ct.base_damage() >= 0.0);
            assert!(ct.base_speed() >= 0.0);
            assert!(ct.experience_value() > 0);
        }
    }

    #[test]
    fn bosses_are_identified() {
        assert!(CreatureType::BossSpider.is_boss());
        assert!(CreatureType::BossAlien.is_boss());
        assert!(CreatureType::BossNest.is_boss());
        assert!(!CreatureType::Zombie.is_boss());
        assert!(!CreatureType::Spider.is_boss());
    }

    #[test]
    fn creature_health_damage_works() {
        let mut health = CreatureHealth::new(100.0);
        health.damage(30.0);
        assert_eq!(health.current, 70.0);
        assert!(!health.is_dead());
    }

    #[test]
    fn creature_health_clamps_to_zero() {
        let mut health = CreatureHealth::new(50.0);
        health.damage(100.0);
        assert_eq!(health.current, 0.0);
        assert!(health.is_dead());
    }

    #[test]
    fn ai_mode_default_is_chase() {
        assert_eq!(AIMode::default(), AIMode::Chase);
    }
}
