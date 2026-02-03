//! Quest builders
//!
//! Contains specialized spawn logic for specific quests.
//! Each quest can have custom spawn patterns beyond the basic wave data.

use bevy::prelude::*;
use crate::creatures::components::CreatureType;

/// Command to spawn a creature
#[derive(Debug, Clone)]
pub struct SpawnCommand {
    pub creature_type: CreatureType,
    pub position: Option<Vec3>,
    /// Optional delay before spawning (seconds)
    pub delay: f32,
}

impl SpawnCommand {
    /// Create a spawn command for immediate spawning at a random position
    pub fn immediate(creature_type: CreatureType) -> Self {
        Self {
            creature_type,
            position: None,
            delay: 0.0,
        }
    }

    /// Create a spawn command at a specific position
    pub fn at_position(creature_type: CreatureType, position: Vec3) -> Self {
        Self {
            creature_type,
            position: Some(position),
            delay: 0.0,
        }
    }

    /// Create a delayed spawn command
    pub fn delayed(creature_type: CreatureType, delay: f32) -> Self {
        Self {
            creature_type,
            position: None,
            delay,
        }
    }
}

/// Trait for custom quest spawn logic
pub trait QuestBuilder: Send + Sync {
    /// Called each frame to handle custom spawning
    fn update(&mut self, delta: f32) -> Vec<SpawnCommand>;

    /// Returns true when the quest is complete
    fn is_complete(&self) -> bool;

    /// Get the name of this builder for debugging
    fn name(&self) -> &str;
}

/// Simple timed wave builder - spawns creatures at regular intervals
pub struct TimedWaveBuilder {
    /// Creatures to spawn
    creatures: Vec<CreatureType>,
    /// Time between spawns
    spawn_interval: f32,
    /// Current timer
    timer: f32,
    /// Index into creatures list
    current_index: usize,
    /// Whether all creatures have been spawned
    spawned_all: bool,
}

impl TimedWaveBuilder {
    pub fn new(creatures: Vec<CreatureType>, spawn_interval: f32) -> Self {
        Self {
            creatures,
            spawn_interval,
            timer: 0.0,
            current_index: 0,
            spawned_all: false,
        }
    }
}

impl QuestBuilder for TimedWaveBuilder {
    fn update(&mut self, delta: f32) -> Vec<SpawnCommand> {
        if self.spawned_all {
            return Vec::new();
        }

        self.timer += delta;
        let mut commands = Vec::new();

        while self.timer >= self.spawn_interval && self.current_index < self.creatures.len() {
            commands.push(SpawnCommand::immediate(self.creatures[self.current_index]));
            self.current_index += 1;
            self.timer -= self.spawn_interval;
        }

        if self.current_index >= self.creatures.len() {
            self.spawned_all = true;
        }

        commands
    }

    fn is_complete(&self) -> bool {
        self.spawned_all
    }

    fn name(&self) -> &str {
        "TimedWaveBuilder"
    }
}

/// Boss wave builder - spawns minions, then a boss
pub struct BossWaveBuilder {
    /// Minion creatures to spawn first
    minions: Vec<CreatureType>,
    /// Boss creature type
    boss: CreatureType,
    /// Spawn interval for minions
    minion_interval: f32,
    /// Current timer
    timer: f32,
    /// Current minion index
    minion_index: usize,
    /// Whether boss has been spawned
    boss_spawned: bool,
    /// Delay after last minion before boss
    boss_delay: f32,
    /// Timer for boss delay
    boss_timer: f32,
}

impl BossWaveBuilder {
    pub fn new(minions: Vec<CreatureType>, boss: CreatureType) -> Self {
        Self {
            minions,
            boss,
            minion_interval: 0.5,
            timer: 0.0,
            minion_index: 0,
            boss_spawned: false,
            boss_delay: 2.0,
            boss_timer: 0.0,
        }
    }

    pub fn with_minion_interval(mut self, interval: f32) -> Self {
        self.minion_interval = interval;
        self
    }

    pub fn with_boss_delay(mut self, delay: f32) -> Self {
        self.boss_delay = delay;
        self
    }
}

