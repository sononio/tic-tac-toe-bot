use std::thread::{self, JoinHandle};

use common::game_process::GameProcess;

use crate::{bots::random_bot::RandomBot, hooks::api_hooks::ApiHooks};

mod api_client;
mod bots;
mod common;
mod hooks;

fn main() {
    let t1 = start_player_thread();
    let t2 = start_player_thread();

    t1.join().unwrap();
    t2.join().unwrap();
}

fn start_player_thread() -> JoinHandle<()> {
    thread::spawn(|| {
        let bot = Box::new(RandomBot {});
        let hooks = Box::new(ApiHooks::new());
        let mut player = GameProcess::new(bot, hooks);
        player.run().unwrap();
    })
}
