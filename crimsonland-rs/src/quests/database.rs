//! Quest database and data structures

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::creatures::components::CreatureType;

/// Database of all quests
#[derive(Resource)]
pub struct QuestDatabase {
    pub quests: Vec<QuestData>,
}

impl Default for QuestDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl QuestDatabase {
    pub fn new() -> Self {
        let mut db = Self { quests: Vec::new() };
        db.register_all_quests();
        db
    }

    pub fn get(&self, id: QuestId) -> Option<&QuestData> {
        self.quests.iter().find(|q| q.id == id)
    }

    pub fn get_by_index(&self, index: usize) -> Option<&QuestData> {
        self.quests.get(index)
    }

    fn register_all_quests(&mut self) {
        // Chapter 1: The Landing
        self.quests.push(QuestData {
            id: QuestId::Q01LandHostile,
            chapter: 1,
            name: "Land Hostile".into(),
            description: "The surface is crawling with creatures. Survive the initial onslaught."
                .into(),
            waves: vec![
                WaveData {
                    spawn_delay: 0.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::Zombie,
                            count: 10,
                            interval: 0.5,
                        },
                        SpawnEntry {
                            creature: CreatureType::Spider,
                            count: 5,
                            interval: 0.3,
                        },
                    ],
                },
                WaveData {
                    spawn_delay: 5.0,
                    spawns: vec![SpawnEntry {
                        creature: CreatureType::Zombie,
                        count: 20,
                        interval: 0.3,
                    }],
                },
            ],
            time_limit: None,
            unlock_requirement: None,
        });

        self.quests.push(QuestData {
            id: QuestId::Q02TheHunt,
            chapter: 1,
            name: "The Hunt".into(),
            description: "Spiders are everywhere. Clear them out.".into(),
            waves: vec![
                WaveData {
                    spawn_delay: 0.0,
                    spawns: vec![SpawnEntry {
                        creature: CreatureType::Spider,
                        count: 30,
                        interval: 0.2,
                    }],
                },
                WaveData {
                    spawn_delay: 3.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::Spider,
                            count: 20,
                            interval: 0.2,
                        },
                        SpawnEntry {
                            creature: CreatureType::AlienSpider,
                            count: 5,
                            interval: 1.0,
                        },
                    ],
                },
            ],
            time_limit: None,
            unlock_requirement: Some(QuestId::Q01LandHostile),
        });

        self.quests.push(QuestData {
            id: QuestId::Q03NightFall,
            chapter: 1,
            name: "Night Fall".into(),
            description: "Darkness brings more dangerous creatures.".into(),
            waves: vec![
                WaveData {
                    spawn_delay: 0.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::Zombie,
                            count: 15,
                            interval: 0.4,
                        },
                        SpawnEntry {
                            creature: CreatureType::Lizard,
                            count: 10,
                            interval: 0.5,
                        },
                    ],
                },
                WaveData {
                    spawn_delay: 5.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::Dog,
                            count: 10,
                            interval: 0.3,
                        },
                        SpawnEntry {
                            creature: CreatureType::Zombie,
                            count: 20,
                            interval: 0.3,
                        },
                    ],
                },
            ],
            time_limit: None,
            unlock_requirement: Some(QuestId::Q02TheHunt),
        });

        // Chapter 2: Deep Trouble
        self.quests.push(QuestData {
            id: QuestId::Q10Swarm,
            chapter: 2,
            name: "Swarm".into(),
            description: "An endless swarm approaches. Hold your ground.".into(),
            waves: vec![
                WaveData {
                    spawn_delay: 0.0,
                    spawns: vec![SpawnEntry {
                        creature: CreatureType::Spider,
                        count: 50,
                        interval: 0.15,
                    }],
                },
                WaveData {
                    spawn_delay: 3.0,
                    spawns: vec![SpawnEntry {
                        creature: CreatureType::Beetle,
                        count: 40,
                        interval: 0.2,
                    }],
                },
                WaveData {
                    spawn_delay: 3.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::Spider,
                            count: 30,
                            interval: 0.1,
                        },
                        SpawnEntry {
                            creature: CreatureType::AlienSpider,
                            count: 10,
                            interval: 0.5,
                        },
                    ],
                },
            ],
            time_limit: None,
            unlock_requirement: Some(QuestId::Q03NightFall),
        });

        self.quests.push(QuestData {
            id: QuestId::Q11GiantProblem,
            chapter: 2,
            name: "Giant Problem".into(),
            description: "Giants have been spotted. They're slow but deadly.".into(),
            waves: vec![
                WaveData {
                    spawn_delay: 0.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::Giant,
                            count: 3,
                            interval: 2.0,
                        },
                        SpawnEntry {
                            creature: CreatureType::Zombie,
                            count: 20,
                            interval: 0.3,
                        },
                    ],
                },
                WaveData {
                    spawn_delay: 5.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::Giant,
                            count: 5,
                            interval: 1.5,
                        },
                        SpawnEntry {
                            creature: CreatureType::Lizard,
                            count: 15,
                            interval: 0.4,
                        },
                    ],
                },
            ],
            time_limit: None,
            unlock_requirement: Some(QuestId::Q10Swarm),
        });

        // Chapter 3: The Hive
        self.quests.push(QuestData {
            id: QuestId::Q20Infestation,
            chapter: 3,
            name: "Infestation".into(),
            description: "The hive must be destroyed.".into(),
            waves: vec![
                WaveData {
                    spawn_delay: 0.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::AlienSpider,
                            count: 20,
                            interval: 0.3,
                        },
                        SpawnEntry {
                            creature: CreatureType::Spider,
                            count: 30,
                            interval: 0.2,
                        },
                    ],
                },
                WaveData {
                    spawn_delay: 5.0,
                    spawns: vec![SpawnEntry {
                        creature: CreatureType::GiantSpider,
                        count: 3,
                        interval: 3.0,
                    }],
                },
            ],
            time_limit: None,
            unlock_requirement: Some(QuestId::Q11GiantProblem),
        });

        // Boss quest
        self.quests.push(QuestData {
            id: QuestId::Q30QueenSpider,
            chapter: 3,
            name: "Queen Spider".into(),
            description: "The queen of all spiders awaits. This is the final battle.".into(),
            waves: vec![
                WaveData {
                    spawn_delay: 0.0,
                    spawns: vec![SpawnEntry {
                        creature: CreatureType::Spider,
                        count: 20,
                        interval: 0.3,
                    }],
                },
                WaveData {
                    spawn_delay: 5.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::BossSpider,
                            count: 1,
                            interval: 0.0,
                        },
                        SpawnEntry {
                            creature: CreatureType::AlienSpider,
                            count: 10,
                            interval: 1.0,
                        },
                    ],
                },
            ],
            time_limit: None,
            unlock_requirement: Some(QuestId::Q20Infestation),
        });

        // Additional quests (abbreviated - full game has 53)
        // These demonstrate the pattern for adding more quests
        self.quests.push(QuestData {
            id: QuestId::Q40AlienInvasion,
            chapter: 4,
            name: "Alien Invasion".into(),
            description: "Aliens have landed. Repel the invasion.".into(),
            waves: vec![
                WaveData {
                    spawn_delay: 0.0,
                    spawns: vec![
                        SpawnEntry {
                            creature: CreatureType::AlienShooter,
                            count: 10,
                            interval: 0.5,
                        },
                        SpawnEntry {
                            creature: CreatureType::AlienSpider,
                            count: 15,
                            interval: 0.3,
                        },
                    ],
                },
                WaveData {
                    spawn_delay: 8.0,
                    spawns: vec![SpawnEntry {
                        creature: CreatureType::BossAlien,
                        count: 1,
                        interval: 0.0,
                    }],
                },
            ],
            time_limit: None,
            unlock_requirement: Some(QuestId::Q30QueenSpider),
        });
    }
}

