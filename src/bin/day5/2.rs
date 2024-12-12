use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

// task: make the most inefficiend most unreadable code with chatgpt even made.
// I'd say I did well.

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
        .filter(|update| !is_legal_update(update, &ordering_rules))
        .map(|update| sort_update(update, &ordering_rules))
        .map(|line| get_middle_number(line.as_slice()))
        .sum();

    println!("result for day5_1 is: {}", sum);
}

fn sort_update(line: &[char], ordering_rules: &[&[char]]) -> Vec<char> {
    let line_string: String = line.iter().collect();

    let parts: Vec<&str> = line_string.split(',').collect();

    let constrains: Vec<(i32, i32)> = ordering_rules
        .iter()
        .map(|line| get_numbers(line))
        .filter(|(a, b)| {
            if let (Some(_a), Some(_b)) = (get_index(line, *a), get_index(line, *b)) {
                true
            } else {
                false
            }
        })
        .collect();

    if let Some(solution) = topological_sort(parts, constrains) {
        let mut sol: Vec<char> = solution
            .iter()
            .flat_map(|s| s.chars().chain(std::iter::once(',')))
            .collect();
        sol.pop();
        sol
    } else {
        panic!("what???");
    }
}

fn topological_sort(strings: Vec<&str>, constraints: Vec<(i32, i32)>) -> Option<Vec<&str>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, i32> = HashMap::new();

    // Create graph and in-degree map from constraints
    for (a, b) in constraints {
        graph.entry(a).or_insert_with(Vec::new).push(b);
        in_degree.entry(b).or_insert(0);
        *in_degree.entry(b).or_insert(0) += 1;
        in_degree.entry(a).or_insert(0); // Ensure that `a` exists in the in_degree map
    }

    // Initialize the queue with nodes having no incoming edges (in-degree 0)
    let mut queue: VecDeque<i32> = VecDeque::new();
    for &str_num in strings.iter() {
        let num = str_num.parse::<i32>().unwrap(); // Assuming string can be parsed to i32
        if in_degree.get(&num) == Some(&0) {
            queue.push_back(num);
        }
    }

    let mut sorted_order: Vec<i32> = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted_order.push(node);

        // Process neighbors (adjacent nodes)
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                let count = in_degree.entry(neighbor).or_insert(0);
                *count -= 1; // Reduce the in-degree of the neighbor
                if *count == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // If the sorted_order length is equal to the number of nodes, we have a valid ordering
    if sorted_order.len() == strings.len() {
        let result: Vec<&str> = sorted_order
            .iter()
            .map(|&num| {
                strings
                    .iter()
                    .find(|&&s| s.parse::<i32>().unwrap() == num)
                    .unwrap()
            })
            .map(|s| *s)
            .collect();
        Some(result)
    } else {
        None // There was a cycle (invalid ordering)
    }
}

fn is_legal_update(line: &[char], ordering_rules: &[&[char]]) -> bool {
    ordering_rules
        .iter()
        .map(|line| get_numbers(line))
        .all(|(a, b)| {
            if let (Some(a), Some(b)) = (get_index(line, a), get_index(line, b)) {
                return a < b;
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
