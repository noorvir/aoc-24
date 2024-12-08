use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_data("data.txt");

    let result = part_1(&data);
    println!("Part 1: {}", result);

    let result = part_2(&data);
    println!("Part 2: {}", result);
}

#[derive(Eq, PartialEq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

impl Debug for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn part_1(data: &Vec<Vec<String>>) -> usize {
    let antennas = generate_antennas_map(data);
    let mut anti_nodes_map: HashMap<Coords, bool> = HashMap::new();

    let map_width = data[0].len();
    let map_height = data.len();

    for (_, coords_vec) in antennas {
        for i in 0..coords_vec.len() {
            for j in i + 1..coords_vec.len() {
                let coord_1 = &coords_vec[i];
                let coord_2 = &coords_vec[j];

                let anti_nodes = calc_anti_nodes(coord_1, coord_2, map_width, map_height, Some(1));
                for an in anti_nodes {
                    anti_nodes_map.insert(an, false);
                }
            }
        }
    }

    anti_nodes_map.len()
}

fn part_2(data: &Vec<Vec<String>>) -> usize {
    let antennas = generate_antennas_map(data);

    let mut anti_nodes_map: HashMap<Coords, bool> = HashMap::new();

    let map_width = data[0].len();
    let map_height = data.len();

    for (_, coords_vec) in antennas {
        for i in 0..coords_vec.len() {
            for j in i + 1..coords_vec.len() {
                let coord_1 = &coords_vec[i];
                let coord_2 = &coords_vec[j];

                let anti_nodes = calc_anti_nodes(coord_1, coord_2, map_width, map_height, None);
                for an in anti_nodes {
                    anti_nodes_map.insert(an, false);
                }
            }

            // The anti-nodes also occur at the antenna position for any pair of antennas
            if coords_vec.len() > 1 {
                anti_nodes_map.insert(
                    Coords {
                        x: coords_vec[i].x,
                        y: coords_vec[i].y,
                    },
                    false,
                );
            }
        }
    }
    // visualize_anti_nodes(&anti_nodes_map, map_width, map_height);
    anti_nodes_map.len()
}

fn generate_antennas_map(data: &Vec<Vec<String>>) -> HashMap<String, Vec<Coords>> {
    let mut antennas: HashMap<String, Vec<Coords>> = HashMap::new();

    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == "." {
                continue;
            }

            if !antennas.contains_key(c) {
                antennas.insert(c.clone(), Vec::new());
            }

            antennas.get_mut(c).unwrap().push(Coords { x, y });
        }
    }

    antennas
}

fn visualize_anti_nodes(
    anti_nodes_map: &HashMap<Coords, bool>,
    map_width: usize,
    map_height: usize,
) {
    for y in 0..map_height {
        for x in 0..map_width {
            if anti_nodes_map.contains_key(&Coords { x, y }) {
                print!("{}", "X");
            } else {
                print!("{}", ".");
            }
        }
        println!();
    }
}

// Returns the anti-nodes for a given pair of coordinates.
// The result could be an empty vector if both the anti-nodes fall outside the map (unlikely)
fn calc_anti_nodes(
    coord_1: &Coords,
    coord_2: &Coords,
    map_width: usize,
    map_height: usize,
    max_anti_nodes: Option<usize>,
) -> Vec<Coords> {
    let mut anti_nodes: Vec<Coords> = Vec::new();

    let x1: i32 = coord_1.x as i32;
    let y1: i32 = coord_1.y as i32;
    let x2: i32 = coord_2.x as i32;
    let y2: i32 = coord_2.y as i32;

    let diff_x: i32 = x1 - x2;
    let diff_y: i32 = y1 - y2;

    // first anti-node
    let mut multiplier: usize = 1;
    loop {
        let an_x1: i32 = x1 + diff_x * multiplier as i32;
        let an_y1: i32 = y1 + diff_y * multiplier as i32;

        // Off the map
        if an_x1 < 0 || an_x1 >= map_width as i32 || an_y1 < 0 || an_y1 >= map_height as i32 {
            break;
        }

        let coord = Coords {
            x: an_x1 as usize,
            y: an_y1 as usize,
        };
        anti_nodes.push(coord);

        if let Some(max_anti_nodes) = max_anti_nodes {
            if multiplier >= max_anti_nodes {
                break;
            }
        }
        multiplier += 1;
    }

    // second anti-node
    let mut multiplier: usize = 1;
    loop {
        let an_x2: i32 = x2 - diff_x * multiplier as i32;
        let an_y2: i32 = y2 - diff_y * multiplier as i32;

        if an_x2 < 0 || an_x2 >= map_width as i32 || an_y2 < 0 || an_y2 >= map_height as i32 {
            break;
        }

        let coord = Coords {
            x: an_x2 as usize,
            y: an_y2 as usize,
        };
        anti_nodes.push(coord);

        if let Some(max_anti_nodes) = max_anti_nodes {
            if multiplier >= max_anti_nodes {
                break;
            }
        }
        multiplier += 1;
    }

    anti_nodes
}

fn read_data(path: &str) -> Vec<Vec<String>> {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut results: Vec<Vec<String>> = Vec::new();

    for line in data.lines() {
        let positions: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        results.push(positions);
    }

    results
}
