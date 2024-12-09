use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_data();

    let result = part_1(&data);
    println!("Part 1: {}", result);

    let result_2 = part_2(&data);
    println!("Part 2: {}", result_2);
}

fn part_1(data: &[usize]) -> i64 {
    let disk = compute_disk_layout(data);
    let compacted_disk = compact_disk(&disk);
    compute_checksum(&compacted_disk)
}

fn part_2(data: &[usize]) -> i64 {
    let disk = compute_disk_layout(data);
    let compacted_disk = compact_disk_2(&disk, data);
    compute_checksum(&compacted_disk)
}

fn compact_disk_2(disk: &[i32], disk_layout: &[usize]) -> Vec<i32> {
    let mut compacted_disk: Vec<i32> = disk.to_vec().clone();

    let mut block_map: Vec<(usize, usize, usize)> = vec![]; // (disk_index, size, block_id)
    let mut free_space_map: Vec<(usize, usize)> = vec![]; // (disk_index, size)

    let mut block_id = 0;
    let mut disk_cursor = 0;
    for i in 0..disk_layout.len() {
        if i % 2 == 0 {
            block_map.push((disk_cursor, disk_layout[i], block_id));
            block_id += 1;
        } else {
            free_space_map.push((disk_cursor, disk_layout[i]));
        }
        disk_cursor += disk_layout[i];
    }

    for (block_idx, block_size, block_id) in block_map.iter().rev() {
        for (i, (free_space_idx, free_space_size)) in free_space_map.iter().enumerate() {
            if *free_space_size == 0 {
                continue;
            }
            if free_space_idx >= block_idx {
                break;
            }

            if *block_size > *free_space_size {
                continue;
            }

            // copy block to compacted disk
            let block = vec![*block_id as i32; *block_size];

            compacted_disk[*free_space_idx..*free_space_idx + *block_size].copy_from_slice(&block);
            compacted_disk[*block_idx..*block_idx + *block_size].fill(-1);
            free_space_map[i] = (free_space_idx + *block_size, *free_space_size - *block_size);
            break;
        }
    }

    compacted_disk
}

fn compute_disk_layout(data: &[usize]) -> Vec<i32> {
    let mut block_index = 0;
    let mut current_index = 0;

    let disk_size = data.iter().sum::<usize>();

    let mut disk: Vec<i32> = vec![-1; disk_size]; // -1 represents free space
    for i in 0..data.len() {
        let value = data[i];
        let block_size = value as usize;

        // Value represents block size
        if i % 2 == 0 {
            let block = vec![block_index as i32; block_size];
            disk[current_index..current_index + block_size].copy_from_slice(&block);
            current_index += block_size;
            block_index += 1;
            continue;
        }

        // Value represents free space
        let block = vec![-1; block_size];
        disk[current_index..current_index + block_size].copy_from_slice(&block);
        current_index += block_size;
    }

    disk
}

fn compact_disk(disk: &[i32]) -> Vec<i32> {
    let compacted_disk_size = disk.iter().filter(|x| **x != -1).count();
    let mut compacted_disk: Vec<i32> = vec![-1; compacted_disk_size];

    let mut idx = 0;
    let mut wrap_index = disk.len() - 1;

    loop {
        let value = disk[idx];
        if idx == compacted_disk_size {
            break;
        }
        if value != -1 {
            compacted_disk[idx] = value;
            idx += 1;
            continue;
        }

        let wrap_value = disk[wrap_index];
        if wrap_value == -1 {
            wrap_index -= 1;
            continue;
        }

        if idx == wrap_index {
            break;
        }
        // free space
        compacted_disk[idx] = disk[wrap_index];

        idx += 1;
        wrap_index -= 1;
    }

    compacted_disk
}

fn visualize_disk(disk: &[i32]) {
    println!(
        "{:?}",
        disk.iter()
            .map(|x| if *x == -1 {
                ".".to_string()
            } else {
                x.to_string()
            })
            .collect::<Vec<String>>()
            .join("")
    );
}

fn compute_checksum(data: &[i32]) -> i64 {
    let mut sum: i64 = 0;
    for i in 0..data.len() {
        if data[i] == -1 {
            continue;
        }
        sum += data[i] as i64 * i as i64;
    }
    sum
}

fn read_data() -> Vec<usize> {
    let mut file = File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}
