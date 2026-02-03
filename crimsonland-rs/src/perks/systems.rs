//! Perk systems

use bevy::prelude::*;

use super::components::{PerkBonuses, PerkId, PerkInventory};
use super::registry::PerkRegistry;
use crate::player::components::{Health, MoveSpeed, Player};
use crate::player::resources::PlayerConfig;
use crate::states::GameState;

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

        // Apply max health bonus
        let base_max = config.base_health + bonuses.max_health_bonus;
        if health.max != base_max {
            let health_percent = health.percentage();
            health.max = base_max;
            health.current = base_max * health_percent;
        }

        // Apply speed multiplier
        speed.0 = config.base_move_speed * bonuses.speed_multiplier;
    }
}

/// Handles perk selection events
pub fn handle_perk_selection(
    mut events: EventReader<PerkSelectedEvent>,
    mut query: Query<(&mut PerkInventory, &mut PerkBonuses), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        if let Ok((mut inventory, mut bonuses)) = query.get_mut(event.player_entity) {
            inventory.add_perk(event.perk_id);
            *bonuses = PerkBonuses::calculate(&inventory);
        }

        // Return to gameplay after selecting a perk
        next_state.set(GameState::Playing);
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
        inventory.add_perk(PerkId::SpeedBoost);

        let bonuses = PerkBonuses::calculate(&inventory);
        assert!(bonuses.speed_multiplier > 1.0);
    }
}
