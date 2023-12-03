use std::io;
use std::cmp;
use regex::Regex;

fn extract_color_amount(pattern: &regex::Regex, round: &str) -> u64 {
    if let Some(caps) = pattern.captures(round) {
        return caps[1].parse::<u64>().unwrap();
    }
    return 0;
}

fn main() {
    let mut total: u64 = 0;
    let game_pattern = Regex::new(r"Game (\d+): (.*)").unwrap();
    let rounds_pattern = Regex::new(r"; ").unwrap();
    let red_pattern = Regex::new(r"(\d+) red").unwrap();
    let green_pattern = Regex::new(r"(\d+) green").unwrap();
    let blue_pattern = Regex::new(r"(\d+) blue").unwrap();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let (_, [_, rounds]) = game_pattern.captures(&line).unwrap().extract();
        let mut r: u64 = 0;
        let mut g: u64 = 0;
        let mut b: u64 = 0;
        for round in rounds_pattern.split(&rounds) {
            r = cmp::max(extract_color_amount(&red_pattern, &round), r);
            g = cmp::max(extract_color_amount(&green_pattern, &round), g);
            b = cmp::max(extract_color_amount(&blue_pattern, &round), b);
        }
        total += r * g * b;
    }
    println!("{total}");
}
