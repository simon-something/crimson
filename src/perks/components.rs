//! Perk components - All 58 original Crimsonland perks

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// All 58 original Crimsonland perks (index matches original game)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PerkId {
    // 0: Sentinel - never offered
    // 1: XP & Gore
    BloodyMess = 1,
    // 2: Accuracy
    Sharpshooter = 2,
    // 3: Reload
    Fastloader = 3,
    // 4: Passive XP
    LeanMeanExpMachine = 4,
    // 5: Movement
    LongDistanceRunner = 5,
    // 6: Fire aura
    Pyrokinetic = 6,
    // 7: Instant XP
    InstantWinner = 7,
    // 8: Risky XP
    GrimDeal = 8,
    // 9: Dual wield
    AlternateWeapon = 9,
    // 10: Infection
    Plaguebearer = 10,
    // 11: Freeze
    EvilEyes = 11,
    // 12: Clip size
    AmmoManiac = 12,
    // 13: Damage aura
    Radioactive = 13,
    // 14: Fire rate
    Fastshot = 14,
    // 15: 50/50 gamble
    FatalLottery = 15,
    // 16: Quest only
    RandomWeapon = 16,
    // 17: Melee counter
    MrMelee = 17,
    // 18: Reload while firing
    AnxiousLoader = 18,
    // 19: Death explosion
    FinalRevenge = 19,
    // 20: Remote pickup
    Telekinetic = 20,
    // 21: 6 perk choices
    PerkExpert = 21,
    // 22: No knockback
    Unstoppable = 22,
    // 23: Fire during reload (XP cost)
    RegressionBullets = 23,
    // 24: Health to 0.1, +3 levels
    InfernalContract = 24,
    // 25: Poison chance
    PoisonBullets = 25,
    // 26: 20% dodge
    Dodger = 26,
    // 27: More bonus spawns
    BonusMagnet = 27,
    // 28: 2x damage
    UraniumFilledBullets = 28,
    // 29: 1.2x damage + health display
    Doctor = 29,
    // 30: Creature highlights
    MonsterVision = 30,
    // 31: Periodic shot ring
    HotTempered = 31,
    // 32: Longer bonus duration
    BonusEconomist = 32,
    // 33: Reduced health/damage taken
    ThickSkinned = 33,
    // 34: 1.4x damage + faster projectiles
    BarrelGreaser = 34,
    // 35: Fire during reload (health cost)
    AmmunitionWithin = 35,
    // 36: Poison on melee contact
    VeinsOfPoison = 36,
    // 37: Strong poison on contact
    ToxicAvenger = 37,
    // 38: Health regen
    Regeneration = 38,
    // 39: Fire damage bonus
    Pyromaniac = 39,
    // 40: 33% dodge
    Ninja = 40,
    // 41: 10% instant kill
    Highlander = 41,
    // 42: Random effects
    Jinxed = 42,
    // 43: 7 perk choices
    PerkMaster = 43,
    // 44: Slow motion
    ReflexBoosted = 44,
    // 45: Better regen
    GreaterRegeneration = 45,
    // 46: Two-player only
    BreathingRoom = 46,
    // 47: Health drain + immunity
    DeathClock = 47,
    // 48: Fixed clip bonus
    MyFavouriteWeapon = 48,
    // 49: Random heal
    Bandage = 49,
    // 50: Ring on reload
    AngryReloader = 50,
    // 51: Ion weapon bonus
    IonGunMaster = 51,
    // 52: Fast reload while still
    StationaryReloader = 52,
    // 53: Ion ring while still
    ManBomb = 53,
    // 54: Periodic fire shot
    FireCough = 54,
    // 55: Damage scales with stillness
    LivingFortress = 55,
    // 56: Less damage during reload
    ToughReloader = 56,
    // 57: Remove half creatures
    Lifeline5050 = 57,
}

