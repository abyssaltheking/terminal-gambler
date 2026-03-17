pub mod app {
    use std::io;

    use clearscreen::clear;

    use crate::utils::utils::{wait, wait_for_input};

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

        wait(5);
        println!("All set! You're ready to start gambling!");
        wait(2);

        Ok(player)
    }

    pub fn menu() -> String {
        clear().expect("failed to clear");
        println!("Enter a command to play!");

        println!("\nAvailable commands:");
        println!("- coinflip [amount] [heads/tails]");
        println!("- blackjack [amount]");
        println!("- slots [amount]");
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
}