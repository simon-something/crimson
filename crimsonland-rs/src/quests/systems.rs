//! Quest systems

use bevy::prelude::*;

use super::database::{QuestDatabase, QuestId};
use crate::creatures::components::{Creature, MarkedForDespawn};
use crate::creatures::systems::{CreatureDeathEvent, SpawnCreatureEvent};
use crate::states::GameState;

/// Currently active quest
#[derive(Resource, Default)]
pub struct ActiveQuest {
    pub quest_id: Option<QuestId>,
}

impl ActiveQuest {
    pub fn new(quest_id: QuestId) -> Self {
        Self {
            quest_id: Some(quest_id),
        }
    }
}

/// Tracks progress through the current quest
#[derive(Resource, Default)]
pub struct QuestProgress {
    /// Current wave index
    pub current_wave: usize,
    /// Time elapsed in current wave
    pub wave_time: f32,
    /// Total time elapsed in quest
    pub total_time: f32,
    /// Creatures spawned so far in current wave
    pub spawned_in_wave: Vec<u32>,
    /// Time until next spawn for each spawn entry
    pub spawn_timers: Vec<f32>,
    /// Whether the current wave is complete
    pub wave_complete: bool,
    /// Delay timer before starting the wave
    pub wave_delay_timer: f32,
    /// Whether we're waiting for the delay
    pub waiting_for_delay: bool,
    /// Total kills in this quest
    pub kills: u32,
}

impl QuestProgress {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn start_wave(&mut self, wave_data: &super::database::WaveData) {
        self.wave_time = 0.0;
        self.spawned_in_wave = vec![0; wave_data.spawns.len()];
        self.spawn_timers = vec![0.0; wave_data.spawns.len()];
        self.wave_complete = false;
        self.wave_delay_timer = wave_data.spawn_delay;
        self.waiting_for_delay = wave_data.spawn_delay > 0.0;
    }

    pub fn advance_wave(&mut self) {
        self.current_wave += 1;
        self.wave_complete = false;
    }
}

/// Event fired when a quest is completed
#[derive(Event)]
pub struct QuestCompletedEvent {
    pub quest_id: QuestId,
    pub time: f32,
    pub kills: u32,
}

/// Event fired when a wave is completed
#[derive(Event)]
pub struct WaveCompletedEvent {
    pub wave_index: usize,
}

/// Starts the active quest when entering Playing state
pub fn start_active_quest(
    active_quest: Res<ActiveQuest>,
    quest_db: Res<QuestDatabase>,
    mut progress: ResMut<QuestProgress>,
) {
    progress.reset();

    if let Some(quest_id) = active_quest.quest_id {
        if let Some(quest_data) = quest_db.get(quest_id) {
            if let Some(first_wave) = quest_data.waves.first() {
                progress.start_wave(first_wave);
            }
        }
    }
}

/// Cleans up quest state when leaving Playing
pub fn cleanup_quest_state(mut progress: ResMut<QuestProgress>) {
    progress.reset();
}

/// Updates quest progress timers
pub fn update_quest_progress(time: Res<Time>, mut progress: ResMut<QuestProgress>) {
    progress.total_time += time.delta_seconds();
    progress.wave_time += time.delta_seconds();

    if progress.waiting_for_delay {
        progress.wave_delay_timer -= time.delta_seconds();
        if progress.wave_delay_timer <= 0.0 {
            progress.waiting_for_delay = false;
        }
    }
}

/// Spawns creatures for the current wave
pub fn spawn_wave_creatures(
    time: Res<Time>,
    active_quest: Res<ActiveQuest>,
    quest_db: Res<QuestDatabase>,
    mut progress: ResMut<QuestProgress>,
    mut spawn_events: EventWriter<SpawnCreatureEvent>,
) {
    // Skip if waiting for delay
    if progress.waiting_for_delay {
        return;
    }

    let Some(quest_id) = active_quest.quest_id else {
        return;
    };

    let Some(quest_data) = quest_db.get(quest_id) else {
        return;
    };

    let Some(wave_data) = quest_data.waves.get(progress.current_wave) else {
        return;
    };

    // Update spawn timers and spawn creatures
    for (i, spawn_entry) in wave_data.spawns.iter().enumerate() {
        if i >= progress.spawned_in_wave.len() {
            continue;
        }

        // Check if we've spawned all of this type
        if progress.spawned_in_wave[i] >= spawn_entry.count {
            continue;
        }

        // Update timer
        progress.spawn_timers[i] -= time.delta_seconds();

        // Spawn if timer is ready
        if progress.spawn_timers[i] <= 0.0 {
            spawn_events.send(SpawnCreatureEvent {
                creature_type: spawn_entry.creature,
                position: None, // Let spawner choose position
            });

            progress.spawned_in_wave[i] += 1;
            progress.spawn_timers[i] = spawn_entry.interval;
        }
    }
}

