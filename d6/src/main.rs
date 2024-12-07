use indexmap::IndexMap;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Debug for Coords {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) {:?}", self.y, self.x, self.direction)
    }
}

type VisitedMap = Vec<Vec<Option<HashMap<Direction, bool>>>>;

struct VisitMap(IndexMap<(usize, usize), VisitedDirection>);

impl VisitMap {
    fn new() -> Self {
        Self(IndexMap::new())
    }

    fn visit(&mut self, pos: (usize, usize), direction: Direction) {
        self.0
            .entry(pos)
            .or_insert_with(VisitedDirection::default)
            .visit(direction);
    }

    fn insert(&mut self, pos: (usize, usize), direction: VisitedDirection) {
        self.0.insert(pos, direction);
    }

    fn get(&self, pos: (usize, usize)) -> Option<&VisitedDirection> {
        self.0.get(&pos)
    }

    fn contains_key(&self, pos: (usize, usize)) -> bool {
        self.0.contains_key(&pos)
    }
}

impl Debug for VisitMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

fn main() {
    let (mut grid, coords) = read_map();

    part_1(&grid, coords);

    let start = std::time::Instant::now();
    let count = part_2_brute_force(&mut grid, coords);
    let duration = start.elapsed();

    println!("Part 2 - brute force:\n {}, Time: {:?}", count, duration);

    let start = std::time::Instant::now();
    let count = part_2_skip_unreachable(&mut grid, coords);
    let duration = start.elapsed();

    println!(
        "Part 2 - skip unreachable:\n {}, Time: {:?}",
        count, duration
    );

    let start = std::time::Instant::now();
    let count = part_2_dynamic_rollout(&mut grid, coords);
    let duration = start.elapsed();

    println!(
        "Part 2 - jump to obstacle:\n {}, Time: {:?}",
        count, duration
    );
}

fn part_1(grid: &Vec<Vec<bool>>, coords: Coords) {
    let mut coords = coords;
    let mut visited: VisitedMap = vec![vec![None; grid[0].len()]; grid.len()];

    let mut map = HashMap::new();
    map.insert(coords.direction, true);
    visited[coords.y][coords.x] = Some(map);

    loop {
        let (next_coords, is_inside, _) = walk(&grid, coords, &mut visited);
        if !is_inside {
            break;
        }
        coords = next_coords;
    }

    let mut count = 0;
    for row in visited {
        for cell in row {
            if cell.is_some() {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}

#[derive(Default)]
struct VisitedDirection {
    right: bool,
    down: bool,
    left: bool,
    up: bool,
}

impl VisitedDirection {
    fn visit(&mut self, direction: Direction) {
        if direction == Direction::Right {
            self.right = true;
        }
        if direction == Direction::Down {
            self.down = true;
        }
        if direction == Direction::Left {
            self.left = true;
        }
        if direction == Direction::Up {
            self.up = true;
        }
    }

    fn iter(&self) -> impl Iterator<Item = Direction> {
        let mut directions = Vec::new();

        if self.up {
            directions.push(Direction::Up);
        }
        if self.down {
            directions.push(Direction::Down);
        }
        if self.left {
            directions.push(Direction::Left);
        }
        if self.right {
            directions.push(Direction::Right);
        }

        directions.into_iter()
    }
}

impl Debug for VisitedDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;
        if self.up {
            write!(f, "up ")?;
        }
        if self.down {
            write!(f, "down ")?;
        }
        if self.left {
            write!(f, "left ")?;
        }
        if self.right {
            write!(f, "right ")?;
        }
        write!(f, "]\n")
    }
}

// For each step, check if putting an obstruction at the next position would result
// in a loop.
fn part_2_dynamic_rollout(grid: &mut Vec<Vec<bool>>, start_coords: Coords) -> usize {
    let mut obstruction_map: HashMap<(usize, usize), bool> = HashMap::new();

    let mut coords = start_coords;

    let height = grid.len();
    let width = grid[0].len();

    let mut visited: VisitMap = VisitMap::new();

    loop {
        visited.visit((coords.y, coords.x), coords.direction);

        let (next_coords, is_inside) = step(width, height, &coords);
        if !is_inside {
            break;
        }

        if is_obstacle(grid, &next_coords) {
            coords = turn(&coords);
            continue;
        }

        // Can't put an obstacle at the start position
        if coords.x == start_coords.x && coords.y == start_coords.y {
            coords = next_coords;
            continue;
            // visited.visit((next_coords.y, next_coords.x), next_coords.direction);
        }

        // If we've already put an obstacle here, skip
        if obstruction_map.contains_key(&(next_coords.y, next_coords.x)) {
            coords = next_coords;
            continue;
        }

        // if the obstruction will be on a position we've already visited (but where
        // it did not result in a loop, and we therefore did not store it), it is
        // invalid, since we would never have got to where we are.
        if visited.contains_key((next_coords.y, next_coords.x)) {
            coords = next_coords;
            continue;
        }

        // place an obstacle at the next position
        grid[next_coords.y][next_coords.x] = true;

        // simulate walking from the position before we took a step
        if walk_until_loop(&grid, coords) {
            obstruction_map.insert((next_coords.y, next_coords.x), true);
        }

        // remove the obstacle from the next position
        grid[next_coords.y][next_coords.x] = false;

        coords = next_coords;
    }

    obstruction_map.len()
}

fn step_back(x: usize, y: usize, direction: Direction) -> Coords {
    match direction {
        Direction::Right => Coords {
            x: x - 1,
            y,
            direction,
        },
        Direction::Down => Coords {
            x,
            y: y - 1,
            direction,
        },
        Direction::Left => Coords {
            x: x + 1,
            y,
            direction,
        },
        Direction::Up => Coords {
            x,
            y: y + 1,
            direction,
        },
    }
}

// 1. Run first pass and store all visited positions
// 2. For each possible obstacle position, check if it is visited in the first pass
// 3. If not, skip it since, we'll never hit it
fn part_2_skip_unreachable(grid: &mut Vec<Vec<bool>>, start_coords: Coords) -> usize {
    let mut obstruction_map: HashMap<(usize, usize), bool> = HashMap::new();

    let mut coords = start_coords;

    let height = grid.len();
    let width = grid[0].len();

    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();

    loop {
        let (next_coords, is_inside) = step(width, height, &coords);
        if !is_inside {
            break;
        }
        if is_obstacle(grid, &next_coords) {
            coords = turn(&coords);
            continue;
        }

        visited.insert((next_coords.y, next_coords.x), true);
        coords = next_coords;
    }

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] {
                continue;
            }

            if !visited.contains_key(&(y, x)) {
                continue;
            }

            grid[y][x] = true;

            if walk_until_loop(&grid, start_coords) {
                obstruction_map.insert((y, x), true);
            }
            grid[y][x] = false;
        }
    }

    obstruction_map.len()
}