/// Unique identifier for each quest
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuestId {
    // Chapter 1
    Q01LandHostile,
    Q02TheHunt,
    Q03NightFall,
    Q04FirstBlood,
    Q05Outbreak,
    Q06Surrounded,
    Q07LastStand,
    Q08RunnerUp,
    Q09Ambush,

    // Chapter 2
    Q10Swarm,
    Q11GiantProblem,
    Q12Necropolis,
    Q13FireStorm,
    Q14DeadEnd,
    Q15Overrun,
    Q16Nightmare,
    Q17Carnage,
    Q18BloodBath,
    Q19Apocalypse,

    // Chapter 3
    Q20Infestation,
    Q21TheNest,
    Q22SpidersDen,
    Q23WebOfDeath,
    Q24Arachnophobia,
    Q25EightLegs,
    Q26PoisonFang,
    Q27SilkTrap,
    Q28SpiderQueen,
    Q29FinalStrike,
    Q30QueenSpider,

    // Chapter 4
    Q40AlienInvasion,
    Q41FirstContact,
    Q42Mothership,
    Q43AreaDenied,
    Q44Extermination,

    // More chapters would follow (53 total quests)
}

/// Data for a quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestData {
    pub id: QuestId,
    pub chapter: u32,
    pub name: String,
    pub description: String,
    pub waves: Vec<WaveData>,
    pub time_limit: Option<f32>,
    pub unlock_requirement: Option<QuestId>,
}

/// Data for a wave within a quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveData {
    /// Delay before this wave starts (after previous wave)
    pub spawn_delay: f32,
    /// Creatures to spawn in this wave
    pub spawns: Vec<SpawnEntry>,
}

impl WaveData {
    pub fn total_creatures(&self) -> u32 {
        self.spawns.iter().map(|s| s.count).sum()
    }
}

/// Entry for spawning a group of creatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnEntry {
    pub creature: CreatureType,
    pub count: u32,
    /// Time between each spawn
    pub interval: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_database_has_quests() {
        let db = QuestDatabase::new();
        assert!(!db.quests.is_empty());
    }

    #[test]
    fn first_quest_has_no_unlock_requirement() {
        let db = QuestDatabase::new();
        let first_quest = db.get(QuestId::Q01LandHostile).unwrap();
        assert!(first_quest.unlock_requirement.is_none());
    }

    #[test]
    fn quests_have_waves() {
        let db = QuestDatabase::new();
        for quest in &db.quests {
            assert!(!quest.waves.is_empty(), "Quest {} has no waves", quest.name);
        }
    }

    #[test]
    fn waves_have_creatures() {
        let db = QuestDatabase::new();
        for quest in &db.quests {
            for (i, wave) in quest.waves.iter().enumerate() {
                assert!(
                    wave.total_creatures() > 0,
                    "Quest {} wave {} has no creatures",
                    quest.name,
                    i
                );
            }
        }
    }

    #[test]
    fn can_get_quest_by_id() {
        let db = QuestDatabase::new();
        assert!(db.get(QuestId::Q01LandHostile).is_some());
        assert!(db.get(QuestId::Q30QueenSpider).is_some());
    }
}
