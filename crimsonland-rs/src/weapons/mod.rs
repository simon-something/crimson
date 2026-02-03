//! Weapons module
//!
//! Handles weapons, projectiles, and firing mechanics.

pub mod components;
pub mod registry;
pub mod systems;

pub use components::*;
pub use registry::*;
pub use systems::*;

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for weapon-related functionality
pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponRegistry>()
            .add_event::<FireWeaponEvent>()
            .add_event::<ProjectileHitEvent>()
            .add_systems(OnExit(GameState::Playing), despawn_all_projectiles)
            .add_systems(
                Update,
                (
                    fire_weapon_system,
                    projectile_movement,
                    projectile_collision,
                    projectile_lifetime,
                    cleanup_projectiles,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