fn part_2_brute_force(grid: &mut Vec<Vec<bool>>, start_coords: Coords) -> usize {
    let mut obstruction_map: HashMap<(usize, usize), bool> = HashMap::new();

    let height = grid.len();
    let width = grid[0].len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] {
                continue;
            }

            grid[y][x] = true;

            if walk_until_loop(&grid, start_coords) {
                obstruction_map.insert((y, x), true);
            }
            grid[y][x] = false;
        }
    }

    obstruction_map.len()
}

fn walk_until_loop(grid: &Vec<Vec<bool>>, start_coords: Coords) -> bool {
    let mut coords = start_coords;
    let mut visited: HashMap<Coords, bool> = HashMap::new();

    loop {
        let (next_coords, is_inside) = step(grid[0].len(), grid.len(), &coords);
        if !is_inside {
            return false;
        }

        if is_obstacle(grid, &next_coords) {
            coords = turn(&coords);
            continue;
        }

        if visited.contains_key(&next_coords) {
            return true;
        }

        visited.insert(next_coords, true);
        coords = next_coords;
    }
}

/// Returns the next coordinates, whether we're still inside the grid, and whether
/// we hit an obstacle.
fn walk(
    grid: &Vec<Vec<bool>>,
    start_coords: Coords,
    visited: &mut VisitedMap,
) -> (Coords, bool, bool) {
    let height = grid.len();
    let width = grid[0].len();

    let (next_coords, is_inside) = step(width, height, &start_coords);

    if !is_inside {
        return (next_coords, false, false);
    }

    if is_obstacle(grid, &next_coords) {
        return (turn(&start_coords), true, true);
    }

    let mut current_map = visited[next_coords.y][next_coords.x]
        .as_ref()
        .map_or_else(HashMap::new, |m| m.clone());

    current_map.insert(next_coords.direction, true);
    visited[next_coords.y][next_coords.x] = Some(current_map);

    (next_coords, true, false)
}

fn turn(coords: &Coords) -> Coords {
    Coords {
        x: coords.x,
        y: coords.y,
        direction: match coords.direction {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        },
    }
}

fn step(width: usize, height: usize, coords: &Coords) -> (Coords, bool) {
    match coords.direction {
        Direction::Right => {
            let next_inside = coords.x + 1 < width;
            let next_coords = Coords {
                x: coords.x + 1,
                y: coords.y,
                direction: coords.direction,
            };
            (if next_inside { next_coords } else { *coords }, next_inside)
        }
        Direction::Down => {
            let next_inside = coords.y + 1 < height;
            let next_coords = Coords {
                x: coords.x,
                y: coords.y + 1,
                direction: coords.direction,
            };
            (if next_inside { next_coords } else { *coords }, next_inside)
        }
        Direction::Left => {
            let next_inside = coords.x > 0;
            if next_inside {
                let next_coords = Coords {
                    x: coords.x - 1,
                    y: coords.y,
                    direction: coords.direction,
                };
                (next_coords, true)
            } else {
                (*coords, false)
            }
        }
        Direction::Up => {
            let next_inside = coords.y > 0;
            if next_inside {
                let next_coords = Coords {
                    x: coords.x,
                    y: coords.y - 1,
                    direction: coords.direction,
                };
                (next_coords, true)
            } else {
                (*coords, false)
            }
        }
    }
}

fn is_obstacle(grid: &Vec<Vec<bool>>, coords: &Coords) -> bool {
    grid[coords.y][coords.x]
}

fn read_map() -> (Vec<Vec<bool>>, Coords) {
    let mut file = File::open("data.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut coords: Coords = Coords {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };

    for (y, line) in data.lines().enumerate() {
        let mut row: Vec<bool> = Vec::new();

        for (x, c) in line.chars().enumerate() {
            row.push(c == '#');
            if c == '>' {
                coords = Coords {
                    x,
                    y,
                    direction: Direction::Right,
                };
                continue;
            }
            if c == 'v' {
                coords = Coords {
                    x,
                    y,
                    direction: Direction::Down,
                };
                continue;
            }
            if c == '<' {
                coords = Coords {
                    x,
                    y,
                    direction: Direction::Left,
                };
                continue;
            }
            if c == '^' {
                coords = Coords {
                    x,
                    y,
                    direction: Direction::Up,
                };
                continue;
            }
        }
        grid.push(row);
    }

    (grid, coords)
}
