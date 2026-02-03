//! Perk components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// All available perk types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerkId {
    // Health & Defense
    Regeneration,
    ThickSkin,
    VitalityBoost,
    SecondChance,

    // Movement
    SpeedBoost,
    Dodger,
    Marathon,

    // Damage & Combat
    DeadlyAccuracy,
    CriticalHit,
    FastReload,
    LargerClips,
    DoubleBarrel,

    // Experience
    FastLearner,
    ExpMagnet,

    // Fire Rate
    TriggerHappy,
    Overcharge,

    // Special Effects
    PoisonBullets,
    FireBullets,
    FreezingBullets,
    ShockBullets,

    // Utility
    Highlander,
    LongBarrel,
    HollowPoints,
    ArmorPiercing,

    // Defensive
    Radioactive,
    DeathClock,
    Inferno,

    // Exotic
    BulletTime,
    Telekinesis,
    Pyromaniac,
}

impl PerkId {
    /// Returns all perk IDs for iteration
    pub fn all() -> &'static [PerkId] {
        &[
            PerkId::Regeneration,
            PerkId::ThickSkin,
            PerkId::VitalityBoost,
            PerkId::SecondChance,
            PerkId::SpeedBoost,
            PerkId::Dodger,
            PerkId::Marathon,
            PerkId::DeadlyAccuracy,
            PerkId::CriticalHit,
            PerkId::FastReload,
            PerkId::LargerClips,
            PerkId::DoubleBarrel,
            PerkId::FastLearner,
            PerkId::ExpMagnet,
            PerkId::TriggerHappy,
            PerkId::Overcharge,
            PerkId::PoisonBullets,
            PerkId::FireBullets,
            PerkId::FreezingBullets,
            PerkId::ShockBullets,
            PerkId::Highlander,
            PerkId::LongBarrel,
            PerkId::HollowPoints,
            PerkId::ArmorPiercing,
            PerkId::Radioactive,
            PerkId::DeathClock,
            PerkId::Inferno,
            PerkId::BulletTime,
            PerkId::Telekinesis,
            PerkId::Pyromaniac,
        ]
    }
}

/// Component storing the player's acquired perks
#[derive(Component, Debug, Clone, Default)]
pub struct PerkInventory {
    /// Count of each perk type (some perks stack)
    counts: [u8; 32],
}

impl PerkInventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_perk(&mut self, perk: PerkId) {
        let index = perk as usize;
        if index < self.counts.len() {
            self.counts[index] = self.counts[index].saturating_add(1);
        }
    }

    pub fn has_perk(&self, perk: PerkId) -> bool {
        self.get_count(perk) > 0
    }

    pub fn get_count(&self, perk: PerkId) -> u8 {
        let index = perk as usize;
        if index < self.counts.len() {
            self.counts[index]
        } else {
            0
        }
    }

    pub fn total_perks(&self) -> u32 {
        self.counts.iter().map(|&c| c as u32).sum()
    }

    /// Creates an inventory with a single perk (for testing)
    #[cfg(test)]
    pub fn with_perk(perk: PerkId) -> Self {
        let mut inv = Self::new();
        inv.add_perk(perk);
        inv
    }
}

/// Computed perk bonuses for quick access during gameplay
#[derive(Component, Debug, Clone, Default)]
pub struct PerkBonuses {
    /// Health regeneration per second
    pub regen_per_second: f32,
    /// Damage reduction multiplier (0.0 to 1.0)
    pub damage_reduction: f32,
    /// Max health bonus (added to base)
    pub max_health_bonus: f32,
    /// Movement speed multiplier
    pub speed_multiplier: f32,
    /// Damage multiplier
    pub damage_multiplier: f32,
    /// Fire rate multiplier
    pub fire_rate_multiplier: f32,
    /// Reload speed multiplier
    pub reload_speed_multiplier: f32,
    /// Ammo capacity multiplier
    pub ammo_multiplier: f32,
    /// Experience gain multiplier
    pub exp_multiplier: f32,
    /// Critical hit chance (0.0 to 1.0)
    pub crit_chance: f32,
    /// Critical hit damage multiplier
    pub crit_multiplier: f32,
    /// Projectile range multiplier
    pub range_multiplier: f32,
    /// Accuracy bonus (reduces spread)
    pub accuracy_bonus: f32,
    /// Chance to dodge damage (0.0 to 1.0)
    pub dodge_chance: f32,
}