impl PerkId {
    /// Returns all perk IDs for iteration (excluding sentinel and quest-only)
    pub fn all() -> &'static [PerkId] {
        &[
            PerkId::BloodyMess,
            PerkId::Sharpshooter,
            PerkId::Fastloader,
            PerkId::LeanMeanExpMachine,
            PerkId::LongDistanceRunner,
            PerkId::Pyrokinetic,
            PerkId::InstantWinner,
            PerkId::GrimDeal,
            PerkId::AlternateWeapon,
            PerkId::Plaguebearer,
            PerkId::EvilEyes,
            PerkId::AmmoManiac,
            PerkId::Radioactive,
            PerkId::Fastshot,
            PerkId::FatalLottery,
            // RandomWeapon (16) is quest-only, excluded
            PerkId::MrMelee,
            PerkId::AnxiousLoader,
            PerkId::FinalRevenge,
            PerkId::Telekinetic,
            PerkId::PerkExpert,
            PerkId::Unstoppable,
            PerkId::RegressionBullets,
            PerkId::InfernalContract,
            PerkId::PoisonBullets,
            PerkId::Dodger,
            PerkId::BonusMagnet,
            PerkId::UraniumFilledBullets,
            PerkId::Doctor,
            PerkId::MonsterVision,
            PerkId::HotTempered,
            PerkId::BonusEconomist,
            PerkId::ThickSkinned,
            PerkId::BarrelGreaser,
            PerkId::AmmunitionWithin,
            PerkId::VeinsOfPoison,
            PerkId::ToxicAvenger,
            PerkId::Regeneration,
            PerkId::Pyromaniac,
            PerkId::Ninja,
            PerkId::Highlander,
            PerkId::Jinxed,
            PerkId::PerkMaster,
            PerkId::ReflexBoosted,
            PerkId::GreaterRegeneration,
            // BreathingRoom (46) is two-player only, excluded for now
            PerkId::DeathClock,
            PerkId::MyFavouriteWeapon,
            PerkId::Bandage,
            PerkId::AngryReloader,
            PerkId::IonGunMaster,
            PerkId::StationaryReloader,
            PerkId::ManBomb,
            PerkId::FireCough,
            PerkId::LivingFortress,
            PerkId::ToughReloader,
            PerkId::Lifeline5050,
        ]
    }

    /// Returns the number of perk choices based on PerkExpert/PerkMaster perks
    pub fn perk_choice_count(inventory: &PerkInventory) -> usize {
        if inventory.has_perk(PerkId::PerkMaster) {
            7
        } else if inventory.has_perk(PerkId::PerkExpert) {
            6
        } else {
            4
        }
    }
}

/// Component storing the player's acquired perks
#[derive(Component, Debug, Clone)]
pub struct PerkInventory {
    /// Count of each perk type (some perks stack)
    counts: [u8; 64],
}

impl Default for PerkInventory {
    fn default() -> Self {
        Self { counts: [0; 64] }
    }
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
}

/// Computed perk bonuses for quick access during gameplay
#[derive(Component, Debug, Clone)]
pub struct PerkBonuses {
    // === XP & Progression ===
    /// Experience gain multiplier (BloodyMess: +30%)
    pub exp_multiplier: f32,
    /// Passive XP per second (LeanMeanExpMachine)
    pub passive_xp_per_second: f32,

    // === Movement ===
    /// Movement speed multiplier (LongDistanceRunner ramps to 2.8)
    pub speed_multiplier: f32,
    /// No knockback on damage (Unstoppable)
    pub unstoppable: bool,

    // === Damage Output ===
    /// Base damage multiplier
    pub damage_multiplier: f32,
    /// Fire damage multiplier (Pyromaniac: 1.5x)
    pub fire_damage_multiplier: f32,
    /// Ion damage multiplier (IonGunMaster: 1.2x)
    pub ion_damage_multiplier: f32,
    /// Ion AoE radius multiplier (IonGunMaster: 1.2x)
    pub ion_aoe_multiplier: f32,
    /// Instant kill chance per hit (Highlander: 10%)
    pub instant_kill_chance: f32,
    /// Projectile speed multiplier (BarrelGreaser)
    pub projectile_speed_multiplier: f32,

    // === Accuracy & Fire Rate ===
    /// Spread multiplier (Sharpshooter: tighter)
    pub spread_multiplier: f32,
    /// Accuracy bonus (0.0-1.0, reduces spread)
    pub accuracy_bonus: f32,
    /// Fire rate multiplier (Fastshot: 0.88 cooldown)
    pub fire_rate_multiplier: f32,
    /// Critical hit chance (Highlander uses instant_kill instead)
    pub crit_chance: f32,
    /// Critical hit damage multiplier
    pub crit_multiplier: f32,
    /// Projectile range/lifetime multiplier
    pub range_multiplier: f32,

