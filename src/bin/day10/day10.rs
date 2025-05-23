use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

fn main() {
    let input = include_str!("../../../input_files/day10_input");

    let first_result = first_part(input);
    println!("first part result is: {}", first_result);

    // random comment: the first part is literally an extension of the first part (add caching)
    // ofc this is because my first part solution is super bad
    let second_result = second_part(input);
    println!("second part result is: {}", second_result);
}

// -------------------------------------------------------------------------
// first part

#[derive(Debug)]
struct Grid {
    values: Vec<Vec<u32>>,
    rows: isize,
    cols: isize,
}

impl Deref for Grid {
    type Target = Vec<Vec<u32>>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let values = input
            .lines()
            .map(|line| {
                line.trim_end()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            cols: values[0].len() as isize,
            rows: values.len() as isize,
            values,
        }
    }
}

fn first_part(input: &str) -> usize {
    let grid = Grid::from_input(input);

    let mut sum = 0;

    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter()
            .enumerate()
            .filter(|&(_, &val)| val == 0)
            .for_each(|(x, _)| {
                sum += fp_traverse_paths(&grid, x, y);
                // println!("x: {}, y: {}, sum: {}", x, y, sum);
            })
    });

    sum
}

fn fp_traverse_paths(grid: &Grid, x: usize, y: usize) -> usize {
    fn explore_directions(
        grid: &Grid,
        x: isize,
        y: isize,
        curr_num: u32,
        points_reached: &mut HashSet<(isize, isize)>,
    ) -> usize {
        if x < 0 || x >= grid.cols || y < 0 || y >= grid.rows {
            return 0;
        }
        if grid[y as usize][x as usize] != curr_num {
            return 0;
        }
        if curr_num == 9 {
            if points_reached.contains(&(y, x)) {
                return 0;
            } else {
                points_reached.insert((y, x));
                return 1;
            }
        }

        explore_directions(grid, x + 1, y, curr_num + 1, points_reached)
            + explore_directions(grid, x - 1, y, curr_num + 1, points_reached)
            + explore_directions(grid, x, y + 1, curr_num + 1, points_reached)
            + explore_directions(grid, x, y - 1, curr_num + 1, points_reached)
    }

    let mut points_reached = HashSet::new();
    explore_directions(grid, x as isize, y as isize, 0, &mut points_reached)
}

// -------------------------------------------------------------------------
// second part
// is this jsut super easy?

fn second_part(input: &str) -> usize {
    let grid = Grid::from_input(input);

    let mut sum = 0;

    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter()
            .enumerate()
            .filter(|&(_, &val)| val == 0)
            .for_each(|(x, _)| {
                sum += sp_traverse_paths(&grid, x, y);
                // println!("x: {}, y: {}, sum: {}", x, y, sum);
            })
    });

    sum
}

fn sp_traverse_paths(grid: &Grid, x: usize, y: usize) -> usize {
    fn explore_directions(grid: &Grid, x: isize, y: isize, curr_num: u32) -> usize {
        if x < 0 || x >= grid.cols || y < 0 || y >= grid.rows {
            return 0;
        }
        if grid[y as usize][x as usize] != curr_num {
            return 0;
        }
        if curr_num == 9 {
            return 1;
        }

        explore_directions(grid, x + 1, y, curr_num + 1)
            + explore_directions(grid, x - 1, y, curr_num + 1)
            + explore_directions(grid, x, y + 1, curr_num + 1)
            + explore_directions(grid, x, y - 1, curr_num + 1)
    }

    explore_directions(grid, x as isize, y as isize, 0)
}
