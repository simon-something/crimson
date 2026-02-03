//! Weapon components

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Weapon types available in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum WeaponId {
    // Pistols
    #[default]
    Pistol,
    PocketRocket,
    Magnum,

    // Submachine Guns
    Uzi,
    Smg,
    DualSmg,

    // Rifles
    AssaultRifle,
    MachineGun,
    Minigun,

    // Shotguns
    Shotgun,
    DoubleBarrel,
    Jackhammer,
    Blowtorch,

    // Special
    Flamethrower,
    PlasmaRifle,
    PulseGun,
    IonRifle,
    GaussGun,
    GaussShotgun,
    ShrinkRay,
    FreezeRay,

    // Heavy
    RocketLauncher,
    HomingMissile,
    GrenadeLauncher,

    // Exotic
    BladeCannon,
    ChainReactor,
    SplitterGun,
    InfernoCannon,
}


/// Component for the player's currently equipped weapon
#[derive(Component, Debug, Clone)]
pub struct EquippedWeapon {
    pub weapon_id: WeaponId,
    pub ammo: Option<u32>,
    pub fire_cooldown: f32,
    /// Timer for reload (0.0 means not reloading)
    pub reload_timer: f32,
    /// Max ammo capacity for current weapon
    pub max_ammo: Option<u32>,
}

impl Default for EquippedWeapon {
    fn default() -> Self {
        Self {
            weapon_id: WeaponId::Pistol,
            ammo: None, // Infinite ammo for pistol
            fire_cooldown: 0.0,
            reload_timer: 0.0,
            max_ammo: None,
        }
    }
}

impl EquippedWeapon {
    pub fn new(weapon_id: WeaponId, ammo: Option<u32>) -> Self {
        Self {
            weapon_id,
            ammo,
            fire_cooldown: 0.0,
            reload_timer: 0.0,
            max_ammo: ammo,
        }
    }

    pub fn can_fire(&self) -> bool {
        self.fire_cooldown <= 0.0 && self.reload_timer <= 0.0 && self.ammo.map(|a| a > 0).unwrap_or(true)
    }

    pub fn consume_ammo(&mut self) {
        if let Some(ref mut ammo) = self.ammo {
            *ammo = ammo.saturating_sub(1);
        }
    }

    pub fn has_ammo(&self) -> bool {
        self.ammo.map(|a| a > 0).unwrap_or(true)
    }

    pub fn is_reloading(&self) -> bool {
        self.reload_timer > 0.0
    }

    pub fn start_reload(&mut self, reload_time: f32) {
        if self.ammo.is_some() && !self.is_reloading() {
            self.reload_timer = reload_time;
        }
    }

    pub fn finish_reload(&mut self) {
        if let Some(max) = self.max_ammo {
            self.ammo = Some(max);
        }
        self.reload_timer = 0.0;
    }
}

/// Marker component for projectile entities
#[derive(Component, Debug, Clone)]
pub struct Projectile {
    pub weapon_id: WeaponId,
    pub damage: f32,
    pub owner: Entity,
    pub pierce_count: u32,
}

/// Velocity component for moving projectiles
#[derive(Component, Debug, Clone, Default)]
pub struct Velocity(pub Vec2);

/// Lifetime component for projectiles
#[derive(Component, Debug, Clone)]
pub struct Lifetime {
    pub remaining: f32,
}

impl Lifetime {
    pub fn new(seconds: f32) -> Self {
        Self { remaining: seconds }
    }

    pub fn tick(&mut self, delta: f32) {
        self.remaining -= delta;
    }

    pub fn is_expired(&self) -> bool {
        self.remaining <= 0.0
    }
}

/// Component for homing projectiles
#[derive(Component, Debug, Clone)]
pub struct Homing {
    pub turn_rate: f32,
    pub target: Option<Entity>,
}

/// Component for explosive projectiles
#[derive(Component, Debug, Clone)]
pub struct Explosive {
    pub radius: f32,
    pub damage: f32,
}