    // === Ammo & Reload ===
    /// Ammo pickup multiplier
    pub ammo_multiplier: f32,
    /// Clip size bonus (AmmoManiac: +25%)
    pub clip_size_multiplier: f32,
    /// Fixed clip bonus (MyFavouriteWeapon: +2)
    pub clip_size_bonus: i32,
    /// Reload speed multiplier (Fastloader: 0.7)
    pub reload_speed_multiplier: f32,
    /// Stationary reload multiplier (StationaryReloader: 3.0)
    pub stationary_reload_multiplier: f32,
    /// Can fire during reload using XP (RegressionBullets)
    pub regression_bullets: bool,
    /// Can fire during reload using health (AmmunitionWithin)
    pub ammunition_within: bool,
    /// Reduce reload by firing (AnxiousLoader)
    pub anxious_loader: bool,

    // === Defense ===
    /// Health multiplier (ThickSkinned: 2/3)
    pub max_health_multiplier: f32,
    /// Damage taken multiplier (ThickSkinned: 2/3)
    pub damage_taken_multiplier: f32,
    /// Damage reduction (0.0-1.0, derived from damage_taken_multiplier)
    pub damage_reduction: f32,
    /// Damage taken during reload (ToughReloader: 0.5)
    pub reload_damage_multiplier: f32,
    /// Dodge chance (Dodger: 20%, Ninja: 33%)
    pub dodge_chance: f32,
    /// Health regen per second
    pub regen_per_second: f32,

    // === Status Effects ===
    /// Poison bullet chance (PoisonBullets: 12.5%)
    pub poison_chance: f32,
    /// Poison on melee contact (VeinsOfPoison, ToxicAvenger)
    pub poison_on_contact: bool,
    /// Strong poison on contact (ToxicAvenger)
    pub toxic_avenger: bool,

    // === Auras & Periodic Effects ===
    /// Radioactive damage aura active
    pub radioactive_aura: bool,
    /// Pyrokinetic heat aura active
    pub pyrokinetic_aura: bool,
    /// Hot Tempered periodic ring
    pub hot_tempered: bool,
    /// Fire Cough periodic projectile
    pub fire_cough: bool,
    /// Man Bomb ion ring while stationary
    pub man_bomb: bool,
    /// Angry Reloader ring on reload
    pub angry_reloader: bool,

    // === Utility ===
    /// Remote bonus pickup range (Telekinetic)
    pub telekinetic_range: f32,
    /// Bonus spawn chance multiplier (BonusMagnet)
    pub bonus_spawn_multiplier: f32,
    /// Timed bonus duration multiplier (BonusEconomist: 1.5)
    pub bonus_duration_multiplier: f32,
    /// Creature health display (Doctor, MonsterVision)
    pub show_creature_health: bool,
    /// Creature highlights (MonsterVision)
    pub monster_vision: bool,
    /// Number of perk choices
    pub perk_choices: usize,

    // === Special Mechanics ===
    /// Has alternate weapon slot
    pub alternate_weapon: bool,
    /// Global time scale (ReflexBoosted: 0.9)
    pub time_scale: f32,
    /// Melee counter damage (MrMelee: 25)
    pub melee_counter_damage: f32,
    /// Explode on death (FinalRevenge)
    pub final_revenge: bool,
    /// Death clock active (health drain + immunity)
    pub death_clock: bool,
    /// Plaguebearer infection active
    pub plaguebearer: bool,
    /// Evil Eyes freeze on aim
    pub evil_eyes: bool,
    /// Jinxed random effects
    pub jinxed: bool,
    /// Living Fortress stationary damage bonus
    pub living_fortress: bool,
    /// Disable weapon bonuses (MyFavouriteWeapon)
    pub disable_weapon_bonuses: bool,
}

