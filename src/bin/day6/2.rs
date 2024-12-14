use std::collections::HashSet;

// this code sadly does not work.
// for the future: make it a bit object oriented and stuff..

fn main() {
    let input: &str = include_str!("../../../input_files/day6_input");

    let guard_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (mut current_col, mut current_row) = guard_map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .expect("no '^' found!");

    let mut direction = 0; // 0 - up, 1 - right, 2 - down, 3 - left.
    let mut loop_count = 1; // including standing position

    loop {
        let facing_block = {
            let row = if direction == 0 {
                current_row - 1
            } else if direction == 2 {
                current_row + 1
            } else {
                current_row
            };
            let col = if direction == 1 {
                current_col + 1
            } else if direction == 3 {
                current_col - 1
            } else {
                current_col
            };
            (col, row)
        }; // this block is always in range. we check it in the end of the loop.

        if guard_map[facing_block.1][facing_block.0] == '#' {
            direction = (direction + 1) % 4;
            continue;
        }
        // else

        (current_col, current_row) = facing_block;

        if check_for_loop((current_col, current_row), direction, &guard_map) {
            loop_count += 1;
            println!("GREAT! ({}, {})", current_col, current_row);
        }

        if current_row == 0
            || current_col == 0
            || current_row == guard_map.len() - 1
            || current_col == guard_map[0].len() - 1
        {
            break;
        }
    }

    println!("result for day6_2 is: {}", loop_count);
}

fn check_for_loop(
    starting_pos: (usize, usize),
    starting_direction: i32,
    og_map: &[Vec<char>],
) -> bool {
    // we put a wall in front of us right now (no walls are currently in front)
    let (mut current_col, mut current_row) = starting_pos;
    let mut direction = starting_direction;

    let mut guard_map: Vec<Vec<char>> = og_map.to_vec();

    let facing_block: (i32, i32) = {
        // in order to stay away from integer undeflow we turn them to i32.
        let current_col = current_col as i32;
        let current_row = current_row as i32;

        let row = if direction == 0 {
            current_row - 1
        } else if direction == 2 {
            current_row + 1
        } else {
            current_row
        };
        let col = if direction == 1 {
            current_col + 1
        } else if direction == 3 {
            current_col - 1
        } else {
            current_col
        };
        (col, row)
    };

    if facing_block.0 < 0
        || facing_block.1 < 0
        || facing_block.0 == guard_map[0].len() as i32
        || facing_block.1 == guard_map.len() as i32
    {
        return false;
    } else {
        let facing_block = (facing_block.0 as usize, facing_block.1 as usize);
        guard_map[facing_block.1][facing_block.0] = '#';
    }

    let mut tracked: HashSet<((usize, usize), i32)> = HashSet::with_capacity(100);

    loop {
        let facing_block: (i32, i32) = {
            // in order to stay away from integer undeflow we turn them to i32.
            let current_col = current_col as i32;
            let current_row = current_row as i32;

            let row = if direction == 0 {
                current_row - 1
            } else if direction == 2 {
                current_row + 1
            } else {
                current_row
            };
            let col = if direction == 1 {
                current_col + 1
            } else if direction == 3 {
                current_col - 1
            } else {
                current_col
            };
            (col, row)
        }; // this now is not necessarily legal, if it's not we return false.

        if facing_block.0 < 0
            || facing_block.1 < 0
            || facing_block.0 == guard_map[0].len() as i32
            || facing_block.1 == guard_map.len() as i32
        {
            return false;
        }
        // else

        // ensured by our previous search.
        let facing_block = (facing_block.0 as usize, facing_block.1 as usize);

        if guard_map[facing_block.1][facing_block.0] == '#' {
            direction = (direction + 1) % 4;
            continue;
        }

        (current_col, current_row) = facing_block;

        if tracked.contains(&((current_col, current_row), direction)) {
            return true;
        } else {
            tracked.insert(((current_col, current_row), direction));
        }
    }
}
