use std::io;

struct Race {
    duration_ms: i64,
    record_mm: i64,
}

fn extract_nums(line: String) -> Vec<i64> {
    line.split_whitespace().filter_map(|n| n.parse::<i64>().ok()).collect()
}

fn count_solutions(race: Race) -> i64 {
    let mut count = 0;
    for held_ms in 1..race.duration_ms {
        let travel_mm_ms = held_ms;
        let travel_ms = race.duration_ms - held_ms;
        let travel_mm = travel_mm_ms * travel_ms;
        if travel_mm > race.record_mm {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut lines_iter = io::stdin().lines();
    let times = extract_nums(lines_iter.next().unwrap().unwrap());
    let distances = extract_nums(lines_iter.next().unwrap().unwrap());
    let total: i64 = times.iter().zip(distances.iter())
        .map(|(&duration_ms, &record_mm)| Race { duration_ms, record_mm })
        .map(count_solutions)
        .product();
    println!("{total}");
}
