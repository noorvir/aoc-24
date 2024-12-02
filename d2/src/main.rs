use std::fs::File;
use std::io::{self, Read};

fn main() {
    let data = read_lines_from_file().unwrap();

    let num_safe = get_num_safe(data.clone());
    let num_sage_problem_damped = num_safe_dropout(data.clone());

    println!("{:?}", num_safe);
    println!("{:?}", num_sage_problem_damped);
}

fn num_safe_dropout(lines: Vec<Vec<i32>>) -> i32 {
    let mut num_unsafe: i32 = 0;
    let num_total: i32 = lines.len().try_into().unwrap();

    for nums in lines {
        if is_seq_safe(&nums) {
            continue;
        }

        let mut is_safe = false;

        // dropout one number at a time
        for i in 0..nums.len() {
            let mut nums_dropout = nums.clone();
            nums_dropout.remove(i);

            if is_seq_safe(&nums_dropout) {
                is_safe = true;
                break;
            }
        }

        if !is_safe {
            num_unsafe += 1;
        }
    }

    num_total - num_unsafe
}

fn get_num_safe(lines: Vec<Vec<i32>>) -> i32 {
    let mut num_unsafe: i32 = 0;
    let num_total: i32 = lines.len().try_into().unwrap();

    for nums in lines {
        if !is_seq_safe(&nums) {
            num_unsafe += 1;
        }
    }

    num_total - num_unsafe
}

fn is_seq_safe(nums: &Vec<i32>) -> bool {
    let trend_increasing = nums[0] < nums[1];

    for i in 1..nums.len() {
        if !is_pair_safe(nums[i - 1], nums[i], trend_increasing) {
            return false;
        }
    }

    true
}

fn is_pair_safe(num_a: i32, num_b: i32, trend_increasing: bool) -> bool {
    if num_a == num_b {
        return false;
    }

    let current_is_increasing = num_a < num_b;
    if current_is_increasing != trend_increasing {
        return false;
    }

    let diff = num_b - num_a;
    let abs_diff = if diff > 0 { diff } else { -diff };

    if abs_diff < 1 || abs_diff > 3 {
        return false;
    }

    true
}

fn read_lines_from_file() -> Result<Vec<Vec<i32>>, io::Error> {
    let mut file = File::open("./data.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.lines().collect();

    let data: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    Ok(data)
}
