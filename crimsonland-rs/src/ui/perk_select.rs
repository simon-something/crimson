//! Perk selection screen

use bevy::prelude::*;

use crate::perks::components::PerkId;
use crate::perks::registry::{PerkData, PerkRegistry};
use crate::perks::systems::PerkSelectedEvent;
use crate::player::components::Player;
use crate::states::GameState;

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
    mut selection_state: Local<PerkSelectionState>,
) {
    // Get random perks to choose from
    let perks = perk_registry.get_random_selection(4);
    selection_state.available_perks = perks.iter().map(|p| p.id).collect();
    selection_state.selected_index = 0;

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
                spawn_perk_button(parent, perk_data, i);
            }

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    ..default()
                },
                ..default()
            });

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

fn spawn_perk_button(parent: &mut ChildBuilder, perk: &PerkData, index: usize) {
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
                background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Perk name with number
            parent.spawn(TextBundle::from_section(
                format!("{}. {}", index + 1, perk.name),
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

    // Mouse click selection
    for (interaction, button) in button_query.iter() {
        if *interaction == Interaction::Pressed {
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
}
