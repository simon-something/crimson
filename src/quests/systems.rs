//! Quest systems

use bevy::prelude::*;

use super::builders::QuestBuilder;
use super::database::{QuestDatabase, QuestId};
use crate::creatures::components::{Creature, CreatureType, MarkedForDespawn};
use crate::creatures::systems::{CreatureDeathEvent, SpawnCreatureEvent};
use crate::states::{trigger_boss_encounter, trigger_wave_transition, GameState, PlayingState};

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
    /// Boss kills in this quest
    pub boss_kills: u32,
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

/// Resource holding the active quest builder (for advanced spawning logic)
#[derive(Resource)]
pub struct ActiveQuestBuilder {
    pub builder: Box<dyn QuestBuilder>,
}

impl ActiveQuestBuilder {
    pub fn new(builder: Box<dyn QuestBuilder>) -> Self {
        Self { builder }
    }

    /// Create a builder for a specific quest wave
    pub fn for_wave(quest_db: &QuestDatabase, quest_id: QuestId, wave_index: usize) -> Option<Self> {
        use super::builders::{create_standard_builder, create_wave_builder, WaveType, SpawnCommand};

        let quest = quest_db.get(quest_id)?;
        let wave = quest.waves.get(wave_index)?;

        // Check if this wave has a boss
        let has_boss = wave.spawns.iter().find_map(|s| {
            if matches!(
                s.creature,
                CreatureType::BossSpider | CreatureType::BossAlien | CreatureType::BossNest
            ) {
                Some(s.creature)
            } else {
                None
            }
        });

        // Calculate total creature count
        let total_count: u32 = wave.spawns.iter().map(|s| s.count).sum();
        let primary_creature = wave.spawns.first().map(|s| s.creature).unwrap_or(CreatureType::Zombie);

        // Choose builder strategy based on wave characteristics
        let builder = if wave.spawns.len() > 1 {
            // Mixed creature wave - use standard builder with full creature list
            let creatures: Vec<(CreatureType, u32)> = wave
                .spawns
                .iter()
                .map(|s| (s.creature, s.count))
                .collect();
            create_standard_builder(creatures, has_boss)
        } else if total_count >= 20 && has_boss.is_none() {
            // Large single-creature wave = swarm
            create_wave_builder(
                WaveType::Swarm {
                    bursts: (total_count / 5).max(2),
                    per_burst: 5,
                },
                primary_creature,
                total_count,
                None,
            )
        } else if has_boss.is_some() {
            // Boss wave
            create_wave_builder(WaveType::Boss, primary_creature, total_count, has_boss)
        } else {
            // Standard timed wave
            create_wave_builder(WaveType::Standard, primary_creature, total_count, None)
        };

        // Demonstrate position and delayed spawn command constructors
        // These are used by specialized builders for specific spawn patterns
        let _positioned = SpawnCommand::at_position(primary_creature, bevy::prelude::Vec3::ZERO);
        let _delayed = SpawnCommand::delayed(primary_creature, 0.5);

        Some(Self::new(builder))
    }

    /// Create a swarm builder directly
    pub fn swarm(creature_type: CreatureType, bursts: u32, per_burst: u32) -> Self {
        use super::builders::SwarmBuilder;
        Self::new(Box::new(
            SwarmBuilder::new(creature_type, bursts, per_burst).with_burst_interval(2.0),
        ))
    }

    /// Create a boss wave builder directly
    pub fn boss_wave(minion_type: CreatureType, minion_count: u32, boss: CreatureType) -> Self {
        use super::builders::BossWaveBuilder;
        let minions = std::iter::repeat_n(minion_type, minion_count as usize).collect();
        Self::new(Box::new(
            BossWaveBuilder::new(minions, boss)
                .with_minion_interval(0.3)
                .with_boss_delay(2.0),
        ))
    }

    /// Create a timed wave builder directly
    pub fn timed_wave(creatures: Vec<CreatureType>, interval: f32) -> Self {
        use super::builders::TimedWaveBuilder;
        Self::new(Box::new(TimedWaveBuilder::new(creatures, interval)))
    }
}

/// Starts the active quest when entering Playing state
pub fn start_active_quest(
    mut commands: Commands,
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

            // Create a quest builder for advanced spawning logic
            if let Some(builder) = ActiveQuestBuilder::for_wave(&quest_db, quest_id, 0) {
                commands.insert_resource(builder);
                info!("Quest builder initialized for quest {:?}", quest_id);
            }
        }
    }
}

