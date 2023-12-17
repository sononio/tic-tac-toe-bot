use super::{bot::{Turn, Coords}, game_process::PlayInfo, error::HookError};

pub trait Hooks {
    fn init(&mut self) -> Result<PlayInfo, HookError>;
    fn wait_for_turn(&mut self) -> Result<Option<Turn>, HookError>;
    fn make_turn(&mut self, turn: &Coords) -> Result<(), HookError>;
}
