//! Rush Mode
//!
//! Time-limited rounds with pre-selected perks, score-based gameplay,
//! and loadout-based weapons (no pickups).

use bevy::prelude::*;
use rand::Rng;

use crate::creatures::{CreatureType, SpawnCreatureEvent};
use crate::perks::components::{PerkBonuses, PerkId, PerkInventory};
use crate::states::GameState;
use crate::weapons::components::WeaponId;

/// Plugin for rush mode functionality
pub struct RushPlugin;

impl Plugin for RushPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RushScoreEvent>()
            .add_systems(OnEnter(GameState::Playing), setup_rush_mode)
            .add_systems(OnExit(GameState::Playing), cleanup_rush_mode)
            .add_systems(
                Update,
                (
                    update_rush_timer,
                    spawn_rush_creatures,
                    handle_rush_kills,
                    track_rush_score,
                    handle_rush_round_end,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing))
                    .run_if(resource_exists::<RushState>),
            );
    }
}

/// Event for scoring in Rush mode
#[derive(Event, Clone)]
pub struct RushScoreEvent {
    pub points: u32,
    pub source: ScoreSource,
}

/// Source of score points
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreSource {
    /// Points from killing a creature
    Kill(CreatureType),
    /// Bonus for remaining time
    TimeBonus,
}

/// Rush mode loadout configuration
#[derive(Debug, Clone)]
pub struct RushLoadout {
    /// Starting weapon
    pub weapon: WeaponId,
    /// Pre-selected perks
    pub perks: Vec<PerkId>,
    /// Loadout name for display
    pub name: String,
}

impl Default for RushLoadout {
    fn default() -> Self {
        Self {
            weapon: WeaponId::AssaultRifle,
            perks: vec![
                PerkId::TriggerHappy,
                PerkId::DeadlyAccuracy,
                PerkId::LongBarrel,
            ],
            name: "Assault".to_string(),
        }
    }
}

/// Available rush loadouts
pub fn available_loadouts() -> Vec<RushLoadout> {
    vec![
        RushLoadout {
            weapon: WeaponId::AssaultRifle,
            perks: vec![
                PerkId::TriggerHappy,
                PerkId::DeadlyAccuracy,
                PerkId::LongBarrel,
            ],
            name: "Assault".to_string(),
        },
        RushLoadout {
            weapon: WeaponId::Shotgun,
            perks: vec![
                PerkId::FastReload,
                PerkId::DeadlyAccuracy,
                PerkId::ThickSkin,
            ],
            name: "Shotgunner".to_string(),
        },
        RushLoadout {
            weapon: WeaponId::Minigun,
            perks: vec![
                PerkId::TriggerHappy,
                PerkId::LongBarrel,
                PerkId::ThickSkin,
            ],
            name: "Heavy".to_string(),
        },
        RushLoadout {
            weapon: WeaponId::RocketLauncher,
            perks: vec![
                PerkId::Pyromaniac,
                PerkId::FastReload,
                PerkId::ThickSkin,
            ],
            name: "Demolition".to_string(),
        },
        RushLoadout {
            weapon: WeaponId::PlasmaRifle,
            perks: vec![
                PerkId::TriggerHappy,
                PerkId::DeadlyAccuracy,
                PerkId::CriticalHit,
            ],
            name: "Plasma".to_string(),
        },
    ]
}

/// Resource tracking rush mode state
#[derive(Resource, Debug)]
pub struct RushState {
    /// Time remaining in seconds
    pub time_remaining: f32,
    /// Total round duration
    pub round_duration: f32,
    /// Current score
    pub score: u32,
    /// Kill streak for combo bonuses
    pub kill_streak: u32,
    /// Time since last kill (for streak tracking)
    pub streak_timer: f32,
    /// Time since last creature spawn
    pub spawn_timer: f32,
    /// Active loadout
    pub loadout: RushLoadout,
    /// Whether the round is over
    pub round_over: bool,
    /// Total kills this round
    pub total_kills: u32,
}

impl RushState {
    pub fn new(duration: f32, loadout: RushLoadout) -> Self {
        Self {
            time_remaining: duration,
            round_duration: duration,
            score: 0,
            kill_streak: 0,
            streak_timer: 0.0,
            spawn_timer: 0.0,
            loadout,
            round_over: false,
            total_kills: 0,
        }
    }

    /// Calculate spawn interval (constant in Rush mode)
    pub fn spawn_interval(&self) -> f32 {
        // Rush mode has a constant fast spawn rate
        0.5
    }

    /// Get score multiplier based on kill streak
    pub fn streak_multiplier(&self) -> f32 {
        match self.kill_streak {
            0..=4 => 1.0,
            5..=9 => 1.5,
            10..=19 => 2.0,
            20..=49 => 3.0,
            _ => 5.0,
        }
    }