/// Component for chain lightning projectiles (ChainReactor)
/// Lightning jumps to nearby enemies on hit
#[derive(Component, Debug, Clone)]
pub struct ChainLightning {
    /// Number of jumps remaining
    pub jumps_remaining: u32,
    /// Maximum distance for chain to jump
    pub jump_range: f32,
    /// Damage reduction per jump (0.8 = 80% damage each jump)
    pub damage_falloff: f32,
    /// Entities already hit (to avoid hitting same target twice)
    pub already_hit: Vec<Entity>,
}

impl ChainLightning {
    pub fn new(jumps: u32, range: f32, falloff: f32) -> Self {
        Self {
            jumps_remaining: jumps,
            jump_range: range,
            damage_falloff: falloff,
            already_hit: Vec::new(),
        }
    }
}

/// Component for splitter projectiles (SplitterGun)
/// Projectile splits into multiple smaller projectiles on hit
#[derive(Component, Debug, Clone)]
pub struct Splitter {
    /// Number of splits remaining
    pub splits_remaining: u32,
    /// Number of projectiles to spawn on split
    pub split_count: u32,
    /// Damage multiplier for split projectiles
    pub damage_multiplier: f32,
}

impl Splitter {
    pub fn new(splits: u32, count: u32, damage_mult: f32) -> Self {
        Self {
            splits_remaining: splits,
            split_count: count,
            damage_multiplier: damage_mult,
        }
    }
}

/// Component for freezing projectiles (FreezeRay)
/// Slows down enemies on hit
#[derive(Component, Debug, Clone)]
pub struct Freezing {
    /// Slow multiplier (0.5 = 50% speed)
    pub slow_amount: f32,
    /// Duration of slow effect
    pub duration: f32,
}

/// Marker for projectiles to be cleaned up
#[derive(Component)]
pub struct ProjectileDespawn;

/// Bundle for spawning basic projectiles
#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub velocity: Velocity,
    pub lifetime: Lifetime,
    pub sprite: SpriteBundle,
}

impl ProjectileBundle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        weapon_id: WeaponId,
        damage: f32,
        owner: Entity,
        position: Vec3,
        direction: Vec2,
        speed: f32,
        lifetime: f32,
        color: Color,
        size: f32,
    ) -> Self {
        Self {
            projectile: Projectile {
                weapon_id,
                damage,
                owner,
                pierce_count: 0,
            },
            velocity: Velocity(direction.normalize_or_zero() * speed),
            lifetime: Lifetime::new(lifetime),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(size, size * 0.5)),
                    ..default()
                },
                transform: Transform::from_translation(position)
                    .with_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x))),
                ..default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equipped_weapon_default_is_pistol() {
        let weapon = EquippedWeapon::default();
        assert_eq!(weapon.weapon_id, WeaponId::Pistol);
        assert!(weapon.ammo.is_none()); // Infinite ammo
    }

    #[test]
    fn equipped_weapon_can_fire_when_ready() {
        let weapon = EquippedWeapon::default();
        assert!(weapon.can_fire());
    }

    #[test]
    fn equipped_weapon_cannot_fire_on_cooldown() {
        let weapon = EquippedWeapon {
            fire_cooldown: 0.5,
            ..default()
        };
        assert!(!weapon.can_fire());
    }

    #[test]
    fn equipped_weapon_cannot_fire_without_ammo() {
        let weapon = EquippedWeapon {
            ammo: Some(0),
            ..default()
        };
        assert!(!weapon.can_fire());
    }

    #[test]
    fn equipped_weapon_consume_ammo_decrements() {
        let mut weapon = EquippedWeapon {
            ammo: Some(10),
            ..default()
        };
        weapon.consume_ammo();
        assert_eq!(weapon.ammo, Some(9));
    }

    #[test]
    fn equipped_weapon_consume_ammo_clamps_to_zero() {
        let mut weapon = EquippedWeapon {
            ammo: Some(0),
            ..default()
        };
        weapon.consume_ammo();
        assert_eq!(weapon.ammo, Some(0));
    }

    #[test]
    fn lifetime_expires_correctly() {
        let mut lifetime = Lifetime::new(1.0);
        assert!(!lifetime.is_expired());
        lifetime.tick(0.5);
        assert!(!lifetime.is_expired());
        lifetime.tick(0.6);
        assert!(lifetime.is_expired());
    }
}
