use std::io;
use std::io::{Lines, StdinLock};
use std::collections::HashMap;

#[derive(Debug)]
struct Turns {
    data: Vec<char>,
    pos: usize,
}

impl Turns {
    fn new(line: &str) -> Turns {
        let data = line.chars().collect();
        Turns { data, pos: 0 }
    }
}

impl Iterator for Turns {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let ret = self.data.get(self.pos).unwrap();
        self.pos = (self.pos + 1) % self.data.len();
        Some(*ret)
    }
}

fn map_from_input(
    lines_iter: &mut Lines<StdinLock<>>
) -> HashMap<String, (String, String)> {
    lines_iter.map(|l| {
        let line = l.unwrap();
        let node = String::from(&line[..3]);
        let left = String::from(&line[7..10]);
        let right = String::from(&line[12..15]);
        (node, (left, right))
    }).collect()
}

fn main() {
    let mut lines_iter = io::stdin().lines();
    let mut turns = Turns::new(&lines_iter.next().unwrap().unwrap());
    lines_iter.next(); // skip blank line
    let directions_by_node = map_from_input(&mut lines_iter);
    let mut steps: u64 = 0;
    let mut cur = "AAA";
    while cur != "ZZZ" {
        let (left, right) = directions_by_node.get(cur).unwrap();
        let next_turn = turns.next().unwrap();
        cur = match next_turn {
            'L' => left,
            'R' => right,
            _ => panic!("unexpected turn symbol: {next_turn}")
        };
        steps += 1;
    }
    println!("{steps}");
}