    /// Get base score for a creature type
    pub fn creature_score(creature_type: CreatureType) -> u32 {
        match creature_type {
            CreatureType::Zombie => 10,
            CreatureType::Spider => 15,
            CreatureType::Lizard => 20,
            CreatureType::Beetle => 15,
            CreatureType::AlienSpider => 35,
            CreatureType::Giant => 100,
            CreatureType::Necromancer => 80,
            CreatureType::GiantSpider => 120,
            CreatureType::Dog => 25,
            CreatureType::Runner => 30,
            CreatureType::AlienShooter => 40,
            CreatureType::Turret => 50,
            CreatureType::Ghost => 45,
            CreatureType::Exploder => 35,
            CreatureType::Splitter => 40,
            CreatureType::BossSpider => 500,
            CreatureType::BossAlien => 800,
            CreatureType::BossNest => 1000,
        }
    }

    /// Calculate time bonus at end of round
    pub fn calculate_time_bonus(&self) -> u32 {
        // No time bonus if round ended naturally
        if self.time_remaining <= 0.0 {
            return 0;
        }
        // Bonus for early completion (if that were possible)
        (self.time_remaining * 10.0) as u32
    }

    /// Pick a random creature, weighted toward more variety in Rush
    pub fn pick_creature(&self) -> CreatureType {
        let mut rng = rand::thread_rng();
        let elapsed = self.round_duration - self.time_remaining;

        // Gradually introduce harder creatures
        let available: Vec<CreatureType> = if elapsed < 30.0 {
            vec![CreatureType::Zombie, CreatureType::Spider, CreatureType::Beetle]
        } else if elapsed < 60.0 {
            vec![
                CreatureType::Zombie,
                CreatureType::Spider,
                CreatureType::Beetle,
                CreatureType::Runner,
                CreatureType::Dog,
            ]
        } else if elapsed < 90.0 {
            vec![
                CreatureType::Zombie,
                CreatureType::Spider,
                CreatureType::Runner,
                CreatureType::Dog,
                CreatureType::Lizard,
                CreatureType::AlienSpider,
                CreatureType::Ghost,
            ]
        } else {
            vec![
                CreatureType::Zombie,
                CreatureType::Spider,
                CreatureType::Runner,
                CreatureType::Dog,
                CreatureType::Lizard,
                CreatureType::AlienSpider,
                CreatureType::AlienShooter,
                CreatureType::GiantSpider,
                CreatureType::Necromancer,
                CreatureType::Exploder,
            ]
        };

        available[rng.gen_range(0..available.len())]
    }
}

impl Default for RushState {
    fn default() -> Self {
        Self::new(120.0, RushLoadout::default()) // 2 minute rounds
    }
}

/// Sets up rush mode when entering Playing state (if rush mode is active)
/// Applies loadout perks and weapon to the player
fn setup_rush_mode(
    rush: Option<Res<RushState>>,
    mut player_query: Query<
        (&mut PerkInventory, &mut PerkBonuses, &mut crate::weapons::components::EquippedWeapon),
        With<crate::player::components::Player>,
    >,
) {
    let Some(rush) = rush else { return };

    for (mut inventory, mut bonuses, mut weapon) in player_query.iter_mut() {
        // Apply loadout perks using the apply_loadout_to_player function
        apply_loadout_to_player(&rush.loadout, &mut inventory, &mut bonuses);

        // Set the loadout weapon
        *weapon = crate::weapons::components::EquippedWeapon::new(
            rush.loadout.weapon,
            Some(200), // Rush mode gives generous ammo
        );

        info!(
            "Rush loadout applied: {} with {} perks",
            rush.loadout.name,
            rush.loadout.perks.len()
        );
    }
}

/// Cleans up rush mode when leaving Playing state
fn cleanup_rush_mode(mut commands: Commands) {
    commands.remove_resource::<RushState>();
}

/// Updates the rush timer and checks for round end
fn update_rush_timer(time: Res<Time>, mut rush: ResMut<RushState>) {
    if rush.round_over {
        return;
    }

    rush.time_remaining -= time.delta_seconds();
    rush.spawn_timer += time.delta_seconds();

    // Update streak timer
    rush.streak_timer += time.delta_seconds();
    if rush.streak_timer > 2.0 {
        // Streak breaks after 2 seconds without a kill
        rush.kill_streak = 0;
    }

    // Check for round end
    if rush.time_remaining <= 0.0 {
        rush.time_remaining = 0.0;
        rush.round_over = true;
        // Time bonus already 0 when time runs out
    }
}

/// Spawns creatures at a fast rate in Rush mode
fn spawn_rush_creatures(
    mut rush: ResMut<RushState>,
    mut spawn_events: EventWriter<SpawnCreatureEvent>,
) {
    if rush.round_over {
        return;
    }

    let interval = rush.spawn_interval();

    if rush.spawn_timer >= interval {
        rush.spawn_timer = 0.0;

        // Spawn 2-4 creatures at a time in Rush mode
        let spawn_count = rand::thread_rng().gen_range(2..=4);

        for _ in 0..spawn_count {
            let creature_type = rush.pick_creature();
            spawn_events.send(SpawnCreatureEvent {
                creature_type,
                position: None,
            });
        }
    }
}