impl Default for PerkBonuses {
    fn default() -> Self {
        Self {
            exp_multiplier: 1.0,
            passive_xp_per_second: 0.0,
            speed_multiplier: 1.0,
            unstoppable: false,
            damage_multiplier: 1.0,
            fire_damage_multiplier: 1.0,
            ion_damage_multiplier: 1.0,
            ion_aoe_multiplier: 1.0,
            instant_kill_chance: 0.0,
            projectile_speed_multiplier: 1.0,
            spread_multiplier: 1.0,
            accuracy_bonus: 0.0,
            fire_rate_multiplier: 1.0,
            crit_chance: 0.0,
            crit_multiplier: 2.0,
            range_multiplier: 1.0,
            ammo_multiplier: 1.0,
            clip_size_multiplier: 1.0,
            clip_size_bonus: 0,
            reload_speed_multiplier: 1.0,
            stationary_reload_multiplier: 1.0,
            regression_bullets: false,
            ammunition_within: false,
            anxious_loader: false,
            max_health_multiplier: 1.0,
            damage_taken_multiplier: 1.0,
            damage_reduction: 0.0,
            reload_damage_multiplier: 1.0,
            dodge_chance: 0.0,
            regen_per_second: 0.0,
            poison_chance: 0.0,
            poison_on_contact: false,
            toxic_avenger: false,
            radioactive_aura: false,
            pyrokinetic_aura: false,
            hot_tempered: false,
            fire_cough: false,
            man_bomb: false,
            angry_reloader: false,
            telekinetic_range: 0.0,
            bonus_spawn_multiplier: 1.0,
            bonus_duration_multiplier: 1.0,
            show_creature_health: false,
            monster_vision: false,
            perk_choices: 4,
            alternate_weapon: false,
            time_scale: 1.0,
            melee_counter_damage: 0.0,
            final_revenge: false,
            death_clock: false,
            plaguebearer: false,
            evil_eyes: false,
            jinxed: false,
            living_fortress: false,
            disable_weapon_bonuses: false,
        }
    }
}

