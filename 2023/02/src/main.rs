use std::io;
use regex::Regex;

static THRESHOLD_RED: u32 = 12;
static THRESHOLD_GREEN: u32 = 13;
static THRESHOLD_BLUE: u32 = 14;

fn extract_color_amount(pattern: &regex::Regex, round: &str) -> u32 {
    if let Some(caps) = pattern.captures(round) {
        return caps[1].parse::<u32>().unwrap();
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
    'line: for line in io::stdin().lines().map(|l| l.unwrap()) {
        let (_, [game_num, rounds]) = game_pattern.captures(&line).unwrap()
                                                  .extract();
        for round in rounds_pattern.split(&rounds) {
            if extract_color_amount(&red_pattern, &round) > THRESHOLD_RED {
                continue 'line;
            }
            if extract_color_amount(&green_pattern, &round) > THRESHOLD_GREEN {
                continue 'line;
            }
            if extract_color_amount(&blue_pattern, &round) > THRESHOLD_BLUE {
                continue 'line;
            }
        }
        let game_num = game_num.parse::<u64>().unwrap();
        total += game_num;
    }
    println!("{total}");
}
