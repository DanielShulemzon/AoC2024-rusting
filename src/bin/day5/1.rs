use std::fs::read_to_string;

fn main() {
    let file_path = "input_files/day5_input";
    let lines = read_to_string(file_path).unwrap();
    let (ordering_rules, to_produce) = {
        let (ordering_rules, to_produce) = lines.split_once("\n\n").unwrap();

        (
            ordering_rules
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            to_produce
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    };

    let ordering_rules: Vec<&[char]> = ordering_rules.iter().map(|line| line.as_slice()).collect();
    let to_produce: Vec<&[char]> = to_produce.iter().map(|line| line.as_slice()).collect();

    let sum: i32 = to_produce
        .iter()
        .filter(|update| is_legal_update(update, &ordering_rules))
        .map(|line| get_middle_number(line))
        .sum();

    println!("result for day5_1 is: {}", sum);
}

fn is_legal_update(line: &[char], ordering_rules: &[&[char]]) -> bool {
    ordering_rules
        .iter()
        .map(|line| get_numbers(line))
        .all(|(a, b)| {
            if let (Some(left), Some(right)) = (get_index(line, a), get_index(line, b)) {
                return left < right;
            }
            true
        })
}

fn get_numbers(line: &[char]) -> (i32, i32) {
    let pair = line.split_at(line.iter().position(|&c| c == '|').unwrap());

    let left_str: String = pair.0.iter().collect();
    let right_str: String = pair.1[1..].iter().collect();

    (left_str.parse().unwrap(), right_str.parse().unwrap())
}

fn get_index(line: &[char], num: i32) -> Option<usize> {
    let line_string: String = line.iter().collect();

    line_string.find(&format!("{}", num))
}

fn get_middle_number(line: &[char]) -> i32 {
    let line_string: String = line.iter().collect();

    let parts: Vec<&str> = line_string.split(',').collect();

    if parts.len() % 2 == 1 {
        parts[parts.len() / 2].parse().unwrap()
    } else {
        panic!("Unexpected even amount of numbers.");
    }
}