impl PerkBonuses {
    /// Recalculate bonuses from perk inventory
    pub fn calculate(inventory: &PerkInventory) -> Self {
        let mut bonuses = Self::default();

        // === XP Perks ===
        // BloodyMess: +30% XP
        if inventory.has_perk(PerkId::BloodyMess) {
            bonuses.exp_multiplier += 0.30;
        }
        // LeanMeanExpMachine: passive XP every 0.25s (4 XP/sec)
        if inventory.has_perk(PerkId::LeanMeanExpMachine) {
            bonuses.passive_xp_per_second = 4.0;
        }

        // === Movement ===
        // LongDistanceRunner: speed ramps to 2.8 (simplified to flat bonus)
        let runner_count = inventory.get_count(PerkId::LongDistanceRunner) as f32;
        bonuses.speed_multiplier = 1.0 + runner_count * 0.4; // Caps around 2.8 with multiple
        // Unstoppable: no knockback
        bonuses.unstoppable = inventory.has_perk(PerkId::Unstoppable);

        // === Damage Output ===
        // UraniumFilledBullets: 2x damage
        if inventory.has_perk(PerkId::UraniumFilledBullets) {
            bonuses.damage_multiplier *= 2.0;
        }
        // Doctor: 1.2x damage + health display
        if inventory.has_perk(PerkId::Doctor) {
            bonuses.damage_multiplier *= 1.2;
            bonuses.show_creature_health = true;
        }
        // BarrelGreaser: 1.4x damage + faster projectiles
        if inventory.has_perk(PerkId::BarrelGreaser) {
            bonuses.damage_multiplier *= 1.4;
            bonuses.projectile_speed_multiplier = 1.3;
        }
        // Pyromaniac: 1.5x fire damage
        if inventory.has_perk(PerkId::Pyromaniac) {
            bonuses.fire_damage_multiplier = 1.5;
        }
        // IonGunMaster: 1.2x ion damage and AoE
        if inventory.has_perk(PerkId::IonGunMaster) {
            bonuses.ion_damage_multiplier = 1.2;
            bonuses.ion_aoe_multiplier = 1.2;
        }
        // Highlander: 10% instant kill chance
        if inventory.has_perk(PerkId::Highlander) {
            bonuses.instant_kill_chance = 0.10;
        }

        // === Accuracy & Fire Rate ===
        // Sharpshooter: tighter spread (multiply by 0.5), slower firing handled elsewhere
        if inventory.has_perk(PerkId::Sharpshooter) {
            bonuses.spread_multiplier = 0.5;
            bonuses.accuracy_bonus = 0.5; // Derived: 1 - spread_multiplier
        }
        // Fastshot: cooldown * 0.88 (fire rate / 0.88 = faster)
        if inventory.has_perk(PerkId::Fastshot) {
            bonuses.fire_rate_multiplier = 1.0 / 0.88; // ~1.136x faster
        }
        // BarrelGreaser also improves range
        if inventory.has_perk(PerkId::BarrelGreaser) {
            bonuses.range_multiplier = 1.3;
        }

        // === Ammo & Reload ===
        // AmmoManiac: +25% clip size
        if inventory.has_perk(PerkId::AmmoManiac) {
            bonuses.clip_size_multiplier = 1.25;
        }
        // MyFavouriteWeapon: +2 clip, disable bonuses
        if inventory.has_perk(PerkId::MyFavouriteWeapon) {
            bonuses.clip_size_bonus = 2;
            bonuses.disable_weapon_bonuses = true;
        }
        // Fastloader: reload * 0.7
        if inventory.has_perk(PerkId::Fastloader) {
            bonuses.reload_speed_multiplier = 0.7;
        }
        // StationaryReloader: 3x reload speed while still
        if inventory.has_perk(PerkId::StationaryReloader) {
            bonuses.stationary_reload_multiplier = 3.0;
        }
        // RegressionBullets: fire during reload using XP
        bonuses.regression_bullets = inventory.has_perk(PerkId::RegressionBullets);
        // AmmunitionWithin: fire during reload using health
        bonuses.ammunition_within = inventory.has_perk(PerkId::AmmunitionWithin);
        // AnxiousLoader: reduce reload by firing
        bonuses.anxious_loader = inventory.has_perk(PerkId::AnxiousLoader);

        // === Defense ===
        // ThickSkinned: health to 2/3, damage taken to 2/3
        if inventory.has_perk(PerkId::ThickSkinned) {
            bonuses.max_health_multiplier = 2.0 / 3.0;
            bonuses.damage_taken_multiplier = 2.0 / 3.0;
            bonuses.damage_reduction = 1.0 / 3.0; // Derived: 1 - damage_taken_multiplier
        }
        // ToughReloader: 0.5x damage during reload
        if inventory.has_perk(PerkId::ToughReloader) {
            bonuses.reload_damage_multiplier = 0.5;
        }
        // Dodger: 20% dodge (1/5)
        if inventory.has_perk(PerkId::Dodger) {
            bonuses.dodge_chance = 0.20;
        }
        // Ninja: 33% dodge (1/3) - overrides Dodger if both
        if inventory.has_perk(PerkId::Ninja) {
            bonuses.dodge_chance = 1.0 / 3.0;
        }
        // Regeneration: passive healing
        let regen_count = inventory.get_count(PerkId::Regeneration) as f32;
        bonuses.regen_per_second = regen_count * 2.0;
        // GreaterRegeneration: improved regen
        if inventory.has_perk(PerkId::GreaterRegeneration) {
            bonuses.regen_per_second += 5.0;
        }

        // === Status Effects ===
        // PoisonBullets: 1/8 (12.5%) chance to poison
        if inventory.has_perk(PerkId::PoisonBullets) {
            bonuses.poison_chance = 0.125;
        }
        // VeinsOfPoison: poison attackers on contact
        bonuses.poison_on_contact = inventory.has_perk(PerkId::VeinsOfPoison);
        // ToxicAvenger: strong poison on contact
        bonuses.toxic_avenger = inventory.has_perk(PerkId::ToxicAvenger);

        // === Auras & Periodic Effects ===
        bonuses.radioactive_aura = inventory.has_perk(PerkId::Radioactive);
        bonuses.pyrokinetic_aura = inventory.has_perk(PerkId::Pyrokinetic);
        bonuses.hot_tempered = inventory.has_perk(PerkId::HotTempered);
        bonuses.fire_cough = inventory.has_perk(PerkId::FireCough);
        bonuses.man_bomb = inventory.has_perk(PerkId::ManBomb);
        bonuses.angry_reloader = inventory.has_perk(PerkId::AngryReloader);

        // === Utility ===
        // Telekinetic: remote pickup at distance
        if inventory.has_perk(PerkId::Telekinetic) {
            bonuses.telekinetic_range = 200.0;
        }
        // BonusMagnet: extra bonus spawn chance
        if inventory.has_perk(PerkId::BonusMagnet) {
            bonuses.bonus_spawn_multiplier = 1.5;
        }
        // BonusEconomist: timed bonuses +50% duration
        if inventory.has_perk(PerkId::BonusEconomist) {
            bonuses.bonus_duration_multiplier = 1.5;
        }
        // MonsterVision: creature highlights
        if inventory.has_perk(PerkId::MonsterVision) {
            bonuses.monster_vision = true;
            bonuses.show_creature_health = true;
        }
        // Perk choices
        bonuses.perk_choices = PerkId::perk_choice_count(inventory);

        // === Special Mechanics ===
        bonuses.alternate_weapon = inventory.has_perk(PerkId::AlternateWeapon);
        // ReflexBoosted: global slow motion (0.9x time)
        if inventory.has_perk(PerkId::ReflexBoosted) {
            bonuses.time_scale = 0.9;
        }
        // MrMelee: counter-hit for 25 damage
        if inventory.has_perk(PerkId::MrMelee) {
            bonuses.melee_counter_damage = 25.0;
        }
        // FinalRevenge: explosion on death
        bonuses.final_revenge = inventory.has_perk(PerkId::FinalRevenge);
        // DeathClock: health drain + immunity
        bonuses.death_clock = inventory.has_perk(PerkId::DeathClock);
        // Plaguebearer: infection system
        bonuses.plaguebearer = inventory.has_perk(PerkId::Plaguebearer);
        // EvilEyes: freeze targeted creature
        bonuses.evil_eyes = inventory.has_perk(PerkId::EvilEyes);
        // Jinxed: random effects
        bonuses.jinxed = inventory.has_perk(PerkId::Jinxed);
        // LivingFortress: damage scales with stationary time
        bonuses.living_fortress = inventory.has_perk(PerkId::LivingFortress);

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
        inv.add_perk(PerkId::Regeneration);
        inv.add_perk(PerkId::Regeneration);
        inv.add_perk(PerkId::Regeneration);
        assert_eq!(inv.get_count(PerkId::Regeneration), 3);
    }

