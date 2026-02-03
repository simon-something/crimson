//! Perk registry and data

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::components::PerkId;

/// Registry containing all perk definitions
#[derive(Resource)]
pub struct PerkRegistry {
    pub perks: Vec<PerkData>,
}

impl Default for PerkRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PerkRegistry {
    pub fn new() -> Self {
        let mut registry = Self { perks: Vec::new() };
        registry.register_all_perks();
        registry
    }

    pub fn get(&self, id: PerkId) -> Option<&PerkData> {
        self.perks.iter().find(|p| p.id == id)
    }

    fn register_all_perks(&mut self) {
        self.perks = vec![
            // Health & Defense
            PerkData {
                id: PerkId::Regeneration,
                name: "Regeneration".into(),
                description: "Slowly regenerate health over time.".into(),
                stackable: true,
                max_stacks: 5,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::ThickSkin,
                name: "Thick Skin".into(),
                description: "Reduce all incoming damage by 10%.".into(),
                stackable: true,
                max_stacks: 5,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::VitalityBoost,
                name: "Vitality Boost".into(),
                description: "Increase maximum health by 25.".into(),
                stackable: true,
                max_stacks: 4,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::SecondChance,
                name: "Second Chance".into(),
                description: "Survive a killing blow with 1 HP (once per life).".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Rare,
            },
            // Movement
            PerkData {
                id: PerkId::SpeedBoost,
                name: "Speed Boost".into(),
                description: "Increase movement speed by 15%.".into(),
                stackable: true,
                max_stacks: 5,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::Dodger,
                name: "Dodger".into(),
                description: "10% chance to completely avoid damage.".into(),
                stackable: true,
                max_stacks: 5,
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::Marathon,
                name: "Marathon".into(),
                description: "Never slow down, even when taking damage.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Uncommon,
            },
            // Damage & Combat
            PerkData {
                id: PerkId::DeadlyAccuracy,
                name: "Deadly Accuracy".into(),
                description: "Reduce weapon spread by 20%.".into(),
                stackable: true,
                max_stacks: 5,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::CriticalHit,
                name: "Critical Hit".into(),
                description: "10% chance for double damage.".into(),
                stackable: true,
                max_stacks: 5,
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::FastReload,
                name: "Fast Reload".into(),
                description: "Reload 25% faster.".into(),
                stackable: true,
                max_stacks: 4,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::LargerClips,
                name: "Larger Clips".into(),
                description: "Increase ammo capacity by 25%.".into(),
                stackable: true,
                max_stacks: 4,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::DoubleBarrel,
                name: "Double Barrel".into(),
                description: "Fire two projectiles instead of one.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Rare,
            },
            // Experience
            PerkData {
                id: PerkId::FastLearner,
                name: "Fast Learner".into(),
                description: "Gain 25% more experience.".into(),
                stackable: true,
                max_stacks: 4,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::ExpMagnet,
                name: "Experience Magnet".into(),
                description: "Automatically collect nearby experience.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Uncommon,
            },
            // Fire Rate
            PerkData {
                id: PerkId::TriggerHappy,
                name: "Trigger Happy".into(),
                description: "Increase fire rate by 15%.".into(),
                stackable: true,
                max_stacks: 5,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::Overcharge,
                name: "Overcharge".into(),
                description: "Weapons fire 50% faster but use more ammo.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Rare,
            },
            // Special Effects
            PerkData {
                id: PerkId::PoisonBullets,
                name: "Poison Bullets".into(),
                description: "Bullets poison enemies, dealing damage over time.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::FireBullets,
                name: "Fire Bullets".into(),
                description: "Bullets set enemies on fire.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::FreezingBullets,
                name: "Freezing Bullets".into(),
                description: "Bullets slow enemies.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::ShockBullets,
                name: "Shock Bullets".into(),
                description: "Bullets chain lightning to nearby enemies.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Rare,
            },
            // Utility
            PerkData {
                id: PerkId::Highlander,
                name: "Highlander".into(),
                description: "Deal 100% more damage when at critical health.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::LongBarrel,
                name: "Long Barrel".into(),
                description: "Increase projectile range by 20%.".into(),
                stackable: true,
                max_stacks: 5,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::HollowPoints,
                name: "Hollow Points".into(),
                description: "Deal 10% more damage.".into(),
                stackable: true,
                max_stacks: 5,
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::ArmorPiercing,
                name: "Armor Piercing".into(),
                description: "Bullets pass through enemies.".into(),
                stackable: true,
                max_stacks: 3,
                rarity: PerkRarity::Uncommon,
            },
            // Defensive
            PerkData {
                id: PerkId::Radioactive,
                name: "Radioactive".into(),
                description: "Damage nearby enemies constantly.".into(),
                stackable: true,
                max_stacks: 3,
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::DeathClock,
                name: "Death Clock".into(),
                description: "Enemies near you move slower.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::Inferno,
                name: "Inferno".into(),
                description: "Explode on death, damaging all nearby enemies.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Uncommon,
            },
            // Exotic
            PerkData {
                id: PerkId::BulletTime,
                name: "Bullet Time".into(),
                description: "Time slows when aiming at enemies.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Epic,
            },
            PerkData {
                id: PerkId::Telekinesis,
                name: "Telekinesis".into(),
                description: "Push enemies away from you.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Epic,
            },
            PerkData {
                id: PerkId::Pyromaniac,
                name: "Pyromaniac".into(),
                description: "All weapons deal fire damage and ignite enemies.".into(),
                stackable: false,
                max_stacks: 1,
                rarity: PerkRarity::Epic,
            },
        ];
    }

    /// Get a random selection of perks for the perk selection screen
    pub fn get_random_selection(&self, count: usize) -> Vec<&PerkData> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let mut shuffled: Vec<_> = self.perks.iter().collect();
        shuffled.shuffle(&mut rng);
        shuffled.into_iter().take(count).collect()
    }
}

/// Rarity of a perk (affects drop rates)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerkRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl PerkRarity {
    pub fn color(&self) -> Color {
        match self {
            PerkRarity::Common => Color::srgb(0.7, 0.7, 0.7),
            PerkRarity::Uncommon => Color::srgb(0.3, 0.8, 0.3),
            PerkRarity::Rare => Color::srgb(0.3, 0.5, 1.0),
            PerkRarity::Epic => Color::srgb(0.8, 0.3, 0.9),
        }
    }
}

/// Data for a perk type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerkData {
    pub id: PerkId,
    pub name: String,
    pub description: String,
    pub stackable: bool,
    pub max_stacks: u8,
    pub rarity: PerkRarity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perk_registry_has_all_perks() {
        let registry = PerkRegistry::new();
        assert!(!registry.perks.is_empty());

        // Check we have the key perks
        assert!(registry.get(PerkId::Regeneration).is_some());
        assert!(registry.get(PerkId::SpeedBoost).is_some());
        assert!(registry.get(PerkId::CriticalHit).is_some());
    }

    #[test]
    fn perk_data_has_description() {
        let registry = PerkRegistry::new();
        let regen = registry.get(PerkId::Regeneration).unwrap();
        assert!(!regen.description.is_empty());
    }

    #[test]
    fn random_selection_returns_correct_count() {
        let registry = PerkRegistry::new();
        let selection = registry.get_random_selection(4);
        assert_eq!(selection.len(), 4);
    }

    #[test]
    fn perk_rarities_have_distinct_colors() {
        let common = PerkRarity::Common.color();
        let epic = PerkRarity::Epic.color();
        assert_ne!(common, epic);
    }
}
