use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_data();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<Vec<String>>) {
    let rows = data.len();
    let cols = data.first().unwrap().len();

    let mut count = 0;

    for i in 0..cols {
        for j in 0..rows {
            // check vertical
            if j < rows - 3 {
                let s = format!(
                    "{}{}{}{}",
                    data[i][j],
                    data[i][j + 1],
                    data[i][j + 2],
                    data[i][j + 3]
                );

                if is_xmas(&s) {
                    count += 1;
                }
            }

            // check horizontal
            if i < cols - 3 {
                let s = format!(
                    "{}{}{}{}",
                    data[i][j],
                    data[i + 1][j],
                    data[i + 2][j],
                    data[i + 3][j]
                );

                if is_xmas(&s) {
                    count += 1;
                }
            }

            // check left diagonal
            if i < cols - 3 && j < rows - 3 {
                let s = format!(
                    "{}{}{}{}",
                    data[i][j],
                    data[i + 1][j + 1],
                    data[i + 2][j + 2],
                    data[i + 3][j + 3]
                );

                if is_xmas(&s) {
                    count += 1;
                }
            }

            // check right diagonal
            if i < cols - 3 && j > 2 {
                let s = format!(
                    "{}{}{}{}",
                    data[i][j],
                    data[i + 1][j - 1],
                    data[i + 2][j - 2],
                    data[i + 3][j - 3]
                );

                if is_xmas(&s) {
                    count += 1;
                }
            }
        }
    }

    println!("count: {}", count);
}

fn is_xmas(s: &str) -> bool {
    s == "XMAS" || s == "SAMX"
}

fn part_2(data: &Vec<Vec<String>>) {
    let rows = data.len();
    let cols = data.first().unwrap().len();

    let mut viz = vec![vec![".".to_string(); cols]; rows];

    let mut count = 0;

    for i in 0..cols {
        for j in 0..rows {
            let mut is_left = false;
            let mut is_right = false;

            // left diagonal
            if i < cols - 2 && j < rows - 2 {
                let s = format!("{}{}{}", data[i][j], data[i + 1][j + 1], data[i + 2][j + 2],);

                if is_mas(&s) {
                    is_left = true;
                    viz[i][j] = data[i][j].clone();
                    viz[i + 1][j + 1] = data[i + 1][j + 1].clone();
                    viz[i + 2][j + 2] = data[i + 2][j + 2].clone();
                }
            }

            // right diagonal shifted
            let x = i + 2;

            if x < cols && j < rows - 2 {
                let s = format!("{}{}{}", data[x][j], data[x - 1][j + 1], data[x - 2][j + 2]);

                if is_mas(&s) {
                    is_right = true;
                    viz[x][j] = data[x][j].clone();
                    viz[x - 1][j + 1] = data[x - 1][j + 1].clone();
                    viz[x - 2][j + 2] = data[x - 2][j + 2].clone();
                }
            }

            if is_left && is_right {
                count += 1;
            }
        }
    }

    println!("count: {}", count);
}

fn is_mas(s: &str) -> bool {
    s == "MAS" || s == "SAM"
}

fn read_data() -> Vec<Vec<String>> {
    let mut file = File::open("data.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let lines: Vec<&str> = data.lines().collect();

    let rows = lines.len();
    let cols = lines.first().unwrap().len();

    let mut result = vec![vec![String::new(); cols]; rows];

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            result[i][j] = char.to_string();
        }
    }

    result
}
