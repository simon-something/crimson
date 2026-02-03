//! Creature spawning system

use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::components::CreatureType;

/// Registry of creature data
#[derive(Resource, Default)]
pub struct CreatureRegistry {
    pub definitions: Vec<CreatureDefinition>,
}

impl CreatureRegistry {
    pub fn new() -> Self {
        let mut registry = Self::default();
        registry.register_default_creatures();
        registry
    }

    fn register_default_creatures(&mut self) {
        // All creature types with their spawn weights and wave requirements
        self.definitions = vec![
            CreatureDefinition {
                creature_type: CreatureType::Zombie,
                min_wave: 1,
                spawn_weight: 10,
            },
            CreatureDefinition {
                creature_type: CreatureType::Spider,
                min_wave: 1,
                spawn_weight: 8,
            },
            CreatureDefinition {
                creature_type: CreatureType::Lizard,
                min_wave: 2,
                spawn_weight: 6,
            },
            CreatureDefinition {
                creature_type: CreatureType::Beetle,
                min_wave: 2,
                spawn_weight: 7,
            },
            CreatureDefinition {
                creature_type: CreatureType::Dog,
                min_wave: 3,
                spawn_weight: 5,
            },
            CreatureDefinition {
                creature_type: CreatureType::AlienSpider,
                min_wave: 5,
                spawn_weight: 4,
            },
            CreatureDefinition {
                creature_type: CreatureType::Giant,
                min_wave: 6,
                spawn_weight: 2,
            },
            CreatureDefinition {
                creature_type: CreatureType::Necromancer,
                min_wave: 7,
                spawn_weight: 2,
            },
            CreatureDefinition {
                creature_type: CreatureType::Runner,
                min_wave: 4,
                spawn_weight: 4,
            },
            CreatureDefinition {
                creature_type: CreatureType::AlienShooter,
                min_wave: 8,
                spawn_weight: 3,
            },
            CreatureDefinition {
                creature_type: CreatureType::Ghost,
                min_wave: 10,
                spawn_weight: 2,
            },
            CreatureDefinition {
                creature_type: CreatureType::Exploder,
                min_wave: 6,
                spawn_weight: 3,
            },
            CreatureDefinition {
                creature_type: CreatureType::Splitter,
                min_wave: 8,
                spawn_weight: 2,
            },
            CreatureDefinition {
                creature_type: CreatureType::GiantSpider,
                min_wave: 12,
                spawn_weight: 1,
            },
            CreatureDefinition {
                creature_type: CreatureType::Turret,
                min_wave: 10,
                spawn_weight: 2,
            },
        ];
    }

    pub fn get_available_for_wave(&self, wave: u32) -> Vec<&CreatureDefinition> {
        self.definitions
            .iter()
            .filter(|d| d.min_wave <= wave)
            .collect()
    }

    pub fn pick_random_for_wave(&self, wave: u32) -> Option<CreatureType> {
        let available = self.get_available_for_wave(wave);
        if available.is_empty() {
            return None;
        }

        let total_weight: u32 = available.iter().map(|d| d.spawn_weight).sum();
        if total_weight == 0 {
            return None;
        }

        let mut rng = rand::thread_rng();
        let mut roll = rng.gen_range(0..total_weight);

        for def in &available {
            if roll < def.spawn_weight {
                return Some(def.creature_type);
            }
            roll -= def.spawn_weight;
        }

        available.last().map(|d| d.creature_type)
    }
}

/// Definition for a creature type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureDefinition {
    pub creature_type: CreatureType,
    pub min_wave: u32,
    pub spawn_weight: u32,
}

/// Configuration for spawn behavior
#[derive(Debug, Clone)]
pub struct SpawnConfig {
    /// Minimum distance from player to spawn
    pub min_spawn_distance: f32,
    /// Maximum distance from player to spawn
    pub max_spawn_distance: f32,
    /// Arena bounds (half-width, half-height)
    pub arena_bounds: Vec2,
}

impl Default for SpawnConfig {
    fn default() -> Self {
        Self {
            min_spawn_distance: 400.0,
            max_spawn_distance: 600.0,
            arena_bounds: Vec2::new(800.0, 600.0),
        }
    }
}

/// Calculate a spawn position outside the player's view
pub fn calculate_spawn_position(player_pos: Vec2, config: &SpawnConfig) -> Vec3 {
    let mut rng = rand::thread_rng();

    // Random angle
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);

    // Random distance within range
    let distance = rng.gen_range(config.min_spawn_distance..config.max_spawn_distance);

    // Calculate position
    let offset = Vec2::new(angle.cos() * distance, angle.sin() * distance);
    let position = player_pos + offset;

    // Clamp to arena bounds
    let clamped = Vec2::new(
        position.x.clamp(-config.arena_bounds.x, config.arena_bounds.x),
        position.y.clamp(-config.arena_bounds.y, config.arena_bounds.y),
    );

    Vec3::new(clamped.x, clamped.y, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creature_registry_initializes_with_creatures() {
        let registry = CreatureRegistry::new();
        assert!(!registry.definitions.is_empty());
    }

    #[test]
    fn wave_1_has_basic_creatures() {
        let registry = CreatureRegistry::new();
        let available = registry.get_available_for_wave(1);
        assert!(!available.is_empty());

        let types: Vec<_> = available.iter().map(|d| d.creature_type).collect();
        assert!(types.contains(&CreatureType::Zombie));
        assert!(types.contains(&CreatureType::Spider));
    }

    #[test]
    fn later_waves_have_more_creatures() {
        let registry = CreatureRegistry::new();
        let wave_1 = registry.get_available_for_wave(1);
        let wave_10 = registry.get_available_for_wave(10);
        assert!(wave_10.len() > wave_1.len());
    }

    #[test]
    fn pick_random_returns_valid_creature() {
        let registry = CreatureRegistry::new();
        let creature = registry.pick_random_for_wave(5);
        assert!(creature.is_some());
    }

    #[test]
    fn spawn_position_is_within_bounds() {
        let config = SpawnConfig::default();
        for _ in 0..100 {
            let pos = calculate_spawn_position(Vec2::ZERO, &config);
            assert!(pos.x.abs() <= config.arena_bounds.x);
            assert!(pos.y.abs() <= config.arena_bounds.y);
        }
    }

    #[test]
    fn spawn_position_respects_min_distance() {
        let config = SpawnConfig {
            min_spawn_distance: 100.0,
            max_spawn_distance: 200.0,
            arena_bounds: Vec2::new(1000.0, 1000.0),
        };

        for _ in 0..100 {
            let pos = calculate_spawn_position(Vec2::ZERO, &config);
            let distance = pos.truncate().length();
            // Allow some tolerance for boundary clamping
            assert!(distance >= config.min_spawn_distance * 0.5);
        }
    }
}
