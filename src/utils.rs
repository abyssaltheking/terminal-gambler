pub mod utils {
    use std::io;
    use std::thread::sleep;
    use std::time::Duration;
    use rand::prelude::*;

    pub fn wait(seconds: f32) {
        sleep(Duration::from_secs_f32(seconds));
    }

    pub fn random_int_from_range(min: i32, max: i32) -> i32 {
        let mut rng: ThreadRng = rand::rng();
        rng.random_range(min..=max)
    }

    pub fn wait_for_input() -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(..) => input,
            Err(error) => panic!("error: {error}")
        }
    }

    pub fn splash_message() -> String {
        let messages: Vec<String> = vec![
            String::from("the ultimate casino experience"),
            String::from("let's go gambling!"),
            String::from("99% of gamblers quit before they win big"),
            String::from("i sure hope you brought a lot of money!"),
            String::from("\"wheeeeeeee!\" - slots wheel"),
        ];

        messages.choose(&mut rand::rng()).unwrap().to_string()
    }
}