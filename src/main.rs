use clearscreen::clear;
use crate::utils::utils::{wait, splash_message};
use crate::app::app::{blackjack, coinflip, init, menu, slots, stats};

mod utils;
mod app;

fn main() {
    clear().expect("failed to clear");
    println!("{}", splash_message());
    wait(3.0);
    let mut player = init().unwrap();
    clear().expect("failed to clear");

    loop {
        let command: String = menu();
        let broken_down_command: Vec<&str> = command.trim().split(' ').collect();

        match broken_down_command[0] {
            "coinflip" => {
                if broken_down_command.len() == 3 {
                    coinflip(&mut player, broken_down_command[1], broken_down_command[2]);
                }

                continue
            },
            "blackjack" => blackjack(),
            "slots" => {
                if broken_down_command.len() == 2 {
                    slots(&mut player, broken_down_command[1]);
                }
                
                continue
            },
            "stats" => stats(&player),
            "quit" => break,
            _ => continue
        }
    }
}