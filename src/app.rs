pub mod app {
    use std::{io, num::ParseIntError};

    use clearscreen::clear;

    use crate::utils::utils::{wait, wait_for_input, random_int_from_range};

    pub struct Player {
        name: String,
        money: i32
    }

    pub fn init() -> io::Result<Player> {
        clear();
        println!("Welcome to TERMINAL GAMBLER!\n");
        println!("Please enter your name: ");

        let input = wait_for_input();
        println!("\nWelcome to the ultimate casino, {}", &input);
        println!("Give us just one moment to set you up, you're almost ready to start gambling!");
        
        let player: Player = Player { 
            name: input, 
            money: 1000 
        };

        wait(3.0);
        println!("All set! You're ready to start gambling!");
        wait(1.5);

        Ok(player)
    }

    pub fn menu() -> String {
        clear().expect("failed to clear");
        println!("Enter a command to play!");

        println!("\nAvailable commands:");
        println!("- coinflip [bet, max 1000] [heads/tails]");
        println!("- blackjack [bet]");
        println!("- slots [bet]");
        println!("- stats");
        println!("- quit\n");

        let command = wait_for_input();
        command
    }

    pub fn stats(player: &Player) {
        clear().expect("failed to clear");
        println!("{}", player.name);
        println!("Money: {}", player.money);

        println!("\nPress enter to return back to the menu.");
        wait_for_input();
    }

    pub fn coinflip(player: &mut Player, amount_raw: &str, heads_or_tails_raw: &str) {
        let amount: i32 = amount_raw.parse::<i32>().unwrap_or(0);

        if &amount < &1 || &amount > &1000 {
            return
        }

        let chose_heads: bool = match heads_or_tails_raw {
            "heads" => true,
            "tails" => false,
            _ => return
        };

        let result: bool = random_int_from_range(0, 1) == 0;
        let mut coin: bool = random_int_from_range(0, 1) == 0;
        let mut timer: f32 = 0.0;

        while timer < 2.0 {
            clear().expect("failed to clear");
            timer += 0.05;

            match coin {
                true => println!("heads"),
                false => println!("tails")
            }

            coin = !coin;
            wait(0.05);
        }

        clear().expect("failed to clear");

        match result {
            true => println!("heads"),
            false => println!("tails")
        }

        wait(1.5);

        if result == chose_heads {
            println!("Congratulations! You won ${}", &amount);
            player.money += amount;
        } else {
            println!("Not this time! Keep playing to win!");
            player.money -= amount;
        }

        wait(2.0);
    }

    fn blackjack() {

    }

    fn slots(player: &mut Player, amount_raw: &str) {

    }
}