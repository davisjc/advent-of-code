use std::io;
use std::cmp::*;
use std::collections::HashSet;

fn main() {
    let mut occupied_rows: HashSet<usize> = HashSet::new();
    let mut occupied_cols: HashSet<usize> = HashSet::new();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (row, line) in io::stdin().lines().map(|l| l.unwrap()).enumerate() {
        for (col, c) in line.char_indices() {
            if c != '#' {
                continue;
            }
            occupied_rows.insert(row);
            occupied_cols.insert(col);
            galaxies.push((row, col));
        }
    }

    let mut total: usize = 0;
    for (i, &a) in galaxies.iter().enumerate() {
        for &b in &galaxies[(i + 1)..] {
            let (a_row, a_col) = a;
            let (b_row, b_col) = b;
            total += a_row.abs_diff(b_row);
            total += a_col.abs_diff(b_col);
            let intervening_rows = (min(a_row, b_row) + 1)..max(a_row, b_row);
            let intervening_cols = (min(a_col, b_col) + 1)..max(a_col, b_col);
            total += intervening_rows
                .filter(|row| !occupied_rows.contains(row))
                .fold(0, |total, _| total + 999999);
            total += intervening_cols
                .filter(|col| !occupied_cols.contains(col))
                .fold(0, |total, _| total + 999999);
        }
    }
    println!("{total}");
}
