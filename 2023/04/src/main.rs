use std::io;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn numbers_from_string(pattern: &Regex, input: &str) -> HashSet<u32> {
    return pattern.find_iter(input)
                  .filter_map(|m| m.as_str().parse::<u32>().ok()).collect();
}

fn increment_extras(extras_by_card: &mut HashMap<u32, u64>, card: u32) {
    if !extras_by_card.contains_key(&card) {
        extras_by_card.insert(card, 0);
    }
    let extras = extras_by_card.get_mut(&card).unwrap();
    *extras += 1;
}

fn main() {
    let line_pattern = Regex::new(r"(Card\s+\d+: )|( \| )").unwrap();
    let number_pattern = Regex::new(r"\d+").unwrap();

    let mut total: u64 = 0;
    let mut extras_by_card: HashMap<u32, u64> = HashMap::new();
    let mut cur_card: u32 = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        cur_card += 1;
        total += 1;
        let mut tokens = line_pattern.split(&line);
        tokens.next(); // trash
        let winning_numbers = numbers_from_string(&number_pattern,
                                                  tokens.next().unwrap());
        let our_numbers = numbers_from_string(&number_pattern,
                                              tokens.next().unwrap());
        let mut matches: u32 = 0;
        for _ in winning_numbers.intersection(&our_numbers) {
            matches += 1;
        }
        let times = if let Some(extras) = extras_by_card.get(&cur_card) {
            1 + extras
        } else {
            1
        };
        for _ in 0..times {
            for card in (cur_card + 1)..(cur_card + 1 + matches) {
                increment_extras(&mut extras_by_card, card);
            }
        }
    }
    let last_card = cur_card;
    for card in 1..=last_card {
        if let Some(extras) = extras_by_card.get(&card) {
            total += extras;
        }
    }
    println!("{total}");
}
