use std::fs;
use std::io;

// is it only me or the more I progress the more diabolical it gets?

fn main() -> io::Result<()> {
    let file_path = "input_files/day3_input";
    let buf: String = fs::read_to_string(file_path)?;

    let sum = sum_mults(&buf);
    println!("The result to day3_2 is: {}", sum);

    Ok(())
}

fn sum_mults(buf: &str) -> i32 {
    let mut sum = 0;
    let mut start = 0;

    let mut enabled = true;

    // good luck cyphering this one.
    loop {
        let closest_mul = buf[start..].find("mul(");
        let closest_do = buf[start..].find("do()");
        let closest_dont = buf[start..].find("don't()");

        let index: usize;
        if let Some(idx) = min_option(closest_mul, closest_do, closest_dont) {
            index = idx + start;
        } else {
            break;
        }

        if buf[index..index + 4].to_string() == "do()" {
            enabled = true;
            start = index + 1;
        } else if buf[index..index + 7].to_string() == "don't()" {
            enabled = false;
            start = index + 1;
        } else if enabled {
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
        } else {
            start = index + 1;
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

fn min_option(a: Option<usize>, b: Option<usize>, c: Option<usize>) -> Option<usize> {
    [a, b, c].into_iter().flatten().min()
}
