use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("../../../input_files/day6_input");

    let guard_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (mut current_col, mut current_row) = guard_map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (x, y)))
        .expect("no '^' found!");

    let mut direction = 0; // 0 - up, 1 - right, 2 - down, 3 - left.
    let mut count = 1; // including standing position

    let mut used_coords: HashSet<(usize, usize)> = HashSet::with_capacity(100);
    used_coords.insert((current_col, current_row));

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

        if !used_coords.contains(&(current_col, current_row)) {
            used_coords.insert((current_col, current_row));
            count += 1;
        }
        println!("updated count at ({}, {})", current_col, current_row);

        if current_row == 0
            || current_col == 0
            || current_row == guard_map.len() - 1
            || current_col == guard_map[0].len() - 1
        {
            break;
        }
    }

    println!("result for day6_1 is: {}", count);
}
