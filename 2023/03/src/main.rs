use std::io;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let number_pattern = Regex::new(r"\d+").unwrap();
    let gear_pattern = Regex::new(r"\*").unwrap();
    let mut nums_by_gear_pos = HashMap::new();
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
                candidates.push((row - 1, search_start_col, above));
            }
            if start_col > 0 {
                let left = &line[search_start_col..=search_start_col];
                candidates.push((row, search_start_col, left));
            }
            if end_col + 1 < line.len() {
                let right = &line[search_end_col..=search_end_col];
                candidates.push((row, search_end_col, right));
            }
            if row + 1 < lines.len() {
                let below = &lines[row + 1][search_start_col..=search_end_col];
                candidates.push((row + 1, search_start_col, below));
            }
            for (gear_row, gear_start_col, candidate) in candidates.iter() {
                for gear_match in gear_pattern.find_iter(&candidate) {
                    let gear_row = *gear_row;
                    let gear_col = *gear_start_col + gear_match.start();
                    let gear_pos = (gear_row, gear_col);
                    let num = num_match.as_str().parse::<u32>().unwrap();
                    if !nums_by_gear_pos.contains_key(&gear_pos) {
                        nums_by_gear_pos.insert(gear_pos, Vec::new());
                    }
                    let nums = nums_by_gear_pos.get_mut(&gear_pos).unwrap();
                    nums.push(num);
                }
            }
        }
    }
    let mut total: u64 = 0;
    for (_, nums) in &nums_by_gear_pos {
        if nums.len() != 2 {
            continue;
        }
        total += u64::from(nums[0] * nums[1]);
    }
    println!("{total}");
}
