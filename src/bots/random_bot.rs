use rand::Rng;

use crate::common::bot::{Bot, Cell, Coords};

pub struct RandomBot {

}

impl Bot for RandomBot {
    fn calculate_best_turn(
        &self,
        field: &crate::common::bot::Field,
        _side: crate::common::bot::Side,
    ) -> Result<crate::common::bot::Coords, crate::common::error::BotError> {
        
        let mut empty_coords = vec![];

        for i in 0..field.size.height {
            for j in 0..field.size.width {
                if let Some(Cell::Empty) = field.geti(i, j) {
                    empty_coords.push(Coords {x: i, y: j});
                }
            }
        }
        let mut rng = rand::thread_rng();
        let selected_index = rng.gen_range(0..empty_coords.len());

        Ok(empty_coords[selected_index])
    }
}
