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
        if player.money < -1_000_000_000 {
            clear().expect("failed to clear");
            println!("good fucking lord you're in serious debt");
            wait(1.0);
            println!("i mean, cmon, you should probably start a new game by now, right?");
            wait(1.0);
            println!("at least you arent spending real money here");
            wait(1.0);
            println!("i mean, think about it? imagine you pulled out a $1 billion loan just to gamble");
            wait(1.0);
            println!("that's what you just did lmao, and now you cant pay it back");
            wait(1.0);
            println!("you're getting reset, please try and not get as bad of debt anymore");
            wait(5.0);
            break;
        }

        let command: String = menu();
        let broken_down_command: Vec<&str> = command.trim().split(' ').collect();

        match broken_down_command[0] {
            "coinflip" => {
                if broken_down_command.len() == 3 {
                    coinflip(&mut player, broken_down_command[1], broken_down_command[2]);
                }

                continue
            },
            "blackjack" => {
                if broken_down_command.len() == 2 {
                    blackjack(&mut player, broken_down_command[1]);
                }

                continue
            },
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