impl PerkBonuses {
    /// Recalculate bonuses from perk inventory
    pub fn calculate(inventory: &PerkInventory) -> Self {
        let mut bonuses = Self::default();

        // Health & Defense
        bonuses.regen_per_second = inventory.get_count(PerkId::Regeneration) as f32 * 2.0;
        bonuses.damage_reduction = (inventory.get_count(PerkId::ThickSkin) as f32 * 0.1).min(0.5);
        bonuses.max_health_bonus = inventory.get_count(PerkId::VitalityBoost) as f32 * 25.0;

        // Movement
        bonuses.speed_multiplier =
            1.0 + (inventory.get_count(PerkId::SpeedBoost) as f32 * 0.15);
        bonuses.dodge_chance = (inventory.get_count(PerkId::Dodger) as f32 * 0.1).min(0.5);

        // Combat
        bonuses.accuracy_bonus = inventory.get_count(PerkId::DeadlyAccuracy) as f32 * 0.2;
        bonuses.crit_chance = (inventory.get_count(PerkId::CriticalHit) as f32 * 0.1).min(0.5);
        bonuses.crit_multiplier = 2.0; // Base crit damage
        bonuses.reload_speed_multiplier =
            1.0 + (inventory.get_count(PerkId::FastReload) as f32 * 0.25);
        bonuses.ammo_multiplier =
            1.0 + (inventory.get_count(PerkId::LargerClips) as f32 * 0.25);

        // Fire rate
        bonuses.fire_rate_multiplier =
            1.0 + (inventory.get_count(PerkId::TriggerHappy) as f32 * 0.15);

        // Damage
        bonuses.damage_multiplier =
            1.0 + (inventory.get_count(PerkId::HollowPoints) as f32 * 0.1);

        // Experience
        bonuses.exp_multiplier =
            1.0 + (inventory.get_count(PerkId::FastLearner) as f32 * 0.25);

        // Range
        bonuses.range_multiplier =
            1.0 + (inventory.get_count(PerkId::LongBarrel) as f32 * 0.2);

        bonuses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perk_inventory_starts_empty() {
        let inv = PerkInventory::new();
        assert_eq!(inv.total_perks(), 0);
    }

    #[test]
    fn perk_inventory_add_perk() {
        let mut inv = PerkInventory::new();
        inv.add_perk(PerkId::Regeneration);
        assert!(inv.has_perk(PerkId::Regeneration));
        assert_eq!(inv.get_count(PerkId::Regeneration), 1);
    }

    #[test]
    fn perk_inventory_perks_stack() {
        let mut inv = PerkInventory::new();
        inv.add_perk(PerkId::SpeedBoost);
        inv.add_perk(PerkId::SpeedBoost);
        inv.add_perk(PerkId::SpeedBoost);
        assert_eq!(inv.get_count(PerkId::SpeedBoost), 3);
    }

    #[test]
    fn perk_inventory_total_perks() {
        let mut inv = PerkInventory::new();
        inv.add_perk(PerkId::Regeneration);
        inv.add_perk(PerkId::SpeedBoost);
        inv.add_perk(PerkId::SpeedBoost);
        assert_eq!(inv.total_perks(), 3);
    }

    #[test]
    fn perk_bonuses_regen_scales_with_count() {
        let mut inv = PerkInventory::new();
        let bonuses = PerkBonuses::calculate(&inv);
        assert_eq!(bonuses.regen_per_second, 0.0);

        inv.add_perk(PerkId::Regeneration);
        let bonuses = PerkBonuses::calculate(&inv);
        assert_eq!(bonuses.regen_per_second, 2.0);

        inv.add_perk(PerkId::Regeneration);
        let bonuses = PerkBonuses::calculate(&inv);
        assert_eq!(bonuses.regen_per_second, 4.0);
    }

    #[test]
    fn perk_bonuses_damage_reduction_capped() {
        let mut inv = PerkInventory::new();
        for _ in 0..10 {
            inv.add_perk(PerkId::ThickSkin);
        }
        let bonuses = PerkBonuses::calculate(&inv);
        assert!(bonuses.damage_reduction <= 0.5);
    }

    #[test]
    fn perk_bonuses_speed_stacks() {
        let mut inv = PerkInventory::new();
        inv.add_perk(PerkId::SpeedBoost);
        inv.add_perk(PerkId::SpeedBoost);
        let bonuses = PerkBonuses::calculate(&inv);
        assert!((bonuses.speed_multiplier - 1.3).abs() < 0.001);
    }
}