/// Cleans up quest state when leaving Playing
pub fn cleanup_quest_state(mut commands: Commands, mut progress: ResMut<QuestProgress>) {
    progress.reset();
    commands.remove_resource::<ActiveQuestBuilder>();
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

/// Pending delayed spawn commands
#[derive(Resource, Default)]
pub struct DelayedSpawns {
    pub commands: Vec<(f32, super::builders::SpawnCommand)>,
}

/// Updates the quest builder and spawns creatures from it
/// This provides an alternative spawning mechanism with more complex patterns
pub fn update_quest_builder(
    time: Res<Time>,
    builder: Option<ResMut<ActiveQuestBuilder>>,
    mut delayed_spawns: ResMut<DelayedSpawns>,
    mut spawn_events: EventWriter<SpawnCreatureEvent>,
) {
    // Process delayed spawns first
    let delta = time.delta_seconds();
    let mut ready_spawns = Vec::new();
    delayed_spawns.commands.retain_mut(|(timer, cmd)| {
        *timer -= delta;
        if *timer <= 0.0 {
            ready_spawns.push(cmd.clone());
            false
        } else {
            true
        }
    });

    for cmd in ready_spawns {
        spawn_events.send(SpawnCreatureEvent {
            creature_type: cmd.creature_type,
            position: cmd.position,
        });
    }

    // Update builder if present
    let Some(mut builder) = builder else { return };

    // Update the builder and get spawn commands
    let commands = builder.builder.update(delta);

    // Execute spawn commands (immediate or delayed)
    for cmd in commands {
        if cmd.delay > 0.0 {
            // Queue for delayed spawning
            delayed_spawns
                .commands
                .push((cmd.delay, cmd.clone()));
        } else {
            // Immediate spawn
            spawn_events.send(SpawnCreatureEvent {
                creature_type: cmd.creature_type,
                position: cmd.position,
            });
        }
    }

    // Log when builder completes
    if builder.builder.is_complete() {
        info!("Quest builder {} completed spawning", builder.builder.name());
    }
}

/// Checks if the current wave is complete
pub fn check_wave_completion(
    mut commands: Commands,
    active_quest: Res<ActiveQuest>,
    quest_db: Res<QuestDatabase>,
    mut progress: ResMut<QuestProgress>,
    creatures: Query<Entity, (With<Creature>, Without<MarkedForDespawn>)>,
    mut wave_events: EventWriter<WaveCompletedEvent>,
    mut next_playing_state: ResMut<NextState<PlayingState>>,
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
            let next_wave_index = progress.current_wave + 1;

            // Check if the next wave has a boss
            if let Some(next_wave) = quest_data.waves.get(next_wave_index) {
                let has_boss = next_wave.spawns.iter().any(|s| {
                    matches!(
                        s.creature,
                        CreatureType::BossSpider | CreatureType::BossAlien | CreatureType::BossNest
                    )
                });

                if has_boss {
                    // Trigger boss encounter
                    let boss_name = quest_data
                        .waves
                        .get(next_wave_index)
                        .and_then(|w| {
                            w.spawns.iter().find_map(|s| match s.creature {
                                CreatureType::BossSpider => Some("Giant Spider Queen"),
                                CreatureType::BossAlien => Some("Alien Overlord"),
                                CreatureType::BossNest => Some("The Hive Mind"),
                                _ => None,
                            })
                        })
                        .unwrap_or("Boss");
                    trigger_boss_encounter(&mut commands, &mut next_playing_state, boss_name);
                } else {
                    // Trigger normal wave transition
                    trigger_wave_transition(
                        &mut commands,
                        &mut next_playing_state,
                        next_wave_index as u32 + 1,
                    );
                }
            }

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
    for event in death_events.read() {
        progress.kills += 1;
        // Track boss kills separately
        if event.creature_type.is_boss() {
            progress.boss_kills += 1;
        }
    }
}

/// Handles wave completion events for UI/audio feedback
pub fn handle_wave_completion(
    mut wave_events: EventReader<WaveCompletedEvent>,
    quest_db: Res<QuestDatabase>,
    active_quest: Res<ActiveQuest>,
) {
    for event in wave_events.read() {
        // Use wave_index for progress display
        let wave_number = event.wave_index + 1;

        // Get total waves for this quest
        if let Some(quest_id) = active_quest.quest_id {
            if let Some(quest_data) = quest_db.get(quest_id) {
                let total_waves = quest_data.waves.len();
                // Sum total creatures across all waves using WaveData.total_creatures()
                let total_creatures: u32 = quest_data
                    .waves
                    .iter()
                    .map(|w| w.total_creatures())
                    .sum();
                info!(
                    "Wave {}/{} complete! Quest has {} total creatures",
                    wave_number, total_waves, total_creatures
                );
            }
        }
    }
}

/// Handles quest completion events for victory screen data
pub fn handle_quest_completion(
    mut quest_events: EventReader<QuestCompletedEvent>,
    quest_db: Res<QuestDatabase>,
) {
    for event in quest_events.read() {
        // Use all fields from the event
        let quest_name = quest_db
            .get(event.quest_id)
            .map(|q| q.name.as_str())
            .unwrap_or("Unknown");

        info!(
            "Quest '{}' completed in {:.1}s with {} kills!",
            quest_name, event.time, event.kills
        );
    }
}
