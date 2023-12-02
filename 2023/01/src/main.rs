use std::io;

fn extract_digit(c: char) -> Option<u64> {
    if let Some(digit) = c.to_digit(10) {
        return Some(u64::from(digit));
    }
    return None;
}

fn main() {
    let mut total: u64 = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        for c in line.chars() {
            if let Some(digit) = extract_digit(c) {
                total += 10 * digit;
                break;
            }
        }
        for c in line.chars().rev() {
            if let Some(digit) = extract_digit(c) {
                total += digit;
                break;
            }
        }
    }
    println!("{total}");
}
