//! Carried items system
//!
//! Items that the player can carry and activate with the Space key.
//! Unlike bonuses which activate on pickup, these are stored and used manually.

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::*;

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for the carried item system
pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemUsedEvent>()
            .add_event::<ItemPickedUpEvent>()
            .add_systems(
                Update,
                (
                    handle_item_use,
                    apply_item_effects,
                    spawn_item_pickups,
                    collect_items,
                    update_item_lifetime,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
