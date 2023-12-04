use std::io;
use regex::Regex;

fn main() {
    let mut total: u64 = 0;
    let number_pattern = Regex::new(r"\d+").unwrap();
    let symbol_pattern = Regex::new(r"[^\d\.]").unwrap();
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    for (row, line) in lines.iter().enumerate() {
        for num_match in number_pattern.find_iter(&line) {
            let start_col = num_match.start();
            let end_col = num_match.end() - 1;
            let search_start_col = if start_col > 0 {
                start_col - 1
            } else {
                start_col
            };
            let search_end_col = if end_col + 1 < line.len() {
                end_col + 1
            } else {
                end_col
            };

            let mut candidates = Vec::new();
            if row > 0 {
                let above = &lines[row - 1][search_start_col..=search_end_col];
                candidates.push(above);
            }
            if start_col > 0 {
                let left = &line[search_start_col..=search_start_col];
                candidates.push(left);
            }
            if end_col + 1 < line.len() {
                let right = &line[search_end_col..=search_end_col];
                candidates.push(right);
            }
            if row + 1 < lines.len() {
                let below = &lines[row + 1][search_start_col..=search_end_col];
                candidates.push(below);
            }
            for candidate in candidates.iter() {
                if let Some(_) = symbol_pattern.find(&candidate) {
                    let num = num_match.as_str().parse::<u64>().unwrap();
                    total += num;
                    break;
                }
            }
        }
    }
    println!("{total}");
}
