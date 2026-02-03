//! Player-related resources

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Configuration for player behavior
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConfig {
    /// Base movement speed in pixels per second
    pub base_move_speed: f32,
    /// Base health
    pub base_health: f32,
    /// Invincibility duration after taking damage
    pub damage_invincibility_duration: f32,
    /// Invincibility duration after spawning
    pub spawn_invincibility_duration: f32,
    /// Base experience per kill multiplier
    pub exp_multiplier: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            base_move_speed: 200.0,
            base_health: 100.0,
            damage_invincibility_duration: 0.5,
            spawn_invincibility_duration: 2.0,
            exp_multiplier: 1.0,
        }
    }
}

/// Input mapping for player controls
#[derive(Resource, Debug, Clone)]
pub struct PlayerInputMapping {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub fire: MouseButton,
    pub reload: KeyCode,
    pub use_item: KeyCode,
}

impl Default for PlayerInputMapping {
    fn default() -> Self {
        Self {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            fire: MouseButton::Left,
            reload: KeyCode::KeyR,
            use_item: KeyCode::Space,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_config_has_reasonable_defaults() {
        let config = PlayerConfig::default();
        assert!(config.base_move_speed > 0.0);
        assert!(config.base_health > 0.0);
        assert!(config.damage_invincibility_duration >= 0.0);
    }

    #[test]
    fn player_input_mapping_has_wasd_defaults() {
        let mapping = PlayerInputMapping::default();
        assert_eq!(mapping.move_up, KeyCode::KeyW);
        assert_eq!(mapping.move_down, KeyCode::KeyS);
        assert_eq!(mapping.move_left, KeyCode::KeyA);
        assert_eq!(mapping.move_right, KeyCode::KeyD);
    }
}
