//! Perk selection screen

use bevy::prelude::*;

use crate::perks::components::{PerkId, PerkInventory};
use crate::perks::registry::{PerkData, PerkRegistry};
use crate::perks::systems::PerkSelectedEvent;
use crate::player::components::Player;
use crate::states::GameState;

/// Gets perks the player already has for display
pub fn get_player_perks(inventory: &PerkInventory) -> Vec<PerkId> {
    PerkId::all()
        .iter()
        .filter(|&&perk| inventory.has_perk(perk))
        .copied()
        .collect()
}

/// Marker for perk selection UI
#[derive(Component)]
pub struct PerkSelectUi;

/// Marker for individual perk buttons
#[derive(Component)]
pub struct PerkButton {
    pub perk_id: PerkId,
    pub index: usize,
}

/// Resource to track current perk selection
#[derive(Resource, Default, Clone)]
pub struct PerkSelectionState {
    pub available_perks: Vec<PerkId>,
    pub selected_index: usize,
}

/// Sets up the perk selection screen
pub fn setup_perk_select(
    mut commands: Commands,
    perk_registry: Res<PerkRegistry>,
    player_query: Query<&PerkInventory, With<Player>>,
    mut selection_state: Local<PerkSelectionState>,
) {
    // Get random perks to choose from
    let perks = perk_registry.get_random_selection(4);
    selection_state.available_perks = perks.iter().map(|p| p.id).collect();
    selection_state.selected_index = 0;

    // Get player's current perks
    let player_inventory = player_query.get_single().ok();

    commands
        .spawn((
            PerkSelectUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.1, 0.9)),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "LEVEL UP! Choose a Perk",
                TextStyle {
                    font_size: 36.0,
                    color: Color::srgb(0.5, 0.8, 1.0),
                    ..default()
                },
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            // Perk buttons
            for (i, perk_data) in perks.iter().enumerate() {
                // Get current level for this perk using PerkRegistry.get()
                let current_level = player_inventory
                    .map(|inv| inv.get_count(perk_data.id))
                    .unwrap_or(0);

                // Verify perk data using PerkRegistry.get() for consistency
                let verified_perk = perk_registry.get(perk_data.id).unwrap_or(perk_data);
                spawn_perk_button(parent, verified_perk, i, current_level);
            }

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    ..default()
                },
                ..default()
            });

            // Show owned perks count using get_player_perks
            let owned_count = player_inventory
                .map(|inv| get_player_perks(inv).len())
                .unwrap_or(0);
            parent.spawn(TextBundle::from_section(
                format!("You have {} perks", owned_count),
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.6, 0.6, 0.6),
                    ..default()
                },
            ));

            // Instructions
            parent.spawn(TextBundle::from_section(
                "Press 1-4 or click to select",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(0.5, 0.5, 0.5),
                    ..default()
                },
            ));
        });

    commands.insert_resource(selection_state.clone());
}

fn spawn_perk_button(parent: &mut ChildBuilder, perk: &PerkData, index: usize, current_level: u8) {
    // Highlight color if player already has this perk
    let bg_color = if current_level > 0 {
        Color::srgb(0.2, 0.25, 0.2) // Slightly green tint
    } else {
        Color::srgb(0.15, 0.15, 0.2)
    };

    parent
        .spawn((
            PerkButton {
                perk_id: perk.id,
                index,
            },
            ButtonBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(80.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    ..default()
                },
                background_color: BackgroundColor(bg_color),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Perk name with number and current level
            let level_text = if current_level > 0 {
                format!("{}. {} (Lv {})", index + 1, perk.name, current_level)
            } else {
                format!("{}. {}", index + 1, perk.name)
            };
            parent.spawn(TextBundle::from_section(
                level_text,
                TextStyle {
                    font_size: 24.0,
                    color: perk.rarity.color(),
                    ..default()
                },
            ));

            // Perk description
            parent.spawn(TextBundle::from_section(
                &perk.description,
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));
        });
}

/// Cleans up the perk selection screen
pub fn cleanup_perk_select(
    mut commands: Commands,
    query: Query<Entity, With<PerkSelectUi>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<PerkSelectionState>();
}

/// Handles perk selection input
pub fn handle_perk_select_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    selection_state: Option<Res<PerkSelectionState>>,
    player_query: Query<Entity, With<Player>>,
    button_query: Query<(&Interaction, &PerkButton), Changed<Interaction>>,
    mut perk_events: EventWriter<PerkSelectedEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(selection_state) = selection_state else {
        return;
    };

    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    // Number key selection
    let selected = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(3)
    } else {
        None
    };

    if let Some(index) = selected {
        if let Some(&perk_id) = selection_state.available_perks.get(index) {
            perk_events.send(PerkSelectedEvent {
                player_entity,
                perk_id,
            });
            next_state.set(GameState::Playing);
            return;
        }
    }

    // Mouse click selection - use button.index for logging
    for (interaction, button) in button_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("Perk {} selected via mouse click", button.index + 1);
            perk_events.send(PerkSelectedEvent {
                player_entity,
                perk_id: button.perk_id,
            });
            next_state.set(GameState::Playing);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perk_selection_state_default() {
        let state = PerkSelectionState::default();
        assert!(state.available_perks.is_empty());
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn perk_button_stores_data() {
        let button = PerkButton {
            perk_id: PerkId::Regeneration,
            index: 2,
        };
        assert_eq!(button.index, 2);
    }

    #[test]
    fn get_player_perks_returns_owned_perks() {
        let mut inventory = PerkInventory::new();
        inventory.add_perk(PerkId::Regeneration);
        inventory.add_perk(PerkId::SpeedBoost);

        let owned = get_player_perks(&inventory);
        assert!(owned.contains(&PerkId::Regeneration));
        assert!(owned.contains(&PerkId::SpeedBoost));
        assert!(!owned.contains(&PerkId::CriticalHit));
    }

    #[test]
    fn registry_get_returns_perk_data() {
        let registry = PerkRegistry::default();
        let perk = registry.get(PerkId::Regeneration);
        assert!(perk.is_some());
        assert_eq!(perk.unwrap().id, PerkId::Regeneration);
    }
}
