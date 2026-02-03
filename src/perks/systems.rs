//! Perk systems

use bevy::prelude::*;

use super::components::{PerkBonuses, PerkId, PerkInventory};
use super::registry::PerkRegistry;
use crate::player::components::{Health, MoveSpeed, Player};
use crate::player::resources::PlayerConfig;

/// Event when a perk is selected
#[derive(Event)]
pub struct PerkSelectedEvent {
    pub player_entity: Entity,
    pub perk_id: PerkId,
}

/// Sets up perk selection state
pub fn setup_perk_selection(_registry: Res<PerkRegistry>) {
    // Perk selection UI is handled by the UI module
    // This system could pre-calculate available perks based on player state
}

/// Applies perk effects each frame
pub fn apply_perk_effects(
    time: Res<Time>,
    config: Res<PlayerConfig>,
    mut query: Query<
        (
            &PerkInventory,
            &mut PerkBonuses,
            &mut Health,
            &mut MoveSpeed,
        ),
        With<Player>,
    >,
) {
    for (inventory, mut bonuses, mut health, mut speed) in query.iter_mut() {
        // Recalculate bonuses
        *bonuses = PerkBonuses::calculate(inventory);

        // Apply regeneration
        if bonuses.regen_per_second > 0.0 {
            let heal_amount = bonuses.regen_per_second * time.delta_seconds();
            health.heal(heal_amount);
        }

        // Apply max health multiplier (ThickSkinned reduces to 2/3)
        let adjusted_max = config.base_health * bonuses.max_health_multiplier;
        if (health.max - adjusted_max).abs() > 0.01 {
            let health_percent = health.percentage();
            health.max = adjusted_max;
            health.current = adjusted_max * health_percent;
        }

        // Apply speed multiplier
        speed.0 = config.base_move_speed * bonuses.speed_multiplier;
    }
}

/// Handles perk selection events (for external listeners)
/// Note: The actual perk application is done in handle_perk_select_input to avoid timing issues
pub fn handle_perk_selection(
    mut events: EventReader<PerkSelectedEvent>,
    query: Query<&PerkInventory, With<Player>>,
) {
    for event in events.read() {
        // Just log - perk is already applied by handle_perk_select_input
        if let Ok(inventory) = query.get(event.player_entity) {
            info!(
                "Perk {:?} selected, player now has {} perks",
                event.perk_id,
                inventory.total_perks()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perk_selected_event_can_be_created() {
        let event = PerkSelectedEvent {
            player_entity: Entity::PLACEHOLDER,
            perk_id: PerkId::Regeneration,
        };
        assert_eq!(event.perk_id, PerkId::Regeneration);
    }

    #[test]
    fn perk_bonuses_apply_regen() {
        let mut inventory = PerkInventory::new();
        inventory.add_perk(PerkId::Regeneration);

        let bonuses = PerkBonuses::calculate(&inventory);
        assert!(bonuses.regen_per_second > 0.0);
    }

    #[test]
    fn perk_bonuses_apply_speed() {
        let mut inventory = PerkInventory::new();
        inventory.add_perk(PerkId::LongDistanceRunner);

        let bonuses = PerkBonuses::calculate(&inventory);
        assert!(bonuses.speed_multiplier > 1.0);
    }
}
