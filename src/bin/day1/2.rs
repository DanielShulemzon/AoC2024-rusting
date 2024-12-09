use std::collections::HashMap;

use aoc2024_rs::Helper;

fn main() {
    let file_path = "input_files/day1_input";
    let lines = Helper::read_lines(file_path).unwrap();

    let mut right_occurrences = HashMap::<i32, i32>::new();

    // iterate the lines for the first time to count how many instances of the numbers on the right
    // exist.
    for line in lines.flatten() {
        let right_num = get_right_number(line.as_str());

        // update the map, with a default number of 1.
        right_occurrences
            .entry(right_num)
            .and_modify(|occurences| *occurences += 1)
            .or_insert(1);
    }

    // load the lines again (lines were moved by iteration).
    let lines = Helper::read_lines(file_path).unwrap();

    let mut sum = 0;

    for line in lines.flatten() {
        let left_num = get_left_number(line.as_str());

        //search if occurence is found and update total sum.
        let occurences = if right_occurrences.contains_key(&left_num) {
            *right_occurrences.get(&left_num).unwrap()
        } else {
            0
        };

        sum += left_num * occurences;
    }

    println!("sum is: {}", sum);
}

fn get_left_number(line: &str) -> i32 {
    let first_space_index = line.find(' ').unwrap();
    line[..first_space_index].parse().unwrap()
}

fn get_right_number(line: &str) -> i32 {
    let last_space_index = line.rfind(' ').unwrap();
    line[last_space_index + 1..].parse().unwrap()
}