/// Checks if the current wave is complete
pub fn check_wave_completion(
    active_quest: Res<ActiveQuest>,
    quest_db: Res<QuestDatabase>,
    mut progress: ResMut<QuestProgress>,
    creatures: Query<Entity, (With<Creature>, Without<MarkedForDespawn>)>,
    mut wave_events: EventWriter<WaveCompletedEvent>,
) {
    if progress.wave_complete {
        return;
    }

    let Some(quest_id) = active_quest.quest_id else {
        return;
    };

    let Some(quest_data) = quest_db.get(quest_id) else {
        return;
    };

    let Some(wave_data) = quest_data.waves.get(progress.current_wave) else {
        return;
    };

    // Check if all creatures have been spawned
    let all_spawned = progress
        .spawned_in_wave
        .iter()
        .zip(wave_data.spawns.iter())
        .all(|(spawned, entry)| *spawned >= entry.count);

    if !all_spawned {
        return;
    }

    // Check if all creatures are dead
    let creatures_alive = creatures.iter().count();

    if creatures_alive == 0 {
        progress.wave_complete = true;
        wave_events.send(WaveCompletedEvent {
            wave_index: progress.current_wave,
        });

        // Move to next wave if available
        if progress.current_wave + 1 < quest_data.waves.len() {
            progress.advance_wave();
            if let Some(next_wave) = quest_data.waves.get(progress.current_wave) {
                progress.start_wave(next_wave);
            }
        }
    }
}

/// Checks if the quest is complete
pub fn check_quest_completion(
    active_quest: Res<ActiveQuest>,
    quest_db: Res<QuestDatabase>,
    progress: Res<QuestProgress>,
    creatures: Query<Entity, (With<Creature>, Without<MarkedForDespawn>)>,
    mut quest_events: EventWriter<QuestCompletedEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(quest_id) = active_quest.quest_id else {
        return;
    };

    let Some(quest_data) = quest_db.get(quest_id) else {
        return;
    };

    // Check if we've completed all waves
    if progress.current_wave + 1 < quest_data.waves.len() {
        return;
    }

    // Check if the last wave is complete and all creatures are dead
    if !progress.wave_complete {
        return;
    }

    let creatures_alive = creatures.iter().count();
    if creatures_alive > 0 {
        return;
    }

    // Quest complete!
    quest_events.send(QuestCompletedEvent {
        quest_id,
        time: progress.total_time,
        kills: progress.kills,
    });

    next_state.set(GameState::Victory);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_quest_can_be_created() {
        let quest = ActiveQuest::new(QuestId::Q01LandHostile);
        assert_eq!(quest.quest_id, Some(QuestId::Q01LandHostile));
    }

    #[test]
    fn quest_progress_reset_clears_all() {
        let mut progress = QuestProgress {
            current_wave: 5,
            wave_time: 100.0,
            total_time: 500.0,
            kills: 50,
            ..default()
        };

        progress.reset();

        assert_eq!(progress.current_wave, 0);
        assert_eq!(progress.wave_time, 0.0);
        assert_eq!(progress.kills, 0);
    }

    #[test]
    fn quest_progress_advance_wave() {
        let mut progress = QuestProgress::default();
        progress.advance_wave();
        assert_eq!(progress.current_wave, 1);
        progress.advance_wave();
        assert_eq!(progress.current_wave, 2);
    }

    #[test]
    fn quest_completed_event_can_be_created() {
        let event = QuestCompletedEvent {
            quest_id: QuestId::Q01LandHostile,
            time: 120.5,
            kills: 100,
        };
        assert_eq!(event.kills, 100);
    }
}

/// Run condition: only run if a quest is active
pub fn quest_is_active(active_quest: Res<ActiveQuest>) -> bool {
    active_quest.quest_id.is_some()
}

/// Tracks kills from creature death events
pub fn track_quest_kills(
    mut progress: ResMut<QuestProgress>,
    mut death_events: EventReader<CreatureDeathEvent>,
) {
    for _event in death_events.read() {
        progress.kills += 1;
    }
}
