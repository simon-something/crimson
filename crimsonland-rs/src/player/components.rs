//! Player components

use bevy::prelude::*;

/// Marker component for player entities
#[derive(Component, Debug, Clone)]
pub struct Player {
    /// Player index (0-3 for multiplayer support)
    pub index: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self { index: 0 }
    }
}

/// Health component for entities that can take damage
#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
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

impl Default for Health {
    fn default() -> Self {
        Self::new(100.0)
    }
}

/// Experience and level tracking
#[derive(Component, Debug, Clone)]
pub struct Experience {
    pub current: u32,
    pub level: u32,
    pub to_next_level: u32,
}

impl Experience {
    pub fn new() -> Self {
        Self {
            current: 0,
            level: 1,
            to_next_level: 100,
        }
    }

    /// Add experience and return true if leveled up
    pub fn add(&mut self, amount: u32) -> bool {
        self.current += amount;
        if self.current >= self.to_next_level {
            self.level_up();
            true
        } else {
            false
        }
    }

    fn level_up(&mut self) {
        self.current -= self.to_next_level;
        self.level += 1;
        // Experience curve: each level requires 20% more XP
        self.to_next_level = (self.to_next_level as f32 * 1.2) as u32;
    }

    pub fn progress(&self) -> f32 {
        if self.to_next_level > 0 {
            self.current as f32 / self.to_next_level as f32
        } else {
            0.0
        }
    }
}

impl Default for Experience {
    fn default() -> Self {
        Self::new()
    }
}

/// Movement speed component
#[derive(Component, Debug, Clone)]
pub struct MoveSpeed(pub f32);

impl Default for MoveSpeed {
    fn default() -> Self {
        Self(200.0) // pixels per second
    }
}

/// Component for aiming direction
#[derive(Component, Debug, Clone, Default)]
pub struct AimDirection {
    /// Normalized direction vector
    pub direction: Vec2,
    /// Angle in radians
    pub angle: f32,
}

impl AimDirection {
    pub fn from_direction(direction: Vec2) -> Self {
        let normalized = direction.normalize_or_zero();
        Self {
            direction: normalized,
            angle: normalized.y.atan2(normalized.x),
        }
    }

    pub fn from_angle(angle: f32) -> Self {
        Self {
            direction: Vec2::new(angle.cos(), angle.sin()),
            angle,
        }
    }
}

/// Component for entities currently firing a weapon
#[derive(Component, Debug, Clone)]
pub struct Firing {
    pub is_firing: bool,
    pub cooldown_timer: f32,
}

impl Default for Firing {
    fn default() -> Self {
        Self {
            is_firing: false,
            cooldown_timer: 0.0,
        }
    }
}

/// Component for temporary invincibility
#[derive(Component, Debug, Clone)]
pub struct Invincibility {
    pub timer: f32,
}

impl Invincibility {
    pub fn new(duration: f32) -> Self {
        Self { timer: duration }
    }

    pub fn is_active(&self) -> bool {
        self.timer > 0.0
    }

    pub fn tick(&mut self, delta: f32) {
        self.timer = (self.timer - delta).max(0.0);
    }
}

/// Bundle for spawning a complete player entity
#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub experience: Experience,
    pub move_speed: MoveSpeed,
    pub aim_direction: AimDirection,
    pub firing: Firing,
    pub sprite: SpriteBundle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_new_creates_full_health() {
        let health = Health::new(100.0);
        assert_eq!(health.current, 100.0);
        assert_eq!(health.max, 100.0);
    }

    #[test]
    fn health_damage_reduces_current() {
        let mut health = Health::new(100.0);
        health.damage(30.0);
        assert_eq!(health.current, 70.0);
    }

    #[test]
    fn health_damage_clamps_to_zero() {
        let mut health = Health::new(100.0);
        health.damage(150.0);
        assert_eq!(health.current, 0.0);
    }

    #[test]
    fn health_heal_increases_current() {
        let mut health = Health::new(100.0);
        health.current = 50.0;
        health.heal(30.0);
        assert_eq!(health.current, 80.0);
    }

    #[test]
    fn health_heal_clamps_to_max() {
        let mut health = Health::new(100.0);
        health.current = 90.0;
        health.heal(50.0);
        assert_eq!(health.current, 100.0);
    }

    #[test]
    fn health_is_dead_when_zero() {
        let mut health = Health::new(100.0);
        assert!(!health.is_dead());
        health.damage(100.0);
        assert!(health.is_dead());
    }

    #[test]
    fn health_percentage_correct() {
        let mut health = Health::new(100.0);
        assert_eq!(health.percentage(), 1.0);
        health.damage(25.0);
        assert_eq!(health.percentage(), 0.75);
    }

    #[test]
    fn experience_add_accumulates() {
        let mut exp = Experience::new();
        exp.add(50);
        assert_eq!(exp.current, 50);
        assert_eq!(exp.level, 1);
    }

    #[test]
    fn experience_level_up_on_threshold() {
        let mut exp = Experience::new();
        let leveled_up = exp.add(100);
        assert!(leveled_up);
        assert_eq!(exp.level, 2);
    }

    #[test]
    fn experience_carries_over_excess() {
        let mut exp = Experience::new();
        exp.add(120); // 100 needed, 20 carries over
        assert_eq!(exp.level, 2);
        assert_eq!(exp.current, 20);
    }

    #[test]
    fn aim_direction_from_angle() {
        let aim = AimDirection::from_angle(0.0);
        assert!((aim.direction.x - 1.0).abs() < 0.001);
        assert!(aim.direction.y.abs() < 0.001);
    }

    #[test]
    fn invincibility_ticks_down() {
        let mut inv = Invincibility::new(1.0);
        assert!(inv.is_active());
        inv.tick(0.5);
        assert!(inv.is_active());
        inv.tick(0.6);
        assert!(!inv.is_active());
    }
}
