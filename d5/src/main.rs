use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::Read;

fn main() {
    let (sort_key, data) = read_data();

    part_1(&sort_key, &data);
    part_2(&sort_key, &data);
}

struct SortKey {
    map: HashMap<BTreeSet<i32>, i32>, // for a pair, value is the number that comes first
}

fn gen_sort_fn(sort_key: &SortKey) -> impl Fn(&i32, &i32) -> Ordering {
    |a: &i32, b: &i32| {
        let key = BTreeSet::from([*a, *b]);
        let value = sort_key.map.get(&key).unwrap();

        if value == a {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

fn is_ordered(vec: &Vec<i32>, compare: impl Fn(&i32, &i32) -> Ordering) -> bool {
    for i in 0..vec.len() - 1 {
        if compare(&vec[i], &vec[i + 1]) == Ordering::Greater {
            return false;
        }
    }

    true
}

fn part_2(sort_key: &SortKey, data: &Vec<Vec<i32>>) {
    let mut sum = 0;
    let compare = gen_sort_fn(sort_key);

    for vec in data {
        if !is_ordered(&vec, &compare) {
            let mut vec = vec.clone();
            vec.sort_by(&compare);

            let middle = vec[vec.len() / 2];
            sum += middle;
        }
    }

    println!("Part 2: {}", sum);
}

fn part_1(sort_key: &SortKey, data: &Vec<Vec<i32>>) {
    let mut sum = 0;
    let compare = gen_sort_fn(sort_key);

    for vec in data {
        if is_ordered(&vec, &compare) {
            let middle = vec[vec.len() / 2];
            sum += middle;
        }
    }

    println!("Part 1: {}", sum);
}

fn read_data() -> (SortKey, Vec<Vec<i32>>) {
    let mut file = File::open("data.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut map: HashMap<BTreeSet<i32>, i32> = HashMap::new();

    let mut data_vec = Vec::new();
    let mut is_data_mode = false;

    for line in data.lines() {
        if line == "" {
            is_data_mode = true;
            continue;
        }

        if !is_data_mode {
            let a: i32 = line.split("|").next().unwrap().parse::<i32>().unwrap();
            let b: i32 = line.split("|").nth(1).unwrap().parse::<i32>().unwrap();

            let mut set: BTreeSet<i32> = BTreeSet::new();
            set.insert(a);
            set.insert(b);

            map.insert(set, a);
            continue;
        }

        let values: Vec<i32> = line.split(",").map(|v| v.parse::<i32>().unwrap()).collect();
        data_vec.push(values);
    }

    (SortKey { map }, data_vec)
}
