//! Weapon registry and data

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::components::WeaponId;

/// Registry containing all weapon definitions
#[derive(Resource)]
pub struct WeaponRegistry {
    pub weapons: Vec<WeaponData>,
}

impl Default for WeaponRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl WeaponRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            weapons: Vec::new(),
        };
        registry.register_all_weapons();
        registry
    }

    pub fn get(&self, id: WeaponId) -> Option<&WeaponData> {
        self.weapons.iter().find(|w| w.id == id)
    }

    fn register_all_weapons(&mut self) {
        self.weapons = vec![
            // Pistols
            WeaponData {
                id: WeaponId::Pistol,
                name: "Pistol".into(),
                damage: 15.0,
                fire_rate: 5.0,
                projectile_speed: 800.0,
                spread: 0.05,
                projectiles_per_shot: 1,
                ammo_capacity: None, // Infinite
                reload_time: 0.0,
                projectile_lifetime: 2.0,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::PocketRocket,
                name: "Pocket Rocket".into(),
                damage: 50.0,
                fire_rate: 2.0,
                projectile_speed: 500.0,
                spread: 0.02,
                projectiles_per_shot: 1,
                ammo_capacity: Some(30),
                reload_time: 1.5,
                projectile_lifetime: 3.0,
                pierce_count: 0,
                homing: false,
                explosive_radius: 50.0,
            },
            WeaponData {
                id: WeaponId::Magnum,
                name: "Magnum".into(),
                damage: 60.0,
                fire_rate: 2.0,
                projectile_speed: 1000.0,
                spread: 0.02,
                projectiles_per_shot: 1,
                ammo_capacity: Some(36),
                reload_time: 1.0,
                projectile_lifetime: 2.5,
                pierce_count: 1,
                homing: false,
                explosive_radius: 0.0,
            },
            // Submachine Guns
            WeaponData {
                id: WeaponId::Uzi,
                name: "Uzi".into(),
                damage: 10.0,
                fire_rate: 15.0,
                projectile_speed: 700.0,
                spread: 0.15,
                projectiles_per_shot: 1,
                ammo_capacity: Some(200),
                reload_time: 1.5,
                projectile_lifetime: 1.5,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::Smg,
                name: "SMG".into(),
                damage: 12.0,
                fire_rate: 12.0,
                projectile_speed: 750.0,
                spread: 0.1,
                projectiles_per_shot: 1,
                ammo_capacity: Some(250),
                reload_time: 1.5,
                projectile_lifetime: 1.5,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::DualSmg,
                name: "Dual SMG".into(),
                damage: 10.0,
                fire_rate: 20.0,
                projectile_speed: 750.0,
                spread: 0.2,
                projectiles_per_shot: 2,
                ammo_capacity: Some(400),
                reload_time: 2.0,
                projectile_lifetime: 1.5,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            // Rifles
            WeaponData {
                id: WeaponId::AssaultRifle,
                name: "Assault Rifle".into(),
                damage: 18.0,
                fire_rate: 10.0,
                projectile_speed: 900.0,
                spread: 0.08,
                projectiles_per_shot: 1,
                ammo_capacity: Some(300),
                reload_time: 1.5,
                projectile_lifetime: 2.0,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::MachineGun,
                name: "Machine Gun".into(),
                damage: 15.0,
                fire_rate: 14.0,
                projectile_speed: 850.0,
                spread: 0.12,
                projectiles_per_shot: 1,
                ammo_capacity: Some(500),
                reload_time: 2.0,
                projectile_lifetime: 2.0,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::Minigun,
                name: "Minigun".into(),
                damage: 12.0,
                fire_rate: 30.0,
                projectile_speed: 800.0,
                spread: 0.15,
                projectiles_per_shot: 1,
                ammo_capacity: Some(1000),
                reload_time: 3.0,
                projectile_lifetime: 1.5,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            // Shotguns
            WeaponData {
                id: WeaponId::Shotgun,
                name: "Shotgun".into(),
                damage: 8.0,
                fire_rate: 2.0,
                projectile_speed: 600.0,
                spread: 0.3,
                projectiles_per_shot: 8,
                ammo_capacity: Some(50),
                reload_time: 1.5,
                projectile_lifetime: 0.8,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::DoubleBarrel,
                name: "Double Barrel".into(),
                damage: 10.0,
                fire_rate: 1.5,
                projectile_speed: 600.0,
                spread: 0.35,
                projectiles_per_shot: 12,
                ammo_capacity: Some(40),
                reload_time: 2.0,
                projectile_lifetime: 0.7,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::Jackhammer,
                name: "Jackhammer".into(),
                damage: 7.0,
                fire_rate: 4.0,
                projectile_speed: 650.0,
                spread: 0.25,
                projectiles_per_shot: 6,
                ammo_capacity: Some(100),
                reload_time: 2.0,
                projectile_lifetime: 0.9,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::Blowtorch,
                name: "Blowtorch".into(),
                damage: 5.0,
                fire_rate: 20.0,
                projectile_speed: 400.0,
                spread: 0.4,
                projectiles_per_shot: 3,
                ammo_capacity: Some(500),
                reload_time: 2.0,
                projectile_lifetime: 0.3,
                pierce_count: 2,
                homing: false,
                explosive_radius: 0.0,
            },
            // Special Weapons
            WeaponData {
                id: WeaponId::Flamethrower,
                name: "Flamethrower".into(),
                damage: 8.0,
                fire_rate: 25.0,
                projectile_speed: 300.0,
                spread: 0.3,
                projectiles_per_shot: 1,
                ammo_capacity: Some(400),
                reload_time: 2.0,
                projectile_lifetime: 0.5,
                pierce_count: 3,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::PlasmaRifle,
                name: "Plasma Rifle".into(),
                damage: 25.0,
                fire_rate: 8.0,
                projectile_speed: 600.0,
                spread: 0.05,
                projectiles_per_shot: 1,
                ammo_capacity: Some(150),
                reload_time: 1.5,
                projectile_lifetime: 2.0,
                pierce_count: 2,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::PulseGun,
                name: "Pulse Gun".into(),
                damage: 30.0,
                fire_rate: 6.0,
                projectile_speed: 550.0,
                spread: 0.03,
                projectiles_per_shot: 1,
                ammo_capacity: Some(100),
                reload_time: 1.5,
                projectile_lifetime: 2.5,
                pierce_count: 3,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::IonRifle,
                name: "Ion Rifle".into(),
                damage: 40.0,
                fire_rate: 3.0,
                projectile_speed: 1200.0,
                spread: 0.01,
                projectiles_per_shot: 1,
                ammo_capacity: Some(60),
                reload_time: 2.0,
                projectile_lifetime: 2.0,
                pierce_count: 5,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::GaussGun,
                name: "Gauss Gun".into(),
                damage: 80.0,
                fire_rate: 1.5,
                projectile_speed: 1500.0,
                spread: 0.0,
                projectiles_per_shot: 1,
                ammo_capacity: Some(30),
                reload_time: 2.5,
                projectile_lifetime: 3.0,
                pierce_count: 10,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::GaussShotgun,
                name: "Gauss Shotgun".into(),
                damage: 30.0,
                fire_rate: 1.0,
                projectile_speed: 1200.0,
                spread: 0.2,
                projectiles_per_shot: 5,
                ammo_capacity: Some(25),
                reload_time: 2.5,
                projectile_lifetime: 2.0,
                pierce_count: 3,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::ShrinkRay,
                name: "Shrink Ray".into(),
                damage: 5.0,
                fire_rate: 10.0,
                projectile_speed: 500.0,
                spread: 0.1,
                projectiles_per_shot: 1,
                ammo_capacity: Some(200),
                reload_time: 1.5,
                projectile_lifetime: 1.5,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::FreezeRay,
                name: "Freeze Ray".into(),
                damage: 3.0,
                fire_rate: 15.0,
                projectile_speed: 400.0,
                spread: 0.15,
                projectiles_per_shot: 1,
                ammo_capacity: Some(300),
                reload_time: 1.5,
                projectile_lifetime: 1.0,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            // Heavy Weapons
            WeaponData {
                id: WeaponId::RocketLauncher,
                name: "Rocket Launcher".into(),
                damage: 100.0,
                fire_rate: 1.0,
                projectile_speed: 400.0,
                spread: 0.02,
                projectiles_per_shot: 1,
                ammo_capacity: Some(20),
                reload_time: 2.0,
                projectile_lifetime: 4.0,
                pierce_count: 0,
                homing: false,
                explosive_radius: 80.0,
            },
            WeaponData {
                id: WeaponId::HomingMissile,
                name: "Homing Missile".into(),
                damage: 80.0,
                fire_rate: 2.0,
                projectile_speed: 350.0,
                spread: 0.1,
                projectiles_per_shot: 1,
                ammo_capacity: Some(30),
                reload_time: 2.0,
                projectile_lifetime: 5.0,
                pierce_count: 0,
                homing: true,
                explosive_radius: 60.0,
            },
            WeaponData {
                id: WeaponId::GrenadeLauncher,
                name: "Grenade Launcher".into(),
                damage: 70.0,
                fire_rate: 2.0,
                projectile_speed: 350.0,
                spread: 0.05,
                projectiles_per_shot: 1,
                ammo_capacity: Some(40),
                reload_time: 2.0,
                projectile_lifetime: 3.0,
                pierce_count: 0,
                homing: false,
                explosive_radius: 100.0,
            },
            // Exotic Weapons
            WeaponData {
                id: WeaponId::BladeCannon,
                name: "Blade Cannon".into(),
                damage: 35.0,
                fire_rate: 5.0,
                projectile_speed: 700.0,
                spread: 0.1,
                projectiles_per_shot: 1,
                ammo_capacity: Some(100),
                reload_time: 1.5,
                projectile_lifetime: 2.0,
                pierce_count: 5,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::ChainReactor,
                name: "Chain Reactor".into(),
                damage: 20.0,
                fire_rate: 4.0,
                projectile_speed: 500.0,
                spread: 0.05,
                projectiles_per_shot: 1,
                ammo_capacity: Some(80),
                reload_time: 2.0,
                projectile_lifetime: 2.5,
                pierce_count: 0,
                homing: false,
                explosive_radius: 40.0,
            },
            WeaponData {
                id: WeaponId::SplitterGun,
                name: "Splitter Gun".into(),
                damage: 15.0,
                fire_rate: 3.0,
                projectile_speed: 600.0,
                spread: 0.05,
                projectiles_per_shot: 1,
                ammo_capacity: Some(60),
                reload_time: 2.0,
                projectile_lifetime: 2.0,
                pierce_count: 0,
                homing: false,
                explosive_radius: 0.0,
            },
            WeaponData {
                id: WeaponId::InfernoCannon,
                name: "Inferno Cannon".into(),
                damage: 50.0,
                fire_rate: 2.0,
                projectile_speed: 450.0,
                spread: 0.1,
                projectiles_per_shot: 1,
                ammo_capacity: Some(50),
                reload_time: 2.5,
                projectile_lifetime: 3.0,
                pierce_count: 2,
                homing: false,
                explosive_radius: 70.0,
            },
        ];
    }
}

/// Data for a weapon type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponData {
    pub id: WeaponId,
    pub name: String,
    pub damage: f32,
    /// Shots per second
    pub fire_rate: f32,
    pub projectile_speed: f32,
    /// Spread in radians
    pub spread: f32,
    pub projectiles_per_shot: u32,
    /// None means infinite ammo
    pub ammo_capacity: Option<u32>,
    pub reload_time: f32,
    pub projectile_lifetime: f32,
    /// Number of enemies a projectile can pass through
    pub pierce_count: u32,
    pub homing: bool,
    /// 0 means no explosion
    pub explosive_radius: f32,
}

impl WeaponData {
    pub fn fire_cooldown(&self) -> f32 {
        if self.fire_rate > 0.0 {
            1.0 / self.fire_rate
        } else {
            1.0
        }
    }

    pub fn is_explosive(&self) -> bool {
        self.explosive_radius > 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_registry_has_all_weapons() {
        let registry = WeaponRegistry::new();
        assert!(!registry.weapons.is_empty());

        // Check we have at least the core weapon types
        assert!(registry.get(WeaponId::Pistol).is_some());
        assert!(registry.get(WeaponId::Shotgun).is_some());
        assert!(registry.get(WeaponId::RocketLauncher).is_some());
    }

    #[test]
    fn weapon_data_fire_cooldown_calculated_correctly() {
        let weapon = WeaponData {
            id: WeaponId::Pistol,
            name: "Test".into(),
            damage: 10.0,
            fire_rate: 5.0, // 5 shots per second
            projectile_speed: 500.0,
            spread: 0.0,
            projectiles_per_shot: 1,
            ammo_capacity: None,
            reload_time: 1.0,
            projectile_lifetime: 2.0,
            pierce_count: 0,
            homing: false,
            explosive_radius: 0.0,
        };

        assert!((weapon.fire_cooldown() - 0.2).abs() < 0.001);
    }

    #[test]
    fn pistol_has_infinite_ammo() {
        let registry = WeaponRegistry::new();
        let pistol = registry.get(WeaponId::Pistol).unwrap();
        assert!(pistol.ammo_capacity.is_none());
    }

    #[test]
    fn rocket_launcher_is_explosive() {
        let registry = WeaponRegistry::new();
        let rocket = registry.get(WeaponId::RocketLauncher).unwrap();
        assert!(rocket.is_explosive());
    }

    #[test]
    fn homing_missile_has_homing() {
        let registry = WeaponRegistry::new();
        let homing = registry.get(WeaponId::HomingMissile).unwrap();
        assert!(homing.homing);
    }
}
