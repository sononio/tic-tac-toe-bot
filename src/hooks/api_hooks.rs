use crate::{
    api_client::{
        dto::{CoordsDto, RegisterInfoDto, SideDto, StateDto, TurnDto},
        methods::{get_player_game, get_players, post_game_turn, post_mathchmaking_queue},
    },
    common::{
        bot::{Coords, Side, Size, Turn},
        error::HookError,
        game_process::PlayInfo,
        hooks::Hooks,
    },
};

static POLLING_TIMEOUT: usize = 3000;

pub struct ApiHooks {
    player_id: Option<String>,
    side: Option<Side>,
}

impl Hooks for ApiHooks {
    fn init(&mut self) -> Result<PlayInfo, HookError> {
        self.register()?;
        self.wait_for_game(true)?;
        let game_dto = self.get_game(false)?;

        Ok(PlayInfo {
            field_size: Size {
                width: game_dto.settings.width,
                height: game_dto.settings.height,
            },
            win_condition: game_dto.settings.win_condition,
            side: self.side.unwrap(),
        })
    }

    fn wait_for_turn(&mut self) -> Result<Option<Turn>, HookError> {
        let game_dto = self.wait_for_turn()?;
        Ok(game_dto.last_turn.map(|dto| dto.to_entity()))
    }

    fn make_turn(&mut self, coords: &Coords) -> Result<(), HookError> {
        self.make_turn(coords)
    }
}

impl ApiHooks {
    pub fn new() -> Self {
        ApiHooks {
            player_id: None,
            side: None,
        }
    }

    fn register(&mut self) -> Result<(), HookError> {
        let player_dto = post_mathchmaking_queue(RegisterInfoDto {
            name: "test_bot".to_owned(),
        })
        .map_err(|e| HookError {
            message: e.to_string(),
        })?;
        println!("player registered: {:?}", player_dto.id);
        self.player_id = Some(player_dto.id);
        Ok(())
    }

    fn wait_for_game(&mut self, polling: bool) -> Result<(), HookError> {
        loop {
            println!("waiting for game...");
            let player_dto =
                get_players(self.player_id.as_ref().unwrap(), polling, POLLING_TIMEOUT).map_err(
                    |e| HookError {
                        message: e.to_string(),
                    },
                )?;

            if let Some(side) = player_dto.side {
                self.side = Some(side.to_entity());
                break;
            }
        }

        println!("game found!");
        println!("side: {:#?}", self.side.unwrap());
        Ok(())
    }

    fn wait_for_turn(&mut self) -> Result<crate::api_client::dto::GameDto, HookError> {
        loop {
            println!("waiting for turn...");
            let game_dto = self.get_game(true).unwrap();

            if (game_dto.state == StateDto::Finished)
                || (game_dto.state == to_state_dto(self.side.unwrap()))
            {
                println!("its our turn now!");
                return Ok(game_dto);
            }
        }
    }

    fn get_game(&mut self, polling: bool) -> Result<crate::api_client::dto::GameDto, HookError> {
        let game_dto = get_player_game(self.player_id.as_ref().unwrap(), polling, POLLING_TIMEOUT)
            .map_err(|e| HookError {
                message: e.to_string(),
            })?;

        // println!("game received!");
        // println!("game: {game_dto:#?}");
        Ok(game_dto)
    }

    fn make_turn(&mut self, coords: &Coords) -> Result<(), HookError> {
        post_game_turn(self.player_id.as_ref().unwrap(), coords.to_dto()).map_err(|e| {
            HookError {
                message: e.to_string(),
            }
        })?;

        println!("turn maked: {coords:?}");
        Ok(())
    }
}

trait ToDto<Dto> {
    fn to_dto(&self) -> Dto;
}

trait ToEntity<Entity> {
    fn to_entity(&self) -> Entity;
}

impl ToEntity<Side> for SideDto {
    fn to_entity(&self) -> Side {
        match self {
            SideDto::X => Side::X,
            SideDto::O => Side::O,
        }
    }
}

impl ToDto<SideDto> for Side {
    fn to_dto(&self) -> SideDto {
        match self {
            Side::X => SideDto::X,
            Side::O => SideDto::O,
        }
    }
}

impl ToEntity<Coords> for CoordsDto {
    fn to_entity(&self) -> Coords {
        Coords {
            x: self.x,
            y: self.y,
        }
    }
}

impl ToDto<CoordsDto> for Coords {
    fn to_dto(&self) -> CoordsDto {
        CoordsDto {
            x: self.x,
            y: self.y,
        }
    }
}

impl ToEntity<Turn> for TurnDto {
    fn to_entity(&self) -> Turn {
        Turn {
            coords: self.coords.to_entity(),
            side: self.side.to_entity(),
        }
    }
}

impl ToDto<TurnDto> for Turn {
    fn to_dto(&self) -> TurnDto {
        TurnDto {
            coords: self.coords.to_dto(),
            side: self.side.to_dto(),
        }
    }
}

fn to_state_dto(side: Side) -> StateDto {
    match side {
        Side::X => StateDto::XMove,
        Side::O => StateDto::OMove,
    }
}
