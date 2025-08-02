use std::str;

use rand::random_range;
use regex::Regex;

pub struct Die {
    hi: u32,
}

impl Die {
    fn new(hi: u32) -> Self {
        Self { hi }
    }

    fn d6() -> Self {
        Self::new(6)
    }

    fn d20() -> Self {
        Self::new(20)
    }

    pub fn roll(&self) -> u32 {
        random_range(1..self.hi)
    }
}

pub fn roll(dice: &mut Vec<Die>) -> Vec<u32> {
    let mut result = Vec::new();

    while !dice.is_empty() {
        match dice.pop() {
            Some(die) => result.push(die.roll()),
            None => break,
        }
    }

    result.sort();

    result
}

pub fn present(values: &Vec<u32>) {
    let str_values = values
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let sum = values.iter().sum::<u32>();

    println!("{} (total: {})", str_values, sum);
}

pub fn parse_dice(input: &str) -> Vec<Die> {
    let mut result = Vec::new();

    let regex = Regex::new(r"(\d*)d(\d+)").unwrap();

    let trimmed = input.trim();

    let captures = regex.captures(trimmed).unwrap();

    let count: u32 = captures[1].to_string().parse().unwrap_or(1);
    let die: u32 = captures[2].to_string().parse().unwrap();

    for i in 0..count {
        match die {
            6 => result.push(Die::d6()),
            20 => result.push(Die::d20()),
            _ => eprintln!("unrecognized die: d{}", die),
        }
    }

    result
}

#[cfg(test)]
mod test {
    mod d6 {
        use crate::dice::Die;

        #[test]
        fn roll() {
            let result = Die::d6().roll();
            assert!(result <= 6);
        }
    }
}
