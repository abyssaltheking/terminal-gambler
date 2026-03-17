pub mod utils {
    use std::process::Command;
    use rand::prelude::*;
    use rand::seq::IteratorRandom;

    pub fn wait(seconds: u32) {
        let mut child = Command::new("sleep").arg(format!("{seconds}")).spawn().unwrap();
        let _result = child.wait().unwrap();
    }

    pub fn random_int_from_range(min: i32, max: i32) -> i32 {
        let mut rng: ThreadRng = rand::rng();
        rng.random_range(min..=max)
    }

    pub fn splash_message() -> String {
        let messages: Vec<String> = vec![
            String::from("welcome to terminal gambler"),
            String::from("let's go gambling!"),
            String::from("99% of gamblers quit before they win big"),
            String::from("i sure hope you brought a lot of money!"),
            String::from("\"wheeeeeeee!\" - slots wheel"),
        ];

        messages.choose(&mut rand::rng()).unwrap().to_string()
    }
}