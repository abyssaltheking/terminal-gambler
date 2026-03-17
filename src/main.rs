use clearscreen::clear;
use crate::utils::utils::{wait, splash_message};
use crate::app::app::{init};

mod utils;
mod app;

fn main() {
    clear().expect("failure to clear screen");
    println!("{}", splash_message());
    wait(3);
    let player = init();
}