/// Tracks score from kills
fn track_rush_score(
    mut rush: ResMut<RushState>,
    mut score_events: EventReader<RushScoreEvent>,
) {
    for event in score_events.read() {
        let multiplier = rush.streak_multiplier();
        let points = (event.points as f32 * multiplier) as u32;
        rush.score += points;

        match event.source {
            ScoreSource::Kill(creature_type) => {
                // Log the kill with creature type
                info!("Kill: {:?} - {} pts (x{:.1})", creature_type, points, multiplier);
                rush.kill_streak += 1;
                rush.streak_timer = 0.0;
                rush.total_kills += 1;

                // Award combo bonus at milestones (10, 25, 50, 100 kills)
                // Add directly to score instead of sending another event (avoids Bevy ECS conflict)
                let streak = rush.kill_streak;
                if streak == 10 || streak == 25 || streak == 50 || streak == 100 {
                    let combo_bonus = streak * 10;
                    let combo_points = (combo_bonus as f32 * multiplier) as u32;
                    rush.score += combo_points;
                    info!("Combo bonus ({} streak): {} pts", streak, combo_points);
                }
            }
            ScoreSource::TimeBonus => {
                info!("Time bonus: {} pts", points);
            }
        }
    }
}

/// Handles creature deaths in Rush mode - sends score events
fn handle_rush_kills(
    rush: Option<Res<RushState>>,
    mut death_events: EventReader<crate::creatures::systems::CreatureDeathEvent>,
    mut score_events: EventWriter<RushScoreEvent>,
) {
    // Only run in Rush mode
    if rush.is_none() {
        return;
    }

    for event in death_events.read() {
        // Use RushState::creature_score to get base points
        let base_score = RushState::creature_score(event.creature_type);
        score_events.send(RushScoreEvent {
            points: base_score,
            source: ScoreSource::Kill(event.creature_type),
        });
    }
}

/// Handles round end in Rush mode - calculates time bonus
fn handle_rush_round_end(
    mut rush: ResMut<RushState>,
    mut score_events: EventWriter<RushScoreEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !rush.round_over {
        return;
    }

    // Calculate time bonus using the method
    let time_bonus = rush.calculate_time_bonus();
    if time_bonus > 0 {
        score_events.send(RushScoreEvent {
            points: time_bonus,
            source: ScoreSource::TimeBonus,
        });
    }

    // Display loadout info
    info!(
        "Rush Round Over! Loadout: {} | Final Score: {}",
        rush.loadout.name, rush.score
    );

    // Transition to game over
    next_state.set(GameState::GameOver);

    // Mark as processed to prevent re-triggering
    rush.round_over = false;
    rush.time_remaining = -1.0; // Prevent re-triggering
}

/// Applies loadout perks to a player (recalculates bonuses from inventory)
pub fn apply_loadout_to_player(
    loadout: &RushLoadout,
    inventory: &mut PerkInventory,
    bonuses: &mut PerkBonuses,
) {
    for perk_id in &loadout.perks {
        inventory.add_perk(*perk_id);
    }
    // Recalculate all bonuses from the updated inventory
    *bonuses = PerkBonuses::calculate(inventory);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rush_state_defaults() {
        let state = RushState::default();
        assert_eq!(state.round_duration, 120.0);
        assert_eq!(state.score, 0);
        assert!(!state.round_over);
    }

    #[test]
    fn streak_multiplier_increases() {
        let mut state = RushState::default();
        assert_eq!(state.streak_multiplier(), 1.0);

        state.kill_streak = 5;
        assert_eq!(state.streak_multiplier(), 1.5);

        state.kill_streak = 10;
        assert_eq!(state.streak_multiplier(), 2.0);

        state.kill_streak = 50;
        assert_eq!(state.streak_multiplier(), 5.0);
    }

    #[test]
    fn creature_scores_vary() {
        assert!(RushState::creature_score(CreatureType::Giant) > RushState::creature_score(CreatureType::Zombie));
        assert!(RushState::creature_score(CreatureType::BossNest) > RushState::creature_score(CreatureType::Giant));
    }

    #[test]
    fn loadouts_have_three_perks() {
        for loadout in available_loadouts() {
            assert_eq!(loadout.perks.len(), 3);
        }
    }

    #[test]
    fn apply_loadout_adds_perks() {
        let loadout = RushLoadout::default();
        let mut inventory = PerkInventory::new();
        let mut bonuses = PerkBonuses::default();

        apply_loadout_to_player(&loadout, &mut inventory, &mut bonuses);

        // Should have all perks from loadout
        for perk_id in &loadout.perks {
            assert!(inventory.has_perk(*perk_id));
        }
    }
}