    #[test]
    fn perk_inventory_total_perks() {
        let mut inv = PerkInventory::new();
        inv.add_perk(PerkId::Regeneration);
        inv.add_perk(PerkId::Dodger);
        inv.add_perk(PerkId::Dodger);
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
    fn perk_bonuses_dodge_ninja_overrides_dodger() {
        let mut inv = PerkInventory::new();
        inv.add_perk(PerkId::Dodger);
        let bonuses = PerkBonuses::calculate(&inv);
        assert!((bonuses.dodge_chance - 0.20).abs() < 0.001);

        inv.add_perk(PerkId::Ninja);
        let bonuses = PerkBonuses::calculate(&inv);
        assert!((bonuses.dodge_chance - 1.0 / 3.0).abs() < 0.001);
    }

    #[test]
    fn perk_bonuses_speed_stacks() {
        let mut inv = PerkInventory::new();
        inv.add_perk(PerkId::LongDistanceRunner);
        inv.add_perk(PerkId::LongDistanceRunner);
        let bonuses = PerkBonuses::calculate(&inv);
        assert!((bonuses.speed_multiplier - 1.8).abs() < 0.001);
    }

    #[test]
    fn perk_choices_increase_with_expert_master() {
        let mut inv = PerkInventory::new();
        assert_eq!(PerkId::perk_choice_count(&inv), 4);

        inv.add_perk(PerkId::PerkExpert);
        assert_eq!(PerkId::perk_choice_count(&inv), 6);

        inv.add_perk(PerkId::PerkMaster);
        assert_eq!(PerkId::perk_choice_count(&inv), 7);
    }
}
