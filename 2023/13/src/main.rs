use std::io;
use std::io::Stdin;

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
}

fn find_reflection_idx<T: PartialEq>(v: &Vec<T>) -> Option<usize> {
    for mid in 1..v.len() {
        let low = if mid > v.len() / 2 { mid - (v.len() - mid) } else { 0 };
        let high = if mid <= v.len() / 2 { mid * 2 - 1 } else { v.len() - 1 };
        if v[low..mid].iter().eq(v[mid..=high].iter().rev()) {
            return Some(mid);
        }
    }
    None
}

impl Pattern {
    fn new(rows: Vec<Vec<bool>>) -> Pattern {
        let mut cols = vec![vec![]; rows[0].len()];
        for row in rows.iter() {
            for (col, &c) in row.iter().enumerate() {
                cols[col].push(c);
            }
        }
        Pattern { rows, cols }
    }

    fn parse(input: Stdin) -> Vec<Pattern> {
        let mut patterns = vec![];
        let mut rows: Vec<Vec<bool>> = vec![];
        for line in input.lines().map(|l| l.unwrap()) {
            if line.is_empty() {
                patterns.push(Pattern::new(rows));
                rows = vec![];
                continue;
            }
            rows.push(line.chars().map(|c| c == '#').collect());
        }
        if !rows.is_empty() {
            patterns.push(Pattern::new(rows));
        }
        patterns
    }

    fn get_incidence(&self) -> Option<usize> {
        let mut total = None;
        if let Some(idx) = find_reflection_idx(&self.cols) {
            total = Some(total.unwrap_or_default() + idx);
        }
        if let Some(idx) = find_reflection_idx(&self.rows) {
            total = Some(total.unwrap_or_default() + 100 * idx);
        }
        total
    }
}

fn main() {
    let mut total = 0;
    let patterns = Pattern::parse(io::stdin());
    for pattern in patterns {
        total += pattern.get_incidence().unwrap();
    }
    println!("{total}");
}
