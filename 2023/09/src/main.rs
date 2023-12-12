use std::io;

fn extract_nums(s: String) -> Vec<i64> {
    s.split_whitespace().filter_map(|n| n.parse::<i64>().ok()).collect()
}

fn build_derivatives(nums: Vec<i64>) -> Vec<Vec<i64>> {
    let mut derivatives = vec![nums];
    while derivatives.last().unwrap().iter().any(|&n| n != 0) {
        let prev = derivatives.last().unwrap();
        let mut cur: Vec<i64> = Vec::new();
        for i in 0..(prev.len() - 1) {
            let a = prev.get(i).unwrap();
            let b = prev.get(i + 1).unwrap();
            cur.push(b - a);
        }
        derivatives.push(cur);
    }
    derivatives
}

fn extrapolate_prev(derivatives: Vec<Vec<i64>>) -> i64 {
    let mut val: i64 = 0;
    for i in (0..(derivatives.len() - 1)).rev() {
        val = derivatives.get(i).unwrap().first().unwrap() - val;
    }
    val
}

fn main() {
    let total: i64 = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(extract_nums)
        .map(build_derivatives)
        .map(extrapolate_prev)
        .sum();
    println!("{total}");
}
