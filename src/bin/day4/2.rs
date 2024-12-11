use std::fs::read_to_string;

fn main() {
    let file_path = "input_files/day4_input";
    let lines: Vec<Vec<char>> = read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let lines: Vec<&[char]> = lines.iter().map(|line| line.as_slice()).collect();

    let sum = count_x_mas(&lines);

    println!("result for day4_2 is: {}", sum);
}

fn count_x_mas(lines: &[&[char]]) -> i32 {
    let sum = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'A')
                .map(|(col, _)| is_legal_x(row, col, lines))
                .sum::<i32>()
        })
        .sum::<i32>();

    sum
}

fn is_legal_x(row: usize, col: usize, lines: &[&[char]]) -> i32 {
    let rows = lines.len();
    let cols = lines[0].len();

    // if the A is at the outside fram an X shape could not appear.
    if row == 0 || col == 0 || row == rows - 1 || col == cols - 1 {
        return 0;
    }
    // why did I manually included any case? idk.. im lazy
    else if ((lines[row - 1][col - 1] == 'S' && lines[row + 1][col + 1] == 'M')
        || (lines[row - 1][col - 1] == 'M' && lines[row + 1][col + 1] == 'S'))
        && ((lines[row - 1][col + 1] == 'S' && lines[row + 1][col - 1] == 'M')
            || (lines[row - 1][col + 1] == 'M' && lines[row + 1][col - 1] == 'S'))
    {
        return 1;
    }
    0
}