impl QuestBuilder for BossWaveBuilder {
    fn update(&mut self, delta: f32) -> Vec<SpawnCommand> {
        if self.boss_spawned {
            return Vec::new();
        }

        let mut commands = Vec::new();

        // Spawn minions first
        if self.minion_index < self.minions.len() {
            self.timer += delta;
            while self.timer >= self.minion_interval && self.minion_index < self.minions.len() {
                commands.push(SpawnCommand::immediate(self.minions[self.minion_index]));
                self.minion_index += 1;
                self.timer -= self.minion_interval;
            }
        } else {
            // All minions spawned, wait for boss
            self.boss_timer += delta;
            if self.boss_timer >= self.boss_delay {
                commands.push(SpawnCommand::immediate(self.boss));
                self.boss_spawned = true;
            }
        }

        commands
    }

    fn is_complete(&self) -> bool {
        self.boss_spawned
    }

    fn name(&self) -> &str {
        "BossWaveBuilder"
    }
}

/// Swarm builder - spawns many weak creatures in bursts
pub struct SwarmBuilder {
    /// Creature type to swarm with
    creature_type: CreatureType,
    /// Number of bursts
    total_bursts: u32,
    /// Creatures per burst
    creatures_per_burst: u32,
    /// Time between bursts
    burst_interval: f32,
    /// Current burst count
    current_burst: u32,
    /// Timer
    timer: f32,
}

impl SwarmBuilder {
    pub fn new(creature_type: CreatureType, total_bursts: u32, creatures_per_burst: u32) -> Self {
        Self {
            creature_type,
            total_bursts,
            creatures_per_burst,
            burst_interval: 3.0,
            current_burst: 0,
            timer: 0.0,
        }
    }

    pub fn with_burst_interval(mut self, interval: f32) -> Self {
        self.burst_interval = interval;
        self
    }
}

impl QuestBuilder for SwarmBuilder {
    fn update(&mut self, delta: f32) -> Vec<SpawnCommand> {
        if self.current_burst >= self.total_bursts {
            return Vec::new();
        }

        self.timer += delta;
        let mut commands = Vec::new();

        if self.timer >= self.burst_interval {
            self.timer = 0.0;
            self.current_burst += 1;

            // Spawn burst of creatures
            for _ in 0..self.creatures_per_burst {
                commands.push(SpawnCommand::immediate(self.creature_type));
            }
        }

        commands
    }

    fn is_complete(&self) -> bool {
        self.current_burst >= self.total_bursts
    }

    fn name(&self) -> &str {
        "SwarmBuilder"
    }
}

/// Wave type for builder selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveType {
    /// Standard timed spawning
    Standard,
    /// Swarm burst spawning
    Swarm { bursts: u32, per_burst: u32 },
    /// Boss wave with minions
    Boss,
}

/// Create a standard quest builder based on wave parameters
pub fn create_standard_builder(
    creatures: Vec<(CreatureType, u32)>,
    has_boss: Option<CreatureType>,
) -> Box<dyn QuestBuilder> {
    // Expand creature counts into a flat list
    let creature_list: Vec<CreatureType> = creatures
        .into_iter()
        .flat_map(|(ct, count)| std::iter::repeat_n(ct, count as usize))
        .collect();

    if let Some(boss) = has_boss {
        Box::new(
            BossWaveBuilder::new(creature_list, boss)
                .with_minion_interval(0.4)
                .with_boss_delay(3.0),
        )
    } else {
        Box::new(TimedWaveBuilder::new(creature_list, 0.5))
    }
}

