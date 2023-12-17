use std::ops::Add;

use super::{
    error::{BotError, CoreError},
    utils::Direction,
};

pub trait Bot {
    fn make_turn(&self, field: &Field, side: Side) -> Result<Turn, BotError> {
        let best_turn = Turn {
            coords: self.calculate_best_turn(field, side)?,
            side,
        };

        Ok(best_turn)
    }

    fn calculate_best_turn(&self, field: &Field, side: Side) -> Result<Coords, BotError>;
}

pub struct Field {
    cells: Vec<Cell>,
    pub size: Size,
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Turn {
    pub coords: Coords,
    pub side: Side,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Cell {
    Value(Side),
    #[default]
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    X,
    O,
}

impl Field {
    pub fn new(size: Size) -> Self {
        Field {
            cells: vec![Cell::Empty; size.width * size.height],
            size,
        }
    }

    pub fn get(&self, coords: Coords) -> Option<Cell> {
        self.geti(coords.x, coords.y)
    }

    pub fn geti(&self, x: usize, y: usize) -> Option<Cell> {
        match x < self.size.width && y < self.size.height {
            true => Some(self.cells[x * self.size.width + y]),
            false => None,
        }
    }

    pub fn add_turn(&mut self, turn: &Turn) -> Result<(), CoreError> {
        match self.get(turn.coords) {
            Some(Cell::Empty) => {
                self.cells[turn.coords.x * self.size.width + turn.coords.y] =
                    Cell::Value(turn.side);
                Ok(())
            }
            Some(Cell::Value(current)) => Err(CoreError::of_filled_cell(turn.coords, current)),
            None => Err(CoreError::of_wrong_coords(turn.coords, self.size)),
        }
    }
}

impl Add<Direction> for Coords {
    type Output = Coords;

    fn add(self, rhs: Direction) -> Self::Output {
        Coords {
            x: add(self.x, rhs.x),
            y: add(self.y, rhs.y),
        }
    }
}

impl Add<Coords> for Coords {
    type Output = Coords;

    fn add(self, rhs: Coords) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}
