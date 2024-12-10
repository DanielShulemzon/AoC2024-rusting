use aoc2024_rs::Helper;

fn main() {
    let file_path = "input_files/day2_input";
    let lines = Helper::read_lines(file_path).unwrap();

    let mut safe_count = 0;

    for line in lines.flatten() {
        if is_line_legal(line.as_str()) {
            safe_count += 1;
        }
    }

    println!("The result to day2_2 is: {}", safe_count);
}

// take a different approach and way less efficient but we'll let it pass.
fn is_line_legal(line: &str) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|s: &str| s.parse().unwrap())
        .collect();

    for i in 0..numbers.len() {
        let temp_vec = numbers
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, val)| val)
            .collect::<Vec<i32>>();

        if check_legality(&temp_vec) {
            return true;
        }
    }
    false
}

fn check_legality(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return false;
    }
    let increase: bool = report[1] > report[0];

    let mut _prev_idx = 0;
    let mut _curr_idx = 1;
    while _curr_idx < report.len() {
        let prev_val = report[_prev_idx];
        let curr_val = report[_curr_idx];

        if (curr_val > prev_val) != increase
            || curr_val.abs_diff(prev_val) > 3
            || curr_val.abs_diff(prev_val) < 1
        {
            return false;
        }

        //increment
        _prev_idx += 1;
        _curr_idx += 1;
    }
    true
}
