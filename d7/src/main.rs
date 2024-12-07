use rayon::prelude::*;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let data = read_data();
    let sum_1 = part_1(&data);

    let start = Instant::now();
    let sum_2 = part_2(&data);
    let duration = start.elapsed();

    println!("Part 1: {:?}", sum_1);
    println!("Part 2: {:?}, {:?}", sum_2, duration);
}

fn part_1(data: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut sum = 0;

    for (res, numbers) in data {
        let acc = numbers[0];
        let remaining = numbers[1..].to_vec();
        let tree = reduce(acc, &remaining);

        if let Some(leaf) = tree.find_eq(*res) {
            sum += leaf;
        }
    }

    sum
}

fn part_2(data: &Vec<(i64, Vec<i64>)>) -> i64 {
    let sum: i64 = data
        .par_iter()
        .map(|(res, numbers)| {
            let acc = numbers[0];
            let remaining = &numbers[1..];
            let tree = reduce_part_2(acc, remaining);

            if let Some(leaf) = tree.find_eq(*res) {
                leaf
            } else {
                0
            }
        })
        .sum();

    sum
}

struct Tree {
    acc: i64,
    mul: Option<Box<Tree>>,
    plus: Option<Box<Tree>>,
}

impl Tree {
    fn get_leaves(&self) -> Vec<i64> {
        if self.mul.is_none() && self.plus.is_none() {
            return vec![self.acc];
        }

        let mut leaves: Vec<i64> = Vec::new();

        if let Some(ref mul) = self.mul {
            leaves.extend(mul.get_leaves());
        }
        if let Some(ref plus) = self.plus {
            leaves.extend(plus.get_leaves());
        }

        leaves
    }

    fn find_eq(&self, target: i64) -> Option<i64> {
        let leaves = self.get_leaves();

        for leaf in leaves {
            if leaf == target {
                return Some(leaf);
            }
        }

        None
    }
}

struct Tree2 {
    acc: i64,
    mul: Option<Box<Tree2>>,
    plus: Option<Box<Tree2>>,
    concat: Option<Box<Tree2>>,
}

impl Tree2 {
    fn get_leaves(&self) -> Vec<i64> {
        if self.mul.is_none() && self.plus.is_none() && self.concat.is_none() {
            return vec![self.acc];
        }

        let mut leaves: Vec<i64> = Vec::new();

        if let Some(ref mul) = self.mul {
            leaves.extend(mul.get_leaves());
        }
        if let Some(ref plus) = self.plus {
            leaves.extend(plus.get_leaves());
        }
        if let Some(ref concat) = self.concat {
            leaves.extend(concat.get_leaves());
        }

        leaves
    }

    fn find_eq(&self, target: i64) -> Option<i64> {
        let leaves = self.get_leaves();

        for leaf in leaves {
            if leaf == target {
                return Some(leaf);
            }
        }

        None
    }
}

fn reduce_part_2(acc: i64, vec: &[i64]) -> Tree2 {
    if vec.is_empty() {
        return Tree2 {
            acc,
            mul: None,
            plus: None,
            concat: None,
        };
    }
    let b = vec[0];
    let remaining = &vec[1..];

    let plus_acc = apply_op("+".to_string(), acc, b);
    let mul_acc = apply_op("*".to_string(), acc, b);
    let concat_acc = acc.to_string() + &b.to_string();
    let concat_acc = concat_acc.parse::<i64>().unwrap();

    let mul_tree = reduce_part_2(mul_acc, &remaining);
    let plus_tree = reduce_part_2(plus_acc, &remaining);
    let concat_tree = reduce_part_2(concat_acc, &remaining);

    let tree = Tree2 {
        acc,
        mul: Some(Box::new(mul_tree)),
        plus: Some(Box::new(plus_tree)),
        concat: Some(Box::new(concat_tree)),
    };

    return tree;
}

// Returns the result and the sequence of operations
fn reduce(acc: i64, vec: &[i64]) -> Tree {
    if vec.is_empty() {
        return Tree {
            acc,
            mul: None,
            plus: None,
        };
    }
    let b = vec[0];
    let remaining = &vec[1..];

    let plus_acc = apply_op("+".to_string(), acc, b);
    let mul_acc = apply_op("*".to_string(), acc, b);

    let mul_tree = reduce(mul_acc, remaining);
    let plus_tree = reduce(plus_acc, remaining);

    let tree = Tree {
        acc,
        mul: Some(Box::new(mul_tree)),
        plus: Some(Box::new(plus_tree)),
    };

    return tree;
}

fn apply_op(op: String, a: i64, b: i64) -> i64 {
    match op.as_str() {
        "+" => a + b,
        "*" => a * b,
        _ => panic!("Invalid operator"),
    }
}

fn read_data() -> Vec<(i64, Vec<i64>)> {
    let mut file = File::open("data.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut result: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in data.lines() {
        let res = line.split(":").next().unwrap();
        let numbers = line.split(":").nth(1).unwrap();

        let a: i64 = res.parse::<i64>().unwrap();
        let b: Vec<i64> = numbers
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<i64>().unwrap())
            .collect();

        result.push((a, b));
    }

    result
}
