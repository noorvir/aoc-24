use std::fs::File;
use std::io::{self, Read};

fn main() {
    let mut numbers = read_data().unwrap();

    // let mut numbers = (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
    numbers.0.sort();
    numbers.1.sort();

    let mut result = lists_distance(&numbers.0, &numbers.1);
    let mut similarity = lists_similarity(&numbers.0, &numbers.1);

    println!("{}", result);
    println!("{:?}", similarity);
}

fn lists_similarity(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut similarity_vec: Vec<(i32, i32)> = Vec::new();

    for i in 0..list1.len() {
        let mut num_occurrences = 0;
        for j in 0..list2.len() {
            if list1[i] == list2[j] {
                num_occurrences += 1;
            }
        }
        similarity_vec.push((list1[i], num_occurrences));
    }

    similarity_vec.iter().map(|(a, b)| a * b).sum()
}

fn lists_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    (0..list1.len()).fold(0, |acc, i| {
        let mut diff: i32 = list1[i] - list2[i];
        if diff < 0 {
            diff = -diff;
        }
        acc + diff
    })
}

fn read_data() -> Result<(Vec<i32>, Vec<i32>), io::Error> {
    let mut file = File::open("./data/data.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let lines: Vec<(i32, i32)> = contents
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok());
            let first = nums.next()?;
            let second = nums.next()?;
            Some((first, second))
        })
        .collect();

    let first_column: Vec<i32> = lines.iter().map(|(a, _)| *a).collect();
    let second_column: Vec<i32> = lines.iter().map(|(_, b)| *b).collect();

    Ok((first_column, second_column))
}