/// Create a builder for a specific wave type
pub fn create_wave_builder(
    wave_type: WaveType,
    creature_type: CreatureType,
    total_count: u32,
    boss: Option<CreatureType>,
) -> Box<dyn QuestBuilder> {
    match wave_type {
        WaveType::Standard => {
            let creatures = std::iter::repeat_n(creature_type, total_count as usize).collect();
            Box::new(TimedWaveBuilder::new(creatures, 0.5))
        }
        WaveType::Swarm { bursts, per_burst } => {
            Box::new(
                SwarmBuilder::new(creature_type, bursts, per_burst).with_burst_interval(2.5),
            )
        }
        WaveType::Boss => {
            let minions = std::iter::repeat_n(creature_type, total_count as usize).collect();
            let boss_type = boss.unwrap_or(CreatureType::BossSpider);
            Box::new(
                BossWaveBuilder::new(minions, boss_type)
                    .with_minion_interval(0.3)
                    .with_boss_delay(2.0),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_command_immediate() {
        let cmd = SpawnCommand::immediate(CreatureType::Zombie);
        assert_eq!(cmd.creature_type, CreatureType::Zombie);
        assert!(cmd.position.is_none());
        assert_eq!(cmd.delay, 0.0);
    }

    #[test]
    fn spawn_command_at_position() {
        let pos = Vec3::new(100.0, 200.0, 0.0);
        let cmd = SpawnCommand::at_position(CreatureType::Spider, pos);
        assert_eq!(cmd.creature_type, CreatureType::Spider);
        assert_eq!(cmd.position, Some(pos));
    }

    #[test]
    fn timed_wave_builder_spawns_all() {
        let creatures = vec![CreatureType::Zombie, CreatureType::Spider, CreatureType::Beetle];
        let mut builder = TimedWaveBuilder::new(creatures, 0.1);

        assert!(!builder.is_complete());

        // Simulate time passing
        let mut total_spawned = 0;
        for _ in 0..50 {
            let commands = builder.update(0.1);
            total_spawned += commands.len();
            if builder.is_complete() {
                break;
            }
        }

        assert!(builder.is_complete());
        assert_eq!(total_spawned, 3);
    }

    #[test]
    fn boss_wave_builder_spawns_boss_last() {
        let minions = vec![CreatureType::Zombie, CreatureType::Zombie];
        let mut builder = BossWaveBuilder::new(minions, CreatureType::BossSpider)
            .with_minion_interval(0.1)
            .with_boss_delay(0.1);

        assert!(!builder.is_complete());

        let mut spawned_types = Vec::new();
        for _ in 0..50 {
            let commands = builder.update(0.1);
            for cmd in commands {
                spawned_types.push(cmd.creature_type);
            }
            if builder.is_complete() {
                break;
            }
        }

        assert!(builder.is_complete());
        assert_eq!(spawned_types.len(), 3);
        assert_eq!(*spawned_types.last().unwrap(), CreatureType::BossSpider);
    }

    #[test]
    fn swarm_builder_spawns_in_bursts() {
        let mut builder = SwarmBuilder::new(CreatureType::Spider, 2, 5)
            .with_burst_interval(0.1);

        assert!(!builder.is_complete());

        let mut burst_sizes = Vec::new();
        for _ in 0..50 {
            let commands = builder.update(0.1);
            if !commands.is_empty() {
                burst_sizes.push(commands.len());
            }
            if builder.is_complete() {
                break;
            }
        }

        assert!(builder.is_complete());
        assert_eq!(burst_sizes.len(), 2);
        assert!(burst_sizes.iter().all(|&s| s == 5));
    }

    #[test]
    fn spawn_command_delayed() {
        let cmd = SpawnCommand::delayed(CreatureType::Ghost, 2.5);
        assert_eq!(cmd.creature_type, CreatureType::Ghost);
        assert!(cmd.position.is_none());
        assert_eq!(cmd.delay, 2.5);
    }

    #[test]
    fn create_wave_builder_standard() {
        let mut builder = create_wave_builder(
            WaveType::Standard,
            CreatureType::Zombie,
            3,
            None,
        );
        assert!(!builder.is_complete());

        let mut total = 0;
        for _ in 0..50 {
            total += builder.update(0.5).len();
            if builder.is_complete() {
                break;
            }
        }
        assert!(builder.is_complete());
        assert_eq!(total, 3);
    }

    #[test]
    fn create_wave_builder_swarm() {
        let mut builder = create_wave_builder(
            WaveType::Swarm { bursts: 2, per_burst: 4 },
            CreatureType::Spider,
            8,
            None,
        );
        assert!(!builder.is_complete());

        let mut total = 0;
        for _ in 0..50 {
            total += builder.update(2.5).len();
            if builder.is_complete() {
                break;
            }
        }
        assert!(builder.is_complete());
        assert_eq!(total, 8);
    }

    #[test]
    fn create_wave_builder_boss() {
        let mut builder = create_wave_builder(
            WaveType::Boss,
            CreatureType::Zombie,
            2,
            Some(CreatureType::BossAlien),
        );
        assert!(!builder.is_complete());

        let mut types = Vec::new();
        for _ in 0..100 {
            for cmd in builder.update(0.5) {
                types.push(cmd.creature_type);
            }
            if builder.is_complete() {
                break;
            }
        }
        assert!(builder.is_complete());
        assert_eq!(*types.last().unwrap(), CreatureType::BossAlien);
    }
}
