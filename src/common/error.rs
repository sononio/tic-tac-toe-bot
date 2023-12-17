use super::bot::{Coords, Size, Side};

#[derive(Debug)]
pub struct BotError {
    pub message: String,
}

#[derive(Debug)]
pub struct HookError {
    pub message: String,
}

#[derive(Debug)]
pub struct CoreError {
    pub message: String,
}

#[derive(Debug)]
pub enum GameProcessError {
    Bot(BotError),
    Hook(HookError),
    Core(CoreError),
}

impl CoreError {
    pub fn of_wrong_coords(coords: Coords, size: Size) -> Self {
        CoreError {
            message: format!("Wrong coordinates ${coords:?}. It has to be in size ${size:?}")
        }
    }

    pub fn of_filled_cell(coords: Coords, current: Side) -> Self {
        CoreError {
            message: format!("Cell ${coords:?} is filled. Current value: ${current:?}")
        }
    }
}