use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_data();

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn part_1(data: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == 0 {
                let res = explore_next_step(data, (x, y));
                let mut peak_coords: HashSet<(usize, usize)> = HashSet::new();
                peak_coords.extend(res);
                sum += peak_coords.len();
            }
        }
    }

    sum
}

fn part_2(data: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == 0 {
                let res = explore_next_step(data, (x, y));
                sum += res.len();
            }
        }
    }

    sum
}

// Returns the coordinates of all valid 9 coordinates reachable from the current position.
// If no valid coordinates are reachable, returns an empty vector.
fn explore_next_step(data: &Vec<Vec<usize>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let current_elevation: usize = data[y][x];
    if current_elevation == 9 {
        return vec![(x, y)];
    }

    let mut results: Vec<(usize, usize)> = Vec::new();

    // North
    if y >= 1 {
        let n_elevation: usize = data[y - 1][x];
        if (n_elevation as isize - current_elevation as isize) == 1 {
            let n_res = explore_next_step(data, (x, y - 1));
            results.extend(n_res);
        }
    }

    // South
    if y < data.len() - 1 {
        let s_elevation: usize = data[y + 1][x];
        if (s_elevation as isize - current_elevation as isize) == 1 {
            let s_res = explore_next_step(data, (x, y + 1));
            results.extend(s_res);
        }
    }

    // West
    if x >= 1 {
        let w_elevation: usize = data[y][x - 1];
        if (w_elevation as isize - current_elevation as isize) == 1 {
            let w_res = explore_next_step(data, (x - 1, y));
            results.extend(w_res);
        }
    }

    // East
    if x < data[0].len() - 1 {
        let e_elevation: usize = data[y][x + 1];
        if (e_elevation as isize - current_elevation as isize) == 1 {
            let e_res = explore_next_step(data, (x + 1, y));
            results.extend(e_res);
        }
    }

    results
}

fn read_data() -> Vec<Vec<usize>> {
    let mut file = File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut result: Vec<Vec<usize>> = Vec::new();

    for line in contents.lines() {
        let numbers = line
            .chars()
            .map(|s| s.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        result.push(numbers);
    }

    result
}
