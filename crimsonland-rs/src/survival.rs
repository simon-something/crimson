//! Survival Mode
//!
//! Endless gameplay with increasing difficulty, random weapon drops,
//! and perk selection on level up.

use bevy::prelude::*;
use rand::Rng;

use crate::bonuses::{BonusType, SpawnBonusEvent};
use crate::creatures::{CreatureDeathEvent, CreatureRegistry, CreatureType, SpawnCreatureEvent};
use crate::player::components::{Experience, Player};
use crate::quests::ActiveQuestBuilder;
use crate::states::GameState;

/// Plugin for survival mode functionality
pub struct SurvivalPlugin;

impl Plugin for SurvivalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_survival_mode)
            .add_systems(OnExit(GameState::Playing), cleanup_survival_mode)
            .add_systems(
                Update,
                (
                    update_survival_mode,
                    spawn_survival_creatures,
                    trigger_survival_swarms,
                    spawn_survival_bonuses,
                    track_survival_kills,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Resource tracking survival mode state
#[derive(Resource, Debug)]
pub struct SurvivalState {
    /// Total game time in seconds
    pub game_time: f32,
    /// Time since last creature spawn
    pub spawn_timer: f32,
    /// Time since last weapon drop
    pub weapon_drop_timer: f32,
    /// Time since last swarm event
    pub swarm_timer: f32,
    /// Base spawn interval (decreases over time)
    pub base_spawn_interval: f32,
    /// Current difficulty multiplier
    pub difficulty: f32,
    /// Total experience earned (for difficulty scaling)
    pub total_exp: u32,
    /// Number of creatures killed
    pub kills: u32,
}

impl Default for SurvivalState {
    fn default() -> Self {
        Self {
            game_time: 0.0,
            spawn_timer: 0.0,
            weapon_drop_timer: 0.0,
            swarm_timer: 0.0,
            base_spawn_interval: 2.0, // Start with 2 seconds between spawns
            difficulty: 1.0,
            total_exp: 0,
            kills: 0,
        }
    }
}

/// Active swarm builder for survival mode
#[derive(Resource)]
pub struct SurvivalSwarm {
    pub builder: ActiveQuestBuilder,
}

impl SurvivalState {
    /// Calculate current spawn interval based on game time and difficulty
    pub fn spawn_interval(&self) -> f32 {
        // Spawn rate increases over time (interval decreases)
        // Starts at 2s, decreases to minimum of 0.3s
        let time_factor = 1.0 - (self.game_time / 300.0).min(0.85); // 5 minutes to max speed
        (self.base_spawn_interval * time_factor).max(0.3)
    }

    /// Calculate difficulty based on total experience
    /// Formula from original: 1 + (total_xp / 1000) * 0.5
    pub fn calculate_difficulty(&self) -> f32 {
        1.0 + (self.total_exp as f32 / 1000.0) * 0.5
    }

    /// Get available creature types for current difficulty
    pub fn available_creatures(&self) -> Vec<CreatureType> {
        let mut creatures = vec![CreatureType::Zombie];

        if self.game_time > 10.0 {
            creatures.push(CreatureType::Spider);
        }
        if self.game_time > 20.0 {
            creatures.push(CreatureType::Beetle);
        }
        if self.game_time > 30.0 {
            creatures.push(CreatureType::Runner);
        }
        if self.game_time > 45.0 {
            creatures.push(CreatureType::Dog);
        }
        if self.game_time > 60.0 {
            creatures.push(CreatureType::Lizard);
        }
        if self.game_time > 75.0 {
            creatures.push(CreatureType::Ghost);
        }
        if self.game_time > 90.0 {
            creatures.push(CreatureType::AlienSpider);
        }
        if self.game_time > 105.0 {
            creatures.push(CreatureType::Exploder);
        }
        if self.game_time > 120.0 {
            creatures.push(CreatureType::AlienShooter);
        }
        if self.game_time > 135.0 {
            creatures.push(CreatureType::Necromancer);
        }
        if self.game_time > 150.0 {
            creatures.push(CreatureType::GiantSpider);
        }
        if self.game_time > 165.0 {
            creatures.push(CreatureType::Splitter);
        }
        if self.game_time > 180.0 {
            creatures.push(CreatureType::Giant);
        }
        // Boss creatures spawn at later stages
        if self.game_time > 240.0 {
            creatures.push(CreatureType::BossSpider);
        }
        if self.game_time > 300.0 {
            creatures.push(CreatureType::BossAlien);
        }

        creatures
    }

    /// Pick a random creature type weighted by difficulty
    pub fn pick_creature(&self) -> CreatureType {
        let available = self.available_creatures();
        let mut rng = rand::thread_rng();

        // Higher difficulty = more chance of later creatures
        let weights: Vec<f32> = available
            .iter()
            .enumerate()
            .map(|(i, _)| {
                // Base weight decreases for later creatures
                let base = (available.len() - i) as f32;
                // Difficulty increases weight for harder creatures
                base * (1.0 + self.difficulty * 0.1 * i as f32)
            })
            .collect();

        let total_weight: f32 = weights.iter().sum();
        let roll = rng.gen::<f32>() * total_weight;

        let mut cumulative = 0.0;
        for (i, weight) in weights.iter().enumerate() {
            cumulative += weight;
            if roll < cumulative {
                return available[i];
            }
        }

        available[0]
    }
}

/// Sets up survival mode when entering Playing state
fn setup_survival_mode(mut commands: Commands) {
    commands.insert_resource(SurvivalState::default());
}

/// Cleans up survival mode when leaving Playing state
fn cleanup_survival_mode(mut commands: Commands) {
    commands.remove_resource::<SurvivalState>();
}

/// Updates survival mode timers and difficulty
fn update_survival_mode(
    time: Res<Time>,
    mut survival: ResMut<SurvivalState>,
    player_query: Query<&Experience, With<Player>>,
) {
    survival.game_time += time.delta_seconds();
    survival.spawn_timer += time.delta_seconds();
    survival.weapon_drop_timer += time.delta_seconds();
    survival.swarm_timer += time.delta_seconds();

    // Update total exp from player
    if let Ok(exp) = player_query.get_single() {
        // Simple approximation - actual total XP would need tracking
        survival.total_exp = exp.current + (exp.level - 1) * 100;
    }

    // Recalculate difficulty
    survival.difficulty = survival.calculate_difficulty();
}

/// Spawns creatures based on survival mode timers
/// Uses CreatureRegistry for wave-based spawning after initial waves
fn spawn_survival_creatures(
    mut survival: ResMut<SurvivalState>,
    creature_registry: Res<CreatureRegistry>,
    mut spawn_events: EventWriter<SpawnCreatureEvent>,
) {
    let interval = survival.spawn_interval();

    if survival.spawn_timer >= interval {
        survival.spawn_timer = 0.0;

        // Spawn 1-3 creatures based on difficulty
        let spawn_count = 1 + (survival.difficulty * 0.5) as u32;
        let spawn_count = spawn_count.min(3);

        // Calculate effective wave from game time (every 15 seconds is a "wave")
        let effective_wave = (survival.game_time / 15.0) as u32 + 1;

        for _ in 0..spawn_count {
            // Use registry for wave-appropriate creatures, fall back to time-based
            let creature_type = if let Some(ct) = creature_registry.pick_random_for_wave(effective_wave) {
                ct
            } else {
                survival.pick_creature()
            };
            spawn_events.send(SpawnCreatureEvent {
                creature_type,
                position: None, // Let spawner pick position
            });
        }
    }
}

/// Triggers periodic swarm events using the quest builder system
fn trigger_survival_swarms(
    mut commands: Commands,
    time: Res<Time>,
    mut survival: ResMut<SurvivalState>,
    mut swarm: Option<ResMut<SurvivalSwarm>>,
    mut spawn_events: EventWriter<SpawnCreatureEvent>,
) {
    const SWARM_INTERVAL: f32 = 60.0; // Swarm every minute

    // Check if we should trigger a new swarm
    if swarm.is_none() && survival.swarm_timer >= SWARM_INTERVAL && survival.game_time > 30.0 {
        survival.swarm_timer = 0.0;

        // Choose swarm type based on game time and difficulty
        let creature = survival.pick_creature();
        let mut rng = rand::thread_rng();

        let builder = if survival.game_time > 180.0 && rng.gen_bool(0.3) {
            // Boss wave after 3 minutes (30% chance)
            let boss = match rng.gen_range(0..3) {
                0 => CreatureType::BossSpider,
                1 => CreatureType::BossAlien,
                _ => CreatureType::BossNest,
            };
            let minion_count = (5 + survival.difficulty as u32).min(12);
            info!("Survival BOSS wave triggered: {:?} with {} minions", boss, minion_count);
            ActiveQuestBuilder::boss_wave(creature, minion_count, boss)
        } else if survival.game_time > 90.0 && rng.gen_bool(0.5) {
            // Timed wave after 1.5 minutes (50% chance)
            let wave_size = (8 + survival.difficulty as u32 * 2).min(20);
            let creatures: Vec<_> = std::iter::repeat_n(creature, wave_size as usize).collect();
            info!("Survival timed wave triggered: {} {:?}", wave_size, creature);
            ActiveQuestBuilder::timed_wave(creatures, 0.3)
        } else {
            // Regular swarm
            let bursts = (2 + survival.difficulty as u32).min(5);
            let per_burst = (3 + survival.difficulty as u32).min(8);
            info!(
                "Survival swarm triggered: {:?} x{} bursts of {}",
                creature, bursts, per_burst
            );
            ActiveQuestBuilder::swarm(creature, bursts, per_burst)
        };

        commands.insert_resource(SurvivalSwarm { builder });
    }

    // Update active swarm
    if let Some(ref mut swarm) = swarm {
        let cmds = swarm.builder.builder.update(time.delta_seconds());

        for cmd in cmds {
            // Use position-based spawning for swarms (spawn around edges)
            let pos = if cmd.position.is_some() {
                cmd.position
            } else {
                // Random edge position
                let mut rng = rand::thread_rng();
                let edge = rng.gen_range(0..4);
                let pos = match edge {
                    0 => Vec3::new(rng.gen_range(-600.0..600.0), 400.0, 0.0),  // Top
                    1 => Vec3::new(rng.gen_range(-600.0..600.0), -400.0, 0.0), // Bottom
                    2 => Vec3::new(-600.0, rng.gen_range(-400.0..400.0), 0.0), // Left
                    _ => Vec3::new(600.0, rng.gen_range(-400.0..400.0), 0.0),  // Right
                };
                Some(pos)
            };

            spawn_events.send(SpawnCreatureEvent {
                creature_type: cmd.creature_type,
                position: pos,
            });
        }

        // Remove swarm when complete
        if swarm.builder.builder.is_complete() {
            info!("Survival swarm completed");
            commands.remove_resource::<SurvivalSwarm>();
        }
    }
}

/// Tracks kills in survival mode
fn track_survival_kills(
    mut survival: ResMut<SurvivalState>,
    mut death_events: EventReader<CreatureDeathEvent>,
) {
    for _event in death_events.read() {
        survival.kills += 1;
    }
}

/// Spawns weapon pickups periodically
fn spawn_survival_bonuses(
    mut survival: ResMut<SurvivalState>,
    mut spawn_events: EventWriter<SpawnBonusEvent>,
    player_query: Query<&Transform, With<Player>>,
) {
    const WEAPON_DROP_INTERVAL: f32 = 30.0;

    if survival.weapon_drop_timer >= WEAPON_DROP_INTERVAL {
        survival.weapon_drop_timer = 0.0;

        // Spawn weapon near player
        if let Ok(player_transform) = player_query.get_single() {
            let mut rng = rand::thread_rng();
            let offset = Vec2::new(
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-100.0..100.0),
            );
            let position = player_transform.translation + Vec3::new(offset.x, offset.y, 0.0);

            spawn_events.send(SpawnBonusEvent {
                bonus_type: BonusType::WeaponPickup,
                position,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn survival_state_defaults() {
        let state = SurvivalState::default();
        assert_eq!(state.game_time, 0.0);
        assert_eq!(state.difficulty, 1.0);
    }

    #[test]
    fn spawn_interval_decreases_over_time() {
        let mut state = SurvivalState::default();
        let initial = state.spawn_interval();

        state.game_time = 60.0;
        let after_1_min = state.spawn_interval();

        state.game_time = 300.0;
        let after_5_min = state.spawn_interval();

        assert!(initial > after_1_min);
        assert!(after_1_min > after_5_min);
    }

    #[test]
    fn difficulty_scales_with_exp() {
        let mut state = SurvivalState::default();
        assert_eq!(state.calculate_difficulty(), 1.0);

        state.total_exp = 1000;
        assert!((state.calculate_difficulty() - 1.5).abs() < 0.01);

        state.total_exp = 2000;
        assert!((state.calculate_difficulty() - 2.0).abs() < 0.01);
    }

    #[test]
    fn more_creatures_available_over_time() {
        let mut state = SurvivalState::default();
        let initial = state.available_creatures().len();

        state.game_time = 120.0;
        let after_2_min = state.available_creatures().len();

        assert!(after_2_min > initial);
    }
}
