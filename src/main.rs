use std::io::{self, Write};

use crate::dice::{parse_dice, present, roll};

pub mod dice;

fn process_input(input: &str) {
    let mut dice = parse_dice(input);

   let values = roll(&mut dice);

   present(&values);
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let input = io::stdin().read_line(&mut buffer);

        match input {
            Ok(_) => process_input(&buffer),
            Err(e) => eprintln!("{}", e),
        }
    }
}
