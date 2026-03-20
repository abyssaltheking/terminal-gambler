pub mod app {
    use std::{io, vec};

    use clearscreen::clear;

    use crate::utils::utils::{wait, wait_for_input, random_int_from_range};

    pub struct Player {
        name: String,
        pub(crate) money: i32,
        wins: u32,
        biggest_win: i32,
        losses: u32,
        biggest_loss: i32
    }

    pub fn init() -> io::Result<Player> {
        clear().expect("failed to clear");
        println!("Welcome to TERMINAL GAMBLER!\n");
        println!("Please enter your name: ");

        let input = wait_for_input();
        println!("\nWelcome to the ultimate casino, {}", &input);
        println!("Give us just one moment to set you up, you're almost ready to start gambling!");
        
        let player: Player = Player { 
            name: input, 
            money: 1000,
            wins: 0,
            biggest_win: 0,
            losses: 0,
            biggest_loss: 0
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
        println!("- coinflip [bet, max 1k] [heads/tails]");
        println!("- blackjack [bet, max 10k]");
        println!("- slots [bet, max 100k]");
        println!("- stats");
        println!("- quit\n");

        let command = wait_for_input();
        command
    }

    pub fn stats(player: &Player) {
        clear().expect("failed to clear");
        println!("{}", player.name);
        println!("Money: ${}", player.money);
        println!("Wins: {}", player.wins);
        println!("Biggest Win: ${}", player.biggest_win);
        println!("Losses: {}", player.losses);
        println!("Biggest Loss: ${}", player.biggest_loss);

        println!("\nPress enter to return back to the menu.");
        wait_for_input();
    }

    pub fn coinflip(player: &mut Player, bet_raw: &str, heads_or_tails_raw: &str) {
        let bet: i32 = bet_raw.parse::<i32>().unwrap_or(0);

        if &bet < &1 || &bet > &1000 {
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
            println!("Congratulations! You won ${}", &bet);
            player.wins += 1;
            player.money += bet;

            if &bet > &player.biggest_win {
                player.biggest_win = bet
            }
        } else {
            println!("Not this time! Keep playing to win!");
            player.losses += 1;
            player.money -= bet;
            
            if &bet > &player.biggest_loss {
                player.biggest_loss = bet
            }
        }

        wait(2.0);
    }

    pub fn blackjack(player: &mut Player, bet_raw: &str) {
        let bet: i32 = bet_raw.parse::<i32>().unwrap_or(0);

        if &bet < &1 || &bet > &10000 {
            return
        }

        let mut player_cards: Vec<i32> = vec![random_int_from_range(1, 10), random_int_from_range(1, 10)];
        let dealer_cards: Vec<i32> = vec![random_int_from_range(1, 10), random_int_from_range(1, 10)];

        loop {
            clear().expect("failed to clear");
            println!("Your cards:");
            for card in &player_cards {
                print!("{}   ", card);
            }
            println!("\nDealer's cards:");
            println!("?   {}\n", dealer_cards[1]);
            println!("Enter either \"hit\" for another card or \"stand\" to reveal the dealer's other card and end the game.");

            let input = wait_for_input();

            match input.trim() {
                "hit" => player_cards.push(random_int_from_range(1, 10)),
                "stand" => break,
                _ => continue
            }
        }

        clear().expect("failed to clear");
        println!("Your cards:");
        for card in &player_cards {
            print!("{}   ", card);
        }
        println!("\nDealer's cards:");
        println!("{}   {}\n", dealer_cards[0], dealer_cards[1]);
        wait(1.0);

        let mut player_cards_value: i32 = 0;

        for card in &player_cards {
            player_cards_value += card;
        }

        let dealer_cards_value: i32 = dealer_cards[0] + dealer_cards[1];

        println!("Your card value: {}", player_cards_value);
        wait(1.0);
        println!("Dealer's card value: {}", dealer_cards_value);
        wait(2.5);

        clear().expect("failed to clear");
        if player_cards_value < dealer_cards_value || player_cards_value > 20 {
            println!("Not this time! Keep playing to win!");
            player.losses += 1;
            player.money -= &bet;
            
            if &bet > &player.biggest_loss {
                player.biggest_loss = bet
            }
        } else if player_cards_value == dealer_cards_value {
            println!("You tied! Neither a win or a loss.");
        } else {
            println!("Congratulations! You won ${}", &bet * 2);
            player.wins += 1;
            player.money += bet * 2;

            if &bet > &player.biggest_win {
                player.biggest_win = bet
            }
        }

        wait(2.0);
    }

    pub fn slots(player: &mut Player, bet_raw: &str) {
        clear().expect("failed to clear");
        let bet: i32 = bet_raw.parse::<i32>().unwrap_or(0);

        if &bet < &1 || &bet > &100000 {
            return
        }

        let mut slots_result: Vec<i32> = vec![];
        for _i in 0..9 {
            slots_result.push(random_int_from_range(2, 7));
        }

        let top_row_win: bool = (slots_result[0] == slots_result[1]) && (slots_result[1] == slots_result[2]);
        let middle_row_win: bool = (slots_result[3] == slots_result[4]) && (slots_result[4] == slots_result[5]);
        let bottom_row_win: bool = (slots_result[6] == slots_result[7]) && (slots_result[7] == slots_result[8]);
        
        let did_win: bool = top_row_win || middle_row_win || bottom_row_win;

        let top_row_value: i32 = if top_row_win { slots_result[1] } else { 0 };
        let middle_row_value: i32 = if middle_row_win { slots_result[4] } else { 0 };
        let bottom_row_value: i32 = if bottom_row_win { slots_result[7] } else { 0 };

        let winnings: i32 = if did_win { 
            (bet as f32 * ((top_row_value + middle_row_value + bottom_row_value) as f32) as f32).round() as i32 
        } else { 0 };

        let mut timer: f32 = 0.0;
        while timer < 4.0 {
            clear().expect("failed to clear");
            timer += 0.05;

            let mut fake_slots = vec![];
            for _i in 0..9 {
                fake_slots.push(random_int_from_range(2, 7));
            }

            println!("{}   {}   {}\n", fake_slots[0], fake_slots[1], fake_slots[2]);
            println!("{}   {}   {}\n", fake_slots[3], fake_slots[4], fake_slots[5]);
            println!("{}   {}   {}", fake_slots[6], fake_slots[7], fake_slots[8]);
            wait(0.05);
        }
        clear().expect("failed to clear");

        println!("{}   {}   {}\n", slots_result[0], slots_result[1], slots_result[2]);
        println!("{}   {}   {}\n", slots_result[3], slots_result[4], slots_result[5]);
        println!("{}   {}   {}", slots_result[6], slots_result[7], slots_result[8]);

        wait(2.0);
        clear().expect("failed to clear");

        if did_win {
            println!("Congratulations! You won ${}", &winnings);
            player.wins += 1;
            player.money += winnings;

            if &winnings > &player.biggest_win {
                player.biggest_win = winnings
            }
        } else {
            println!("Not this time! Keep playing to win!");
            player.losses += 1;
            player.money -= &bet;
            
            if &bet > &player.biggest_loss {
                player.biggest_loss = bet
            }
        }

        wait(3.0);
    }
}