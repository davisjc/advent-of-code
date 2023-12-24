use std::io;

fn parse_line(line: String) -> (Vec<char>, Vec<usize>) {
    let (springs, groupings) = line.split_once(' ').unwrap();
    let groupings = groupings
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();
    (springs.chars().collect(), groupings)
}

fn count_arrangements(
    springs: &[char], groupings: &[usize], cur_count: usize
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
    if springs[0] == '#' || springs[0] == '?' && !groupings.is_empty() {
        total += count_arrangements(&springs[1..], groupings, cur_count + 1);
    }
    // try '.'
    if springs[0] == '.' || springs[0] == '?' {
        if cur_count > 0 && !groupings.is_empty() && cur_count == groupings[0] {
            total += count_arrangements(&springs[1..], &groupings[1..], 0);
        }
        if cur_count == 0 {
            total += count_arrangements(&springs[1..], groupings, 0);
        }
    }
    return total;
}

fn main() {
    let mut total = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let (springs, groupings) = parse_line(line);
        total += count_arrangements(&springs[..], &groupings[..], 0);
    }
    println!("{total}");
}
