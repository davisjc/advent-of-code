use std::io;

static DIGIT_WORD_PAIRS: [(u64, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn digit_from_word(candidate: &str) -> Option<u64> {
    for (digit, word) in DIGIT_WORD_PAIRS {
        if candidate.starts_with(word) {
            return Some(digit);
        }
    }
    return None;
}

fn extract_digit(candidate_first: char, candidate: &str) -> Option<u64> {
    if let Some(digit) = candidate_first.to_digit(10) {
        return Some(u64::from(digit));
    }
    if let Some(digit) = digit_from_word(candidate) {
        return Some(digit);
    }
    return None;
}

fn main() {
    let mut total: u64 = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        for (i, c) in line.char_indices() {
            if let Some(digit) = extract_digit(c, &line[i..]) {
                total += 10 * digit;
                break;
            }
        }
        for (i, c) in line.char_indices().rev() {
            if let Some(digit) = extract_digit(c, &line[i..]) {
                total += digit;
                break;
            }
        }
    }
    println!("{total}");
}
