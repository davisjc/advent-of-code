use std::io;
use std::io::Stdin;
use std::collections::HashSet;
use PipeShape::*;
use Location::*;

#[derive(PartialEq, Eq, Copy, Clone)]
enum PipeShape {
    Vertical,
    Horizontal,
    ElbowNorthEast,
    ElbowNorthWest,
    ElbowSouthEast,
    ElbowSouthWest,
}

impl PipeShape {
    fn infer_from_neighbors(
        grid: &Vec<Vec<Location>>, row: i64, col: i64
    ) -> PipeShape {
        if let (Some(north), Some(south)) =
               (Location::lookup(grid, row - 1, col),
                Location::lookup(grid, row + 1, col)) {
            if north.connects_south() && south.connects_north() {
                return Vertical;
            }
        }
        if let (Some(east), Some(west)) =
               (Location::lookup(grid, row, col + 1),
                Location::lookup(grid, row, col - 1)) {
            if east.connects_west() && west.connects_east() {
                return Horizontal;
            }
        }
        if let (Some(north), Some(east)) =
               (Location::lookup(grid, row - 1, col),
                Location::lookup(grid, row, col + 1)) {
            if north.connects_south() && east.connects_west() {
                return ElbowNorthEast;
            }
        }
        if let (Some(north), Some(west)) =
               (Location::lookup(grid, row - 1, col),
                Location::lookup(grid, row, col - 1)) {
            if north.connects_south() && west.connects_east() {
                return ElbowNorthWest;
            }
        }
        if let (Some(south), Some(east)) =
               (Location::lookup(grid, row + 1, col),
                Location::lookup(grid, row, col + 1)) {
            if south.connects_north() && east.connects_west() {
                return ElbowSouthEast;
            }
        }
        if let (Some(south), Some(west)) =
               (Location::lookup(grid, row + 1, col),
                Location::lookup(grid, row, col - 1)) {
            if south.connects_north() && west.connects_east() {
                return ElbowSouthWest;
            }
        }
        panic!("can only infer locations along the pipe");
    }
}

#[derive(PartialEq, Eq)]
enum Location {
    Empty,
    UninitializedStart,
    Pipe(PipeShape),
}

impl Location {
    fn from(c: char) -> Location {
        match c {
            '|' => Pipe(Vertical),
            '-' => Pipe(Horizontal),
            'L' => Pipe(ElbowNorthEast),
            'J' => Pipe(ElbowNorthWest),
            'F' => Pipe(ElbowSouthEast),
            '7' => Pipe(ElbowSouthWest),
            'S' => UninitializedStart,
            _ => Empty,
        }
    }

    fn lookup(
        grid: &Vec<Vec<Location>>, row: i64, col: i64
    ) -> Option<&Location> {
        match grid.get(row as usize) {
            Some(cols) => cols.get(col as usize),
            None => None,
        }
    }

    fn lookup_mut(
        grid: &mut Vec<Vec<Location>>, row: i64, col: i64
    ) -> &mut Location {
        grid.get_mut(row as usize).unwrap().get_mut(col as usize).unwrap()
    }

    fn connects_north(&self) -> bool {
        *self == Pipe(Vertical) || *self == Pipe(ElbowNorthEast) ||
        *self == Pipe(ElbowNorthWest)
    }

    fn connects_south(&self) -> bool {
        *self == Pipe(Vertical) || *self == Pipe(ElbowSouthEast) ||
        *self == Pipe(ElbowSouthWest)
    }

    fn connects_east(&self) -> bool {
        *self == Pipe(Horizontal) || *self == Pipe(ElbowNorthEast) ||
        *self == Pipe(ElbowSouthEast)
    }

    fn connects_west(&self) -> bool {
        *self == Pipe(Horizontal) || *self == Pipe(ElbowNorthWest) ||
        *self == Pipe(ElbowSouthWest)
    }
}

struct Map {
    grid: Vec<Vec<Location>>,
    start: (i64, i64),
    pipe: HashSet<(i64, i64)>,
}

