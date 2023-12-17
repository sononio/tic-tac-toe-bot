use std::time::Duration;

use const_format::formatcp;
use ureq::Error;

use super::dto::{CoordsDto, GameDto, PlayerDto, RegisterInfoDto};

static MATCHMAKING_QUEUE: &str =
    formatcp!("http://localhost:8080/api/v1/matchmaking/queue");
static PLAYERS: &str =
    formatcp!("http://localhost:8080/api/v1/players");

pub fn post_mathchmaking_queue(body: RegisterInfoDto) -> Result<PlayerDto, Error> {
    let res = ureq::post(MATCHMAKING_QUEUE)
        .set("Content-Type", "application/json")
        .send_string(&serde_json::to_string(&body).unwrap())?;

    let ans = &res.into_string()?;
    let ans: PlayerDto = serde_json::from_str(ans).unwrap();

    Ok(ans)
}

pub fn get_players(
    id: &str,
    long_polling_enabled: bool,
    long_polling_timeout: usize,
) -> Result<PlayerDto, Error> {
    let res = ureq::get(&format!("{PLAYERS}/{id}"))
        .timeout(Duration::from_secs(3600))
        .query("longPollingEnabled", &long_polling_enabled.to_string())
        .query("longPollingTimeout", &long_polling_timeout.to_string())
        .set("Content-Type", "application/json")
        .call()?;

    let ans = &res.into_string()?;
    let ans: PlayerDto = serde_json::from_str(ans).unwrap();

    Ok(ans)
}

pub fn get_player_game(
    id: &str,
    long_polling_enabled: bool,
    long_polling_timeout: usize,
) -> Result<GameDto, Error> {
    let res = ureq::get(&format!("{PLAYERS}/{id}/game"))
        .timeout(Duration::from_secs(3600))
        .query("longPollingEnabled", &long_polling_enabled.to_string())
        .query("longPollingTimeout", &long_polling_timeout.to_string())
        .set("Content-Type", "application/json")
        .call()?;
    let ans = &res.into_string()?;
    let ans: GameDto = serde_json::from_str(ans).unwrap();

    Ok(ans)
}

pub fn post_game_turn(id: &str, body: CoordsDto) -> Result<(), Error> {
    let _ = ureq::post(&format!("{PLAYERS}/{id}/game/turn"))
        .set("Content-Type", "application/json")
        .send_string(&serde_json::to_string(&body).unwrap());

    Ok(())
}
