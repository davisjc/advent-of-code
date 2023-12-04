use std::io;
use std::collections::HashSet;
use regex::Regex;

fn numbers_from_string(pattern: &Regex, input: &str) -> HashSet<u32> {
    return pattern.find_iter(input)
                  .filter_map(|m| m.as_str().parse::<u32>().ok()).collect();
}

fn main() {
    let mut total: u64 = 0;
    let line_pattern = Regex::new(r"(Card\s+\d+: )|( \| )").unwrap();
    let number_pattern = Regex::new(r"\d+").unwrap();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
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
        if matches > 0 {
            total += u64::pow(2, matches - 1);
        }
    }
    println!("{total}");
}
