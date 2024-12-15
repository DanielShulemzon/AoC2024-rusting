use std::collections::{HashMap, HashSet};

struct Grid {
    tiles: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = tiles.len();
        let width = tiles[0].len();

        Self {
            tiles,
            height,
            width,
        }
    }

    pub fn coords_in_grid(&self, pos: (isize, isize)) -> bool {
        let (col, row) = pos;
        col >= 0 && col < self.width as isize && row >= 0 && row < self.height as isize
    }
}

fn update_antinodes_pt1(
    unique_locations: &mut HashSet<(usize, usize)>,
    occurences: &[(usize, usize)],
    signal_grid: &Grid,
) -> usize {
    let mut unique_count: usize = 0;

    for &occur1 in occurences.iter() {
        for &occur2 in occurences.iter() {
            if occur1 == occur2 {
                // the skip dude.
                continue;
            }

            // calculate dxdy
            let dx: isize = occur1.0 as isize - occur2.0 as isize;
            let dy: isize = occur1.1 as isize - occur2.1 as isize;

            // now we go opposite direction in the same line.
            let new_position = (occur1.0 as isize + dx, occur1.1 as isize + dy);

            if !signal_grid.coords_in_grid(new_position) {
                // out of bounds
                continue;
            }
            let new_position: (usize, usize) = (new_position.0 as usize, new_position.1 as usize);

            if unique_locations.insert(new_position) {
                unique_count += 1;
            }
        }
    }
    unique_count
}

// part 2

fn get_antinodes_on_line(
    signal_grid: &Grid,
    occur1: (usize, usize),
    occur2: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut antinodes: Vec<(usize, usize)> = Vec::new();

    // Compute dx dy
    let dx = occur1.0 as isize - occur2.0 as isize;
    let dy = occur1.1 as isize - occur2.1 as isize;

    let mut curr_position = (occur1.0 as isize, occur1.1 as isize);

    // Traverse forward while staying within grid bounds
    while signal_grid.coords_in_grid(curr_position) {
        let curr_usize = (curr_position.0 as usize, curr_position.1 as usize);
        antinodes.push(curr_usize);
        curr_position = (curr_position.0 + dx, curr_position.1 + dy);
    }

    // Traverse in the other direction
    curr_position = (occur1.0 as isize - dx, occur1.1 as isize - dy);

    while signal_grid.coords_in_grid(curr_position) {
        let curr_usize = (curr_position.0 as usize, curr_position.1 as usize);
        antinodes.push(curr_usize);
        curr_position = (curr_position.0 - dx, curr_position.1 - dy);
    }

    antinodes
}

fn update_antinodes_pt2(
    unique_locations: &mut HashSet<(usize, usize)>,
    occurences: &[(usize, usize)],
    signal_grid: &Grid,
) -> usize {
    let mut unique_count: usize = 0;

    for &occur1 in occurences.iter() {
        for &occur2 in occurences.iter() {
            if occur1 == occur2 {
                // the skip dude.
                continue;
            }

            // get all possible antinodes in the line
            let positions: Vec<(usize, usize)> = get_antinodes_on_line(signal_grid, occur1, occur2);

            for &pos in &positions {
                if unique_locations.insert(pos) {
                    unique_count += 1;
                }
            }
        }
    }
    unique_count
}

fn main() {
    let input = include_str!("../../../input_files/day8_input");

    let signal_grid: Grid = Grid::new(input);

    let mut antenna_table: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut unique_locations: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in signal_grid.tiles.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch != '.' {
                antenna_table
                    .entry(ch)
                    .and_modify(|occurences| occurences.push((x, y)))
                    .or_insert(vec![(x, y)]);
            }
        }
    }

    let unique_count: usize = antenna_table
        .values()
        .map(|occurences| update_antinodes_pt1(&mut unique_locations, occurences, &signal_grid))
        .sum();

    println!("solution for day 8_1 is: {}", unique_count);

    // part 2.
    let mut unique_locations: HashSet<(usize, usize)> = HashSet::new();

    let unique_count: usize = antenna_table
        .values()
        .map(|occurences| update_antinodes_pt2(&mut unique_locations, occurences, &signal_grid))
        .sum();

    println!("solution for day 8_2 is: {}", unique_count);
}
