use std::io;
use std::io::{Lines, StdinLock};
use std::collections::HashMap;
use num;

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

    fn reset(&mut self) {
        self.pos = 0;
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

fn count_steps(
    directions_by_node: &HashMap<String, (String, String)>, turns: &mut Turns,
    start: &str
) -> u64 {
    turns.reset();
    let mut steps: u64 = 0;
    let mut cur = start;
    while !cur.ends_with('Z') {
        let (left, right) = directions_by_node.get(cur).unwrap();
        let next_turn = turns.next().unwrap();
        cur = match next_turn {
            'L' => left,
            'R' => right,
            _ => panic!("unexpected turn symbol: {next_turn}")
        };
        steps += 1;
    }
    steps
}

fn main() {
    let mut lines_iter = io::stdin().lines();
    let mut turns = Turns::new(&lines_iter.next().unwrap().unwrap());
    lines_iter.next(); // skip blank line
    let directions_by_node = map_from_input(&mut lines_iter);
    let steps = directions_by_node
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|start| count_steps(&directions_by_node, &mut turns, start))
        .fold(1, num::integer::lcm);
    println!("{steps}");
}
