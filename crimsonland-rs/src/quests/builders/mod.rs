//! Quest builders
//!
//! Contains specialized spawn logic for specific quests.
//! Each quest can have custom spawn patterns beyond the basic wave data.

// Builder modules for complex quests would go here
// For now, the basic wave spawning system handles most cases

/// Trait for custom quest spawn logic
pub trait QuestBuilder {
    /// Called each frame to handle custom spawning
    fn update(&mut self, delta: f32) -> Vec<SpawnCommand>;

    /// Returns true when the quest is complete
    fn is_complete(&self) -> bool;
}

/// Command to spawn a creature
pub struct SpawnCommand {
    pub creature_type: crate::creatures::components::CreatureType,
    pub position: Option<bevy::prelude::Vec3>,
}
