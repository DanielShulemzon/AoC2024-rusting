use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "input_files/day3_input";
    let buf: String = fs::read_to_string(file_path)?;

    let sum = sum_mults(&buf);
    println!("The result to day3_1 is: {}", sum);

    Ok(())
}

fn sum_mults(buf: &str) -> i32 {
    let mut sum = 0;
    let mut start = 0;
    while let Some(index) = buf[start..].find("mul(") {
        let index = start + index;

        if let Some(closing_idx) = buf[index..].find(')') {
            let closing_idx = index + closing_idx;
            if let Some((a, b)) = is_legal_nums(&buf[index + 4..closing_idx]) {
                sum += a * b;
                start = closing_idx + 1;
            } else {
                start = index + 1;
            }
        } else {
            break;
        }
    }
    sum
}

fn is_legal_nums(nums_str: &str) -> Option<(i32, i32)> {
    nums_str
        .split_once(',')
        .and_then(|(a, b)| a.parse::<i32>().ok().zip(b.parse::<i32>().ok()))
        .filter(|(a, b)| (1..=999).contains(a) && (1..=999).contains(b))
}
