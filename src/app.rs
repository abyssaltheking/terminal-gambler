pub mod app {
    use std::io;

    use clearscreen::clear;

    use crate::utils::utils::wait;
    pub struct Player {
        name: String,
        money: i32
    }

    pub fn init() -> io::Result<Player> {
        clear();
        println!("Welcome to TERMINAL GAMBLER!\n");
        println!("Please enter your name:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
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
}