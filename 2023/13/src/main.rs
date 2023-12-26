use std::io;
use std::io::Stdin;

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
}

fn find_reflection_idx<T: PartialEq>(
    v: &Vec<T>, skip: Option<usize>
) -> Option<usize> {
    for mid in 1..v.len() {
        let low = if mid > v.len() / 2 { mid - (v.len() - mid) } else { 0 };
        let high = if mid <= v.len() / 2 { mid * 2 - 1 } else { v.len() - 1 };
        if v[low..mid].iter().eq(v[mid..=high].iter().rev()) {
            if skip.is_some() && skip.unwrap() == mid {
                continue;
            }
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

    fn get_incidence(
        &self, skip_row: Option<usize>, skip_col: Option<usize>
    ) -> (Option<usize>, Option<usize>) {
        (find_reflection_idx(&self.rows, skip_row),
         find_reflection_idx(&self.cols, skip_col))
    }

    fn flip(&mut self, row: usize, col: usize) {
        self.rows[row][col] = !self.rows[row][col];
        self.cols[col][row] = !self.cols[col][row];
    }
}

fn get_next_incidence(
    pattern: &mut Pattern, original_row: Option<usize>,
    original_col: Option<usize>
) -> (Option<usize>, Option<usize>) {
    for row in 0..pattern.rows.len() {
        for col in 0..pattern.cols.len() {
            pattern.flip(row, col);
            let (next_row, next_col) = pattern.get_incidence(original_row,
                                                             original_col);
            pattern.flip(row, col);
            if next_row.is_some()  {
                return (next_row, None);
            }
            if next_col.is_some() {
                return (None, next_col);
            }
        }
    }
    panic!("could not find incidence value in pattern: {pattern:?}");
}

fn main() {
    let mut total = 0;
    let patterns = Pattern::parse(io::stdin());
    for mut pattern in patterns {
        let (original_row, original_col) = pattern.get_incidence(None, None);
        let (next_row, next_col) = get_next_incidence(&mut pattern,
                                                      original_row,
                                                      original_col);
        if let Some(row) = next_row {
            total += 100 * row;
        }
        if let Some(col) = next_col {
            total += col;
        }
    }
    println!("{total}");
}
