//! Item components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Types of items the player can carry and activate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    /// Kills all creatures on screen
    Nuke,
    /// Freezes all creatures for a duration
    Freeze,
    /// Shield that absorbs damage
    Shield,
    /// Fires plasma in all directions
    PlasmaBlast,
    /// Slows down time
    TimeWarp,
    /// Grants temporary invincibility
    Invincibility,
    /// Fires homing missiles at all enemies
    MissileSalvo,
    /// Creates a damaging explosion around player
    Shockwave,
    /// Poisons all nearby enemies
    ToxicCloud,
    /// Doubles fire rate temporarily
    Overdrive,
}

impl ItemType {
    /// Get the display name
    pub fn name(&self) -> &'static str {
        match self {
            ItemType::Nuke => "Nuke",
            ItemType::Freeze => "Freeze",
            ItemType::Shield => "Shield",
            ItemType::PlasmaBlast => "Plasma Blast",
            ItemType::TimeWarp => "Time Warp",
            ItemType::Invincibility => "Invincibility",
            ItemType::MissileSalvo => "Missile Salvo",
            ItemType::Shockwave => "Shockwave",
            ItemType::ToxicCloud => "Toxic Cloud",
            ItemType::Overdrive => "Overdrive",
        }
    }

    /// Get the color for UI/pickup display
    pub fn color(&self) -> Color {
        match self {
            ItemType::Nuke => Color::srgb(1.0, 0.8, 0.0),        // Gold
            ItemType::Freeze => Color::srgb(0.5, 0.8, 1.0),      // Ice blue
            ItemType::Shield => Color::srgb(0.3, 0.5, 1.0),      // Blue
            ItemType::PlasmaBlast => Color::srgb(0.8, 0.2, 1.0), // Purple
            ItemType::TimeWarp => Color::srgb(0.6, 0.3, 0.8),    // Violet
            ItemType::Invincibility => Color::srgb(1.0, 1.0, 1.0), // White
            ItemType::MissileSalvo => Color::srgb(1.0, 0.4, 0.2), // Orange
            ItemType::Shockwave => Color::srgb(1.0, 0.6, 0.0),   // Orange-yellow
            ItemType::ToxicCloud => Color::srgb(0.4, 0.8, 0.2),  // Green
            ItemType::Overdrive => Color::srgb(1.0, 0.2, 0.2),   // Red
        }
    }

    /// Spawn weight for random item selection
    pub fn spawn_weight(&self) -> u32 {
        match self {
            ItemType::Nuke => 2,           // Very rare
            ItemType::Freeze => 8,
            ItemType::Shield => 10,
            ItemType::PlasmaBlast => 5,
            ItemType::TimeWarp => 4,
            ItemType::Invincibility => 3,  // Rare
            ItemType::MissileSalvo => 6,
            ItemType::Shockwave => 7,
            ItemType::ToxicCloud => 6,
            ItemType::Overdrive => 8,
        }
    }

    /// Pick a random item type weighted by spawn weights
    pub fn random() -> Self {
        use rand::Rng;
        let items = [
            ItemType::Nuke,
            ItemType::Freeze,
            ItemType::Shield,
            ItemType::PlasmaBlast,
            ItemType::TimeWarp,
            ItemType::Invincibility,
            ItemType::MissileSalvo,
            ItemType::Shockwave,
            ItemType::ToxicCloud,
            ItemType::Overdrive,
        ];

        let total_weight: u32 = items.iter().map(|i| i.spawn_weight()).sum();
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..total_weight);

        let mut cumulative = 0;
        for item in items {
            cumulative += item.spawn_weight();
            if roll < cumulative {
                return item;
            }
        }

        ItemType::Shield // Fallback
    }
}

/// Component for the player's currently carried item
#[derive(Component, Debug, Clone, Default)]
pub struct CarriedItem {
    /// The item type (None if no item carried)
    pub item: Option<ItemType>,
}

impl CarriedItem {
    pub fn new() -> Self {
        Self { item: None }
    }

    pub fn set_item(&mut self, item: ItemType) {
        self.item = Some(item);
    }

    pub fn take_item(&mut self) -> Option<ItemType> {
        self.item.take()
    }
}

/// Marker component for item pickup entities in the world
#[derive(Component, Debug, Clone)]
pub struct ItemPickup {
    pub item_type: ItemType,
}

/// Lifetime component for item pickups
#[derive(Component, Debug, Clone)]
pub struct ItemLifetime {
    pub remaining: f32,
}

impl Default for ItemLifetime {
    fn default() -> Self {
        Self { remaining: 20.0 } // Items last 20 seconds
    }
}

/// Bundle for spawning item pickups
#[derive(Bundle)]
pub struct ItemPickupBundle {
    pub pickup: ItemPickup,
    pub lifetime: ItemLifetime,
    pub sprite: SpriteBundle,
}

impl ItemPickupBundle {
    pub fn new(item_type: ItemType, position: Vec3) -> Self {
        Self {
            pickup: ItemPickup { item_type },
            lifetime: ItemLifetime::default(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: item_type.color(),
                    custom_size: Some(Vec2::splat(24.0)), // Larger than bonuses
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
    fn item_type_has_names() {
        assert!(!ItemType::Nuke.name().is_empty());
        assert!(!ItemType::Freeze.name().is_empty());
    }

    #[test]
    fn carried_item_starts_empty() {
        let carried = CarriedItem::new();
        assert!(!carried.has_item());
    }

    #[test]
    fn carried_item_can_set_and_take() {
        let mut carried = CarriedItem::new();
        carried.set_item(ItemType::Nuke);
        assert!(carried.has_item());

        let taken = carried.take_item();
        assert_eq!(taken, Some(ItemType::Nuke));
        assert!(!carried.has_item());
    }

    #[test]
    fn random_item_returns_valid_type() {
        // Just verify it doesn't panic
        for _ in 0..10 {
            let _item = ItemType::random();
        }
    }
}