impl Map {
    fn new(input: Stdin) -> Map {
        let mut grid: Vec<Vec<Location>> = input
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.chars().map(Location::from).collect())
            .collect();
        let row_count = grid.len();
        let col_count = grid.first().unwrap().len();
        let mut start = (0, 0);
        'outer: for row in 0..row_count {
            for col in 0..col_count {
                let row = row as i64;
                let col = col as i64;
                let cur = Location::lookup(&grid, row, col).unwrap();
                if *cur == UninitializedStart {
                    start = (row, col);
                    let t = PipeShape::infer_from_neighbors(&grid, row, col);
                    let cur = Location::lookup_mut(&mut grid, row, col);
                    *cur = Pipe(t);
                    break 'outer;
                }
            }
        }
        let pipe = HashSet::new();
        Map { grid, start, pipe }
    }

    fn lookup(&self, row: i64, col: i64) -> Option<&Location> {
        Location::lookup(&self.grid, row, col)
    }

    fn is_pipe(&self, row: i64, col: i64) -> bool {
        self.pipe.contains(&(row, col))
    }

    fn get_neighbors(&self, row: i64, col: i64) -> ((i64, i64), (i64, i64)) {
        match self.lookup(row, col).unwrap() {
            Pipe(Vertical) => ((row - 1, col), (row + 1, col)),
            Pipe(Horizontal) => ((row, col + 1), (row, col - 1)),
            Pipe(ElbowNorthEast) => ((row, col + 1), (row - 1, col)),
            Pipe(ElbowNorthWest) => ((row - 1, col), (row, col - 1)),
            Pipe(ElbowSouthEast) => ((row, col + 1), (row + 1, col)),
            Pipe(ElbowSouthWest) => ((row, col - 1), (row + 1, col)),
            Empty | UninitializedStart => {
                panic!("only pipes have connected neighbors!")
            },
        }
    }

    fn mark_loop(&mut self, row: i64, col: i64) -> i64 {
        let start = (row, col);
        let mut cur = start;
        let mut prev = cur;
        let mut count: i64 = 0;
        loop {
            let (cur_row, cur_col) = cur;
            let (next1, next2) = self.get_neighbors(cur_row, cur_col);
            let next_prev = cur;
            cur = if next1 == prev { next2 } else { next1 };
            prev = next_prev;
            self.pipe.insert(cur);
            count += 1;
            if cur == start {
                break;
            }
        }
        count
    }

    fn count_insides(&self) -> usize {
        let mut total = 0;
        let row_count = self.grid.len();
        let col_count = self.grid.first().unwrap().len();
        for row in 0..row_count {
            let mut inside = false;
            let mut last_eastward_elbow = Horizontal; // track runs of pipe
            for col in 0..col_count {
                let row = row as i64;
                let col = col as i64;
                if self.is_pipe(row, col) {
                    let cur = self.lookup(row, col).unwrap();
                    match *cur {
                        Pipe(Vertical) => {
                            inside = !inside;
                        },
                        Pipe(Horizontal) => continue,
                        Pipe(ElbowNorthEast) => {
                            last_eastward_elbow = ElbowNorthEast;
                        },
                        Pipe(ElbowSouthEast) => {
                            last_eastward_elbow = ElbowSouthEast;
                        },
                        Pipe(ElbowNorthWest) => {
                            if last_eastward_elbow == ElbowSouthEast {
                                inside = !inside;
                            }
                        },
                        Pipe(ElbowSouthWest) => {
                            if last_eastward_elbow == ElbowNorthEast {
                                inside = !inside;
                            }
                        },
                        Empty | UninitializedStart => {
                            panic!("location reported as pipe, but wasn't");
                        },
                    }
                } else {
                    if inside {
                        total += 1;
                    }
                }
            }
        }
        total
    }
}

fn main() {
    let mut map = Map::new(io::stdin());
    let (start_row, start_col) = map.start;
    map.mark_loop(start_row, start_col);
    let total = map.count_insides();
    println!("{total}");
}
