use super::{
    bot::{Bot, Cell, Coords, Field, Side, Size, Turn},
    error::GameProcessError,
    error_utils::ResultExt,
    hooks::Hooks,
    utils::Direction,
};

pub struct GameProcess {
    bot: Box<dyn Bot>,
    hooks: Box<dyn Hooks>,
    state: GameState,
    field: Option<Field>,
    play_info: Option<PlayInfo>,
    history: Vec<Turn>,
    bot_prediction: Option<Turn>,
}

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Initial,
    TurnWaiting,
    BotThinking,
    TurnMaking,
    Finished(GameResult),
}

#[derive(Clone, Copy, Debug)]
pub enum GameResult {
    Win,
    Defeat,
    Draw,
}

pub struct PlayInfo {
    pub field_size: Size,
    pub win_condition: usize,
    pub side: Side,
}

impl GameProcess {
    pub fn new(bot: Box<dyn Bot>, hooks: Box<dyn Hooks>) -> Self {
        GameProcess {
            bot,
            hooks,
            state: GameState::Initial,
            field: None,
            play_info: None,
            history: Default::default(),
            bot_prediction: None,
        }
    }

    pub fn run(&mut self) -> Result<GameResult, GameProcessError> {
        loop {
            match &self.state {
                GameState::Finished(result) => return Ok(*result),
                _ => {
                    self.run_step()?;
                }
            }
        }
    }

    pub fn run_step(&mut self) -> Result<GameState, GameProcessError> {
        let old_state = self.state;

        self.state = match self.state {
            GameState::Initial => self.run_initial_state()?,
            GameState::TurnWaiting => self.run_turn_waiting_state()?,
            GameState::BotThinking => self.run_bot_thinking_state()?,
            GameState::TurnMaking => self.run_turn_making_state()?,
            GameState::Finished(_) => self.state,
        };

        println!("game process state {:?} -> {:?}", old_state, self.state);
        Ok(self.state)
    }

    fn run_initial_state(&mut self) -> Result<GameState, GameProcessError> {
        let play_info = self.hooks.init().pack_err()?;
        self.field = Some(Field::new(play_info.field_size));
        self.play_info = Some(play_info);

        Ok(GameState::TurnWaiting)
    }

    fn run_turn_waiting_state(&mut self) -> Result<GameState, GameProcessError> {
        let enemy_turn = self.hooks.wait_for_turn().pack_err()?;

        match enemy_turn {
            None => Ok(GameState::BotThinking),
            Some(enemy_turn) => {
                self.field.as_mut().unwrap().add_turn(&enemy_turn).pack_err()?;

                let next_state = if self.check_win(&enemy_turn) {
                    GameState::Finished(GameResult::Defeat)
                } else if self.is_field_full() {
                    GameState::Finished(GameResult::Draw)
                } else {
                    GameState::BotThinking
                };
        
                Ok(next_state)
            },
        }
    }

    fn run_bot_thinking_state(&mut self) -> Result<GameState, GameProcessError> {
        let best_turn = self
            .bot
            .make_turn(self.field.as_ref().unwrap(), self.play_info.as_ref().unwrap().side)
            .pack_err()?;
        self.bot_prediction = Some(best_turn);

        Ok(GameState::TurnMaking)
    }

    fn run_turn_making_state(&mut self) -> Result<GameState, GameProcessError> {
        let best_turn = self.bot_prediction.as_ref().unwrap();
        self.hooks.make_turn(&best_turn.coords).pack_err()?;
        self.field.as_mut().unwrap().add_turn(best_turn).pack_err()?;

        let next_state = if self.check_win(best_turn) {
            GameState::Finished(GameResult::Win)
        } else if self.is_field_full() {
            GameState::Finished(GameResult::Draw)
        } else {
            GameState::TurnWaiting
        };

        Ok(next_state)
    }

    fn check_win(&self, turn: &Turn) -> bool {
        let field_size = self.play_info.as_ref().unwrap().field_size;
        let win_condition = self.play_info.as_ref().unwrap().win_condition;

        for i in 0..=(field_size.width - win_condition) {
            for j in 0..field_size.height {
                let start = Coords { x: i, y: j };
                let direction = Direction { x: 1, y: 0 };
                if self.check_win_from(turn, start, direction) {
                    return true;
                }
            }
        }

        for i in 0..field_size.width {
            for j in 0..=(field_size.height - win_condition) {
                let start = Coords { x: i, y: j };
                let direction = Direction { x: 0, y: 1 };
                if self.check_win_from(turn, start, direction) {
                    return true;
                }
            }
        }

        for i in 0..=(field_size.width - win_condition) {
            for j in 0..=(field_size.height - win_condition) {
                let start = Coords { x: i, y: j };
                let direction = Direction { x: 1, y: 0 };
                if self.check_win_from(turn, start, direction) {
                    return true;
                }
            }
        }

        false
    }

    fn check_win_from(&self, turn: &Turn, start: Coords, dir: Direction) -> bool {
        let field = self.field.as_ref().unwrap();
        let win_condition = self.play_info.as_ref().unwrap().win_condition;
        let mut in_a_row = 0;
        let mut k = 0;

        loop {
            match field.get(start + dir * k) {
                Some(cell) => match cell {
                    Cell::Value(cell_side) => {
                        if cell_side == turn.side {
                            in_a_row += 1;
                        } else {
                            in_a_row = 0;
                        }
                    }
                    Cell::Empty => in_a_row = 0,
                },
                None => return false,
            }

            if in_a_row >= win_condition {
                return true;
            }

            k += 1;
        }
    }

    fn is_field_full(&self) -> bool {
        let field_size = self.play_info.as_ref().unwrap().field_size;

        self.history.len() >= field_size.width * field_size.height
    }
}
