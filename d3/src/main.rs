use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_data();
    println!("Part 1:");
    println!("{}", compute_exprs(parse_mul_expr(data.clone())));

    println!("Part 2:");
    println!("{}", compute_flagged(data));
}

fn compute_flagged(data: String) -> i32 {
    let mut sum: i32 = 0;
    let mut do_op: bool = true;

    let mut curr_mul: String = "".to_string();
    let mut mul_exprs: Vec<String> = Vec::new();

    for i in 4..data.len() {
        let ch: String = data.chars().nth(i).unwrap().to_string();
        let mut sub_str = curr_mul.clone();

        if sub_str == "" {
            // Check the last 4 letters to see if they container the start of a capturing group
            sub_str = data[i - 4..i].to_string();

            if sub_str == "do()" {
                do_op = true;
                continue;
            }

            if i > 6 && data[i - 7..i].to_string() == "don't()" {
                do_op = false;
                continue;
            }

            if sub_str != "mul(" {
                continue;
            }
        }

        let (new_mul, capturing) = capture_mul(sub_str, ch);

        if new_mul == "" {
            curr_mul = "".to_string();
            continue;
        }

        // Successfully matched a mul expression
        if !capturing {
            if do_op {
                sum += compute_expr(new_mul.clone());
            }

            curr_mul = "".to_string();
            mul_exprs.push(new_mul);

            continue;
        }

        // Still capturing a mul expression
        curr_mul = new_mul;
    }

    sum
}

fn capture_mul(curr: String, ch: String) -> (String, bool) {
    if curr.contains(",") {
        return capture_mul_b(curr, ch);
    }
    return capture_mul_a(curr, ch);
}

fn capture_mul_a(curr: String, ch: String) -> (String, bool) {
    if ch == "," {
        return (curr + &ch, true);
    }
    if ch.parse::<i32>().is_ok() {
        return (curr + &ch, true);
    }
    // invalid character
    return ("".to_string(), false);
}

fn capture_mul_b(curr: String, ch: String) -> (String, bool) {
    if ch == ")" {
        return (curr + &ch, false); // done
    }
    if ch.parse::<i32>().is_ok() {
        return (curr + &ch, true);
    }
    // invalid character
    return ("".to_string(), false);
}

fn compute_exprs(exprs: Vec<String>) -> i32 {
    let mut sum = 0;
    for expr in exprs {
        sum += compute_expr(expr);
    }

    sum
}

fn compute_expr(expr: String) -> i32 {
    let digits = expr[4..expr.len() - 1]
        .to_string()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let a = digits[0];
    let b = digits[1];

    a * b
}

fn parse_mul_expr(data: String) -> Vec<String> {
    let mut exprs: Vec<String> = Vec::new();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.find_iter(&data);
    for cap in matches {
        exprs.push(cap.as_str().to_string());
    }

    exprs
}

fn read_data() -> String {
    let mut file = File::open("./data.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    contents
}
