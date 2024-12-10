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

    println!("The result to day2_1 is: {}", safe_count);
}

fn is_line_legal(line: &str) -> bool {
    let mut _curr: i32 = 0;
    let mut _prev: i32 = 0;
    let mut increasing: bool = true;

    for (i, num_str) in line.split_terminator(' ').enumerate() {
        let num = num_str.parse().unwrap();
        if i == 0 {
            _prev = num;
            continue;
        }
        if i == 1 {
            _curr = num;
            increasing = _curr > _prev;

            // check if difference is legal.
            if _curr.abs_diff(_prev) < 1 || _curr.abs_diff(_prev) > 3 {
                return false;
            }
            _prev = _curr;
            continue;
        }

        // now handle the rest cases.
        _curr = num;
        if (_curr > _prev) != increasing || _curr.abs_diff(_prev) < 1 || _curr.abs_diff(_prev) > 3 {
            return false;
        }
        _prev = _curr;
    }
    true
}
