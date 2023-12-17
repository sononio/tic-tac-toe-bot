use super::error::{GameProcessError, HookError, CoreError, BotError};

pub trait ResultExt<T> {
    fn pack_err(self) -> Result<T, GameProcessError>;
}

impl <T> ResultExt<T> for Result<T, HookError> {
    fn pack_err(self) -> Result<T, GameProcessError> {
        self.map_err(GameProcessError::Hook)
    }
}

impl <T> ResultExt<T> for Result<T, CoreError> {
    fn pack_err(self) -> Result<T, GameProcessError> {
        self.map_err(GameProcessError::Core)
    }
}

impl <T> ResultExt<T> for Result<T, BotError> {
    fn pack_err(self) -> Result<T, GameProcessError> {
        self.map_err(GameProcessError::Bot)
    }
}