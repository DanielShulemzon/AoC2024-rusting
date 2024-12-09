use aoc2024_rs::Helper;

fn main() {
    let file_path = "input_files/day1_input";
    let lines = Helper::read_lines(file_path).unwrap();

    let mut sum = 0;

    let mut vec_left = Vec::<i32>::with_capacity(1000);
    let mut vec_right = Vec::<i32>::with_capacity(1000);

    // append in order to our vectors.
    for line in lines.flatten() {
        let (n1, n2) = extract_numbers(line.as_str());
        vec_left.push(n1);
        vec_right.push(n2);
    }

    // sort them both
    vec_left.sort();
    vec_right.sort();

    // iterate trough the two vectors and find the difference.
    for i in 0..vec_left.len() {
        let n1 = vec_left[i];
        let n2 = vec_right[i];
        sum += (n1 - n2).abs();
    }

    println!("The result to day1_1 is: {}", sum)
}

fn extract_numbers(line: &str) -> (i32, i32) {
    let first_space_index = line.find(' ').unwrap();
    let last_space_index = line.rfind(' ').unwrap();

    let n1: i32 = line[..first_space_index].parse().unwrap();
    let n2: i32 = line[last_space_index + 1..].parse().unwrap();

    (n1, n2)
}
