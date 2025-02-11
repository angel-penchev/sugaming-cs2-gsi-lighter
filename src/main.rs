pub mod round_manager;

use poem::{handler, listener::TcpListener, post, web::Json, Route, Server};
use round_manager::{GameStateIntegrationPayload, RoundManager, RoundPhase};

static ROUND_MANAGER: once_cell::sync::Lazy<tokio::sync::Mutex<RoundManager>> =
    once_cell::sync::Lazy::new(|| tokio::sync::Mutex::new(RoundManager::new()));

#[handler]
async fn update(data: Json<GameStateIntegrationPayload>) {
    println!("{:#?}", data);
    if let Some(round_data) = &data.0.round {
        let phase_str = &round_data.phase;
        let round_phase = match phase_str.as_str() {
            "freezetime" => RoundPhase::Freezetime,
            "live" => RoundPhase::Live,
            "bomb" if round_data.bomb == Some("planted".to_string()) => RoundPhase::BombPlanted,
            "bomb" if round_data.bomb == Some("defused".to_string()) => RoundPhase::BombDefused,
            "bomb" if round_data.bomb == Some("exploded".to_string()) => RoundPhase::BombExploded,
            "over" => RoundPhase::Over,
            _ => {
                eprintln!("Unknown round phase: {}", phase_str);
                return;
            }
        };

        let mut manager = ROUND_MANAGER.lock().await;
        manager.set_phase(round_phase);
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", post(update));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
