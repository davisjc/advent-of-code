use std::io;
use std::iter;
use memoize::memoize;

fn parse_line(line: String) -> (Vec<char>, Vec<usize>) {
    let (springs, groupings) = line.split_once(' ').unwrap();
    let springs = iter::repeat(springs)
        .take(5)
        .collect::<Vec<&str>>()
        .join("?")
        .chars()
        .collect();
    let groupings = iter::repeat(groupings)
        .take(5)
        .collect::<Vec<&str>>()
        .join(",")
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();
    (springs, groupings)
}

#[memoize]
fn count_arrangements(
    springs: Vec<char>, groupings: Vec<usize>, cur_count: usize
) -> usize {
    if springs.is_empty() {
        if groupings.is_empty() {
            return 1;
        }
        if groupings.len() > 1 {
            return 0;
        }
        if cur_count == groupings[0] {
            return 1
        }
        return 0;
    }
    if springs[0] == '.' && cur_count > 0 &&
       cur_count != groupings[0] {
        return 0;
    }
    if springs[0] == '#' && groupings.is_empty() {
        return 0;
    }

    let mut total = 0;
    // try '#'
    if springs[0] == '#' || springs[0] == '?' && !groupings.is_empty() &&
       cur_count < groupings[0] {
        total += count_arrangements(springs[1..].to_vec(), groupings.clone(),
                                    cur_count + 1);
    }
    // try '.'
    if springs[0] == '.' || springs[0] == '?' {
        if cur_count > 0 && !groupings.is_empty() && cur_count == groupings[0] {
            total += count_arrangements(springs[1..].to_vec(),
                                        groupings[1..].to_vec(), 0);
        }
        if cur_count == 0 {
            total += count_arrangements(springs[1..].to_vec(),
                                        groupings.clone(), 0);
        }
    }
    return total;
}

fn main() {
    let mut total = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let (springs, groupings) = parse_line(line);
        total += count_arrangements(springs, groupings, 0);
    }
    println!("{total}");
}
