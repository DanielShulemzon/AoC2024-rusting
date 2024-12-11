use std::{char, fs::read_to_string};

fn main() {
    let file_path = "input_files/day4_input";
    let lines: Vec<Vec<char>> = read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let lines: Vec<&[char]> = lines.iter().map(|line| line.as_slice()).collect();

    let sum = count_xmas(&lines);

    println!("result of day4_1 is: {}", sum);
}

fn count_xmas(lines: &[&[char]]) -> i32 {
    let sum = lines.iter().enumerate().fold(0, |acc, (row, line)| {
        acc + line
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 'X')
            .map(|(col, _)| count_legal(row, col, lines))
            .sum::<i32>()
    });

    sum
}

fn count_legal(row: usize, col: usize, lines: &[&[char]]) -> i32 {
    let rows = lines.len();
    let cols = lines[0].len();

    //values for capabilities of all directions
    let left = col >= 3;
    let right = col < cols - 3;
    let up = row >= 3;
    let down = row < rows - 3;

    let mut sum = 0;

    if left && lines[row][col - 3..col] == ['S', 'A', 'M'] {
        sum += 1;
    }

    if up && lines[row - 1][col] == 'M' && lines[row - 2][col] == 'A' && lines[row - 3][col] == 'S'
    {
        sum += 1;
    }

    if right && lines[row][col + 1..=col + 3] == ['M', 'A', 'S'] {
        sum += 1;
    }

    if down
        && lines[row + 1][col] == 'M'
        && lines[row + 2][col] == 'A'
        && lines[row + 3][col] == 'S'
    {
        sum += 1;
    }

    if right
        && up
        && lines[row - 1][col + 1] == 'M'
        && lines[row - 2][col + 2] == 'A'
        && lines[row - 3][col + 3] == 'S'
    {
        sum += 1;
    }

    if right
        && down
        && lines[row + 1][col + 1] == 'M'
        && lines[row + 2][col + 2] == 'A'
        && lines[row + 3][col + 3] == 'S'
    {
        sum += 1;
    }

    if left
        && up
        && lines[row - 1][col - 1] == 'M'
        && lines[row - 2][col - 2] == 'A'
        && lines[row - 3][col - 3] == 'S'
    {
        sum += 1;
    }

    if left
        && down
        && lines[row + 1][col - 1] == 'M'
        && lines[row + 2][col - 2] == 'A'
        && lines[row + 3][col - 3] == 'S'
    {
        sum += 1;
    }

    sum
}
