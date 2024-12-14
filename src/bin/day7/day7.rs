use aoc2024_rs::Helper;

fn process_line_into_numbers(line: &str) -> (usize, Vec<usize>) {
    let (target, parts) = line.split_once(':').unwrap();

    let cumilative: usize = target.parse().unwrap();

    let parts: Vec<usize> = parts
        .to_string()
        .split_whitespace()
        .map(|part| part.parse().unwrap())
        .collect();

    (cumilative, parts)
}

fn check_plus_mult(target: usize, parts: &[usize], curr_sum: usize) -> bool {
    if parts.is_empty() {
        return target == curr_sum;
    }
    if curr_sum > target {
        return false;
    }

    check_plus_mult(target, &parts[1..], curr_sum + parts[0])
        || check_plus_mult(target, &parts[1..], curr_sum * parts[0])
}

fn part_1() -> usize {
    let lines = Helper::read_lines("input_files/day7_input").unwrap();
    let mut result: usize = 0;

    for line in lines.flatten() {
        let (target, parts) = process_line_into_numbers(&line);
        let (&start, parts) = parts.split_first().unwrap();

        if check_plus_mult(target, parts, start) {
            result += target;
        }
    }
    result
}

fn concat_two_numbers(n1: usize, n2: usize) -> usize {
    (format!("{n1}") + format!("{n2}").as_str())
        .parse()
        .unwrap()
}

fn check_all_ops(target: usize, parts: &[usize], curr_sum: usize) -> bool {
    if parts.is_empty() {
        return target == curr_sum;
    }
    if curr_sum > target {
        return false;
    }

    check_all_ops(target, &parts[1..], curr_sum + parts[0])
        || check_all_ops(target, &parts[1..], curr_sum * parts[0])
        || check_all_ops(target, &parts[1..], concat_two_numbers(curr_sum, parts[0]))
}

fn part_2() -> usize {
    let lines = Helper::read_lines("input_files/day7_input").unwrap();
    let mut result: usize = 0;

    for line in lines.flatten() {
        let (target, parts) = process_line_into_numbers(&line);
        let (&start, parts) = parts.split_first().unwrap();

        if check_all_ops(target, parts, start) {
            result += target;
        }
    }
    result
}

fn main() {
    println!("result for day 7_1 is: {}\n", part_1());
    println!("result for day 7_2 is: {}", part_2());
}
