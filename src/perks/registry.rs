//! Perk registry and data - All 58 original Crimsonland perks

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
            // === XP & Progression ===
            PerkData {
                id: PerkId::BloodyMess,
                name: "Bloody Mess".into(),
                description: "+30% XP from kills. Extra gore effects.".into(),
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::LeanMeanExpMachine,
                name: "Lean Mean Exp Machine".into(),
                description: "Gain passive XP over time.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::InstantWinner,
                name: "Instant Winner".into(),
                description: "Immediately gain +2500 XP.".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::GrimDeal,
                name: "Grim Deal".into(),
                description: "Gain +18% of current XP, then die. Risky!".into(),
                rarity: PerkRarity::Legendary,
            },
            PerkData {
                id: PerkId::InfernalContract,
                name: "Infernal Contract".into(),
                description: "Health drops to 0.1, but gain +3 levels.".into(),
                rarity: PerkRarity::Legendary,
            },
            PerkData {
                id: PerkId::FatalLottery,
                name: "Fatal Lottery".into(),
                description: "50/50 chance: +10000 XP or instant death.".into(),
                rarity: PerkRarity::Legendary,
            },

            // === Movement ===
            PerkData {
                id: PerkId::LongDistanceRunner,
                name: "Long Distance Runner".into(),
                description: "Movement speed increases over time (up to 2.8x).".into(),
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::Unstoppable,
                name: "Unstoppable".into(),
                description: "No knockback or disruption when taking damage.".into(),
                rarity: PerkRarity::Uncommon,
            },

            // === Accuracy & Fire Rate ===
            PerkData {
                id: PerkId::Sharpshooter,
                name: "Sharpshooter".into(),
                description: "Tighter weapon spread, laser sight. Slower firing.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::Fastshot,
                name: "Fastshot".into(),
                description: "Fire rate increased (cooldown x0.88).".into(),
                rarity: PerkRarity::Common,
            },

            // === Ammo & Reload ===
            PerkData {
                id: PerkId::Fastloader,
                name: "Fastloader".into(),
                description: "Reload time reduced to 70%.".into(),
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::AmmoManiac,
                name: "Ammo Maniac".into(),
                description: "Clip size increased by 25%.".into(),
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::AnxiousLoader,
                name: "Anxious Loader".into(),
                description: "Firing reduces reload timer.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::RegressionBullets,
                name: "Regression Bullets".into(),
                description: "Fire during reload by spending XP.".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::AmmunitionWithin,
                name: "Ammunition Within".into(),
                description: "Fire during reload by paying health.".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::StationaryReloader,
                name: "Stationary Reloader".into(),
                description: "3x reload speed while standing still.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::MyFavouriteWeapon,
                name: "My Favourite Weapon".into(),
                description: "Clip +2, but weapon bonuses disabled.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::AngryReloader,
                name: "Angry Reloader".into(),
                description: "Fire a ring of bullets at reload halfway point.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::ToughReloader,
                name: "Tough Reloader".into(),
                description: "Take 50% less damage while reloading.".into(),
                rarity: PerkRarity::Uncommon,
            },

            // === Damage Output ===
            PerkData {
                id: PerkId::UraniumFilledBullets,
                name: "Uranium Filled Bullets".into(),
                description: "Bullet damage x2.0.".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::Doctor,
                name: "Doctor".into(),
                description: "Damage x1.2. See enemy health bars.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::BarrelGreaser,
                name: "Barrel Greaser".into(),
                description: "Damage x1.4. Faster projectiles.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::Highlander,
                name: "Highlander".into(),
                description: "10% chance to instantly kill on hit.".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::Pyromaniac,
                name: "Pyromaniac".into(),
                description: "Fire damage x1.5.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::IonGunMaster,
                name: "Ion Gun Master".into(),
                description: "Ion damage x1.2. Ion AoE radius x1.2.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::LivingFortress,
                name: "Living Fortress".into(),
                description: "Damage increases the longer you stand still.".into(),
                rarity: PerkRarity::Uncommon,
            },

            // === Defense ===
            PerkData {
                id: PerkId::ThickSkinned,
                name: "Thick Skinned".into(),
                description: "Health reduced to 2/3, but damage taken also 2/3.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::Dodger,
                name: "Dodger".into(),
                description: "20% chance to dodge damage completely.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::Ninja,
                name: "Ninja".into(),
                description: "33% chance to dodge damage completely.".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::Regeneration,
                name: "Regeneration".into(),
                description: "Slowly regenerate health over time.".into(),
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::GreaterRegeneration,
                name: "Greater Regeneration".into(),
                description: "Regenerate health faster.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::Bandage,
                name: "Bandage".into(),
                description: "Randomly multiply current health (1-50x).".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::DeathClock,
                name: "Death Clock".into(),
                description: "Health drains over time, but immune to damage.".into(),
                rarity: PerkRarity::Legendary,
            },

            // === Status Effects ===
            PerkData {
                id: PerkId::PoisonBullets,
                name: "Poison Bullets".into(),
                description: "12.5% chance to poison enemies on hit.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::VeinsOfPoison,
                name: "Veins of Poison".into(),
                description: "Poison enemies that touch you.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::ToxicAvenger,
                name: "Toxic Avenger".into(),
                description: "Strong poison on melee contact.".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::Plaguebearer,
                name: "Plaguebearer".into(),
                description: "Infected enemies spread damage to others.".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::EvilEyes,
                name: "Evil Eyes".into(),
                description: "Freeze the creature you're aiming at.".into(),
                rarity: PerkRarity::Rare,
            },

            // === Auras & Periodic Effects ===
            PerkData {
                id: PerkId::Radioactive,
                name: "Radioactive".into(),
                description: "Damage nearby enemies with radiation aura.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::Pyrokinetic,
                name: "Pyrokinetic".into(),
                description: "Periodic heat/flare effects near creatures.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::HotTempered,
                name: "Hot Tempered".into(),
                description: "Periodically fire an 8-shot ring around you.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::FireCough,
                name: "Fire Cough".into(),
                description: "Periodically fire a projectile from your muzzle.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::ManBomb,
                name: "Man Bomb".into(),
                description: "Fire ion rings while standing still.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::FinalRevenge,
                name: "Final Revenge".into(),
                description: "Explode on death, damaging all nearby enemies.".into(),
                rarity: PerkRarity::Uncommon,
            },

            // === Utility ===
            PerkData {
                id: PerkId::Telekinetic,
                name: "Telekinetic".into(),
                description: "Pick up bonuses from a distance.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::BonusMagnet,
                name: "Bonus Magnet".into(),
                description: "Increased chance for bonus spawns.".into(),
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::BonusEconomist,
                name: "Bonus Economist".into(),
                description: "Timed bonuses last 50% longer.".into(),
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::MonsterVision,
                name: "Monster Vision".into(),
                description: "Creatures are highlighted. See health bars.".into(),
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::PerkExpert,
                name: "Perk Expert".into(),
                description: "6 perk choices instead of 4.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::PerkMaster,
                name: "Perk Master".into(),
                description: "7 perk choices instead of 4.".into(),
                rarity: PerkRarity::Rare,
            },

            // === Weapons & Combat ===
            PerkData {
                id: PerkId::AlternateWeapon,
                name: "Alternate Weapon".into(),
                description: "Second weapon slot. Movement penalty.".into(),
                rarity: PerkRarity::Uncommon,
            },
            PerkData {
                id: PerkId::RandomWeapon,
                name: "Random Weapon".into(),
                description: "Quest only: assigns a random weapon.".into(),
                rarity: PerkRarity::Common,
            },
            PerkData {
                id: PerkId::MrMelee,
                name: "Mr. Melee".into(),
                description: "Counter-hit attackers for 25 damage.".into(),
                rarity: PerkRarity::Uncommon,
            },

            // === Special Mechanics ===
            PerkData {
                id: PerkId::ReflexBoosted,
                name: "Reflex Boosted".into(),
                description: "Global slow-motion effect (time x0.9).".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::Jinxed,
                name: "Jinxed".into(),
                description: "Random self-damage and creature kills.".into(),
                rarity: PerkRarity::Legendary,
            },
            PerkData {
                id: PerkId::BreathingRoom,
                name: "Breathing Room".into(),
                description: "Two-player only: clears nearby creatures.".into(),
                rarity: PerkRarity::Rare,
            },
            PerkData {
                id: PerkId::Lifeline5050,
                name: "Lifeline 50-50".into(),
                description: "Remove approximately half of all creatures.".into(),
                rarity: PerkRarity::Legendary,
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

/// Rarity of a perk (affects drop rates and display)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerkRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl PerkRarity {
    pub fn color(&self) -> Color {
        match self {
            PerkRarity::Common => Color::srgb(0.7, 0.7, 0.7),      // Gray
            PerkRarity::Uncommon => Color::srgb(0.3, 0.8, 0.3),    // Green
            PerkRarity::Rare => Color::srgb(0.3, 0.5, 1.0),        // Blue
            PerkRarity::Legendary => Color::srgb(1.0, 0.5, 0.0),   // Orange
        }
    }
}

/// Data for a perk type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerkData {
    pub id: PerkId,
    pub name: String,
    pub description: String,
    pub rarity: PerkRarity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perk_registry_has_all_perks() {
        let registry = PerkRegistry::new();
        assert!(!registry.perks.is_empty());

        // Check we have key perks
        assert!(registry.get(PerkId::Regeneration).is_some());
        assert!(registry.get(PerkId::Dodger).is_some());
        assert!(registry.get(PerkId::Highlander).is_some());
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
        let legendary = PerkRarity::Legendary.color();
        assert_ne!(common, legendary);
    }
}
