use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoordsDto {
    pub x: usize,
    pub y: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterInfoDto {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDto {
    pub id: String,
    pub status: StatusDto,
    pub side: Option<SideDto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatusDto {
    #[serde(rename = "IN_QUEUE")]
    InQueue,
    #[serde(rename = "IN_GAME")]
    InGame,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SideDto {
    X,
    O,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CellDto {
    X,
    O,
    #[serde(rename = "EMPTY")]
    Empty,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldDto {
    pub cells: Vec<Vec<CellDto>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSettingsDto {
    pub width: usize,
    pub height: usize, 
    #[serde(rename = "winCondition")]
    pub win_condition: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum StateDto {
    #[serde(rename = "X_MOVE")]
    XMove,
    #[serde(rename = "O_MOVE")]
    OMove,
    #[serde(rename = "FINISHED")]
    Finished,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResultDto {
    #[serde(rename = "X_WIN")]
    XWin,
    #[serde(rename = "O_WIN")]
    OWin,
    #[serde(rename = "Draw")]
    Draw,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameDto {
    pub field: FieldDto,
    pub settings: GameSettingsDto,
    pub state: StateDto,
    #[serde(rename = "currentTurn")]
    pub current_turn: usize,
    #[serde(rename = "lastTurn")]
    pub last_turn: Option<TurnDto>,
    pub result: Option<ResultDto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurnDto {
    pub side: SideDto,
    pub coords: CoordsDto,
}