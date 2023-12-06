use std::io;
use std::cmp;

enum ParseState {
    ReadSeeds,
    PrepareNextMap,
    ReadMapName,
    ProcessMap,
}

struct Mapping {
    start: i64,
    end: i64,
    offset: i64,
}

impl Mapping {
    fn resolve(&self, val: i64) -> Option<i64> {
        if self.start <= val && val <= self.end {
            Some(val + self.offset)
        } else {
            None
        }
    }
}

fn extract_nums(s: &str) -> Vec<i64> {
    s.split_whitespace().filter_map(|n| n.parse::<i64>().ok()).collect()
}

fn extract_seeds(line: &str) -> Vec<i64> {

    extract_nums(&line[7..])
}

fn extract_mapping(line: &str) -> Mapping {
    if let [dst_start, src_start, len] = extract_nums(line)[..] {
        Mapping {
            start: src_start,
            end: src_start + len - 1,
            offset: dst_start - src_start,
        }
    } else {
        panic!("invalid map format");
    }
}

fn main() {
    let mut state = ParseState::ReadSeeds;
    let mut seeds = None;
    let mut transformations: Vec<Vec<Mapping>> = Vec::new();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        match state {
            ParseState::ReadSeeds => {
                seeds = Some(extract_seeds(&line));
                state = ParseState::PrepareNextMap;
            },
            ParseState::PrepareNextMap => {
                state = ParseState::ReadMapName;
            },
            ParseState::ReadMapName => {
                transformations.push(Vec::new());
                state = ParseState::ProcessMap;
            },
            ParseState::ProcessMap => {
                if line.is_empty() {
                    state = ParseState::ReadMapName;
                    continue;
                }
                let transformation = transformations.last_mut().unwrap();
                transformation.push(extract_mapping(&line));
            },
        }
    }
    let mut lowest_location = i64::MAX;
    for seed in seeds.unwrap().into_iter() {
        let mut val = seed;
        for transformation in transformations.iter() {
            for mapping in transformation.iter() {
                if let Some(resolved) = mapping.resolve(val) {
                    val = resolved;
                    break;
                }
            }
        }
        lowest_location = cmp::min(val, lowest_location);
    }
    println!("{lowest_location}");
}
