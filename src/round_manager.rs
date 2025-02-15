use crate::led::{blink, rgb_cycle, timed_blink};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum RoundPhase {
    Freezetime,
    Live,
    BombPlanted,
    BombDefused,
    BombExploded,
    Over,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoundData {
    pub phase: String,
    pub win_team: Option<String>,
    pub bomb: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameStateIntegrationPayload {
    pub round: Option<RoundData>,
}

pub struct RoundManager {
    current_phase: Option<RoundPhase>,
    current_task: Option<JoinHandle<()>>,
}

impl Default for RoundManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RoundManager {
    pub fn new() -> Self {
        RoundManager {
            current_phase: None,
            current_task: None,
        }
    }

    pub fn set_phase(&mut self, new_phase: RoundPhase) {
        if self.current_phase == Some(new_phase) {
            println!("Phase is already: {:?}", new_phase);
            return;
        }

        println!("Phase changed to: {:?}", new_phase);

        // Stop the current task
        if let Some(task) = self.current_task.take() {
            println!("Aborting previous task...");
            task.abort();
        }

        // Start a new task based on the new phase
        match new_phase {
            RoundPhase::Freezetime => {
                self.current_task = Some(tokio::spawn(freezetime_function()));
            }
            RoundPhase::Live => {
                self.current_task = Some(tokio::spawn(live_function()));
            }
            RoundPhase::BombPlanted => {
                self.current_task = Some(tokio::spawn(bomb_planted_function()));
            }
            RoundPhase::BombDefused => {
                self.current_task = Some(tokio::spawn(bomb_defused_function()));
            }
            RoundPhase::BombExploded => {
                self.current_task = Some(tokio::spawn(bomb_exploded_function()));
            }
            RoundPhase::Over => {
                self.current_task = Some(tokio::spawn(round_over_function()));
            }
        }
        self.current_phase = Some(new_phase);
    }
}

async fn freezetime_function() {
    println!("Entering Freezetime Phase");
    println!("Freezetime function running...");
    rgb_cycle();
    println!("Freezetime function ending...");
}

async fn live_function() {
    println!("Entering Live Phase");
    println!("Live function running...");
    blink();
    println!("Live function ending...");
}

async fn bomb_planted_function() {
    println!("Bomb Planted!");
    println!("Bomb planted function running...");
    timed_blink(40000);
    println!("Bomb planted function ending...");
}

async fn bomb_defused_function() {
    println!("Bomb Defused!");
    println!("Bomb defused function running...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    println!("Bomb defused function ending...");
}

async fn bomb_exploded_function() {
    println!("Bomb Exploded!");
    println!("Bomb exploded function running...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    println!("Bomb exploded function ending...");
}

async fn round_over_function() {
    println!("Round Over");
    println!("Round over function running...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    println!("Round over function ending...");
}
