use std::{collections::HashMap, fs};

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* SHARED */
    let mut current_path: Vec<&str> = vec![];
    let mut directory_sizes_single: HashMap<String, usize> = HashMap::new();
    let mut directory_sizes_total: Vec<usize>;

    /* split input into parts corresponding to a single command and its result */
    let parts = input
        .trim()
        .split('$')
        .filter(|string| !string.is_empty())
        .collect::<Vec<&str>>();

    /* parse input into HashMap */
    for part in parts {
        let mut lines = part.lines();

        let command = lines
            .next()
            .expect("Error: empty command")
            .split_whitespace()
            .collect::<Vec<&str>>();
        let command_type = command[0];

        match command_type {
            "ls" => {
                let file_size_sum = lines
                    .map(|file| file.split_once(' ').expect("Error: bad format"))
                    .map(|file| match file.0 {
                        "dir" => 0,
                        file_size => file_size.parse::<usize>().expect("Error: NaN"),
                    })
                    .sum::<usize>();

                directory_sizes_single.insert(current_path.join("/"), file_size_sum);
            }
            "cd" => match command[1] {
                ".." => {
                    current_path.pop();
                }
                "/" => current_path = vec!["/"],
                dir => current_path.push(dir),
            },
            other => panic!("Error: bad command '{}'", other),
        }
    }

    /* calculate size of directory and all its sub-directories */
    let mut raw_sizes = directory_sizes_single
        .iter()
        .collect::<Vec<(&String, &usize)>>();
    directory_sizes_total = vec![0; raw_sizes.len()];

    /* Sort by path length (descending)
     * if one path is a substring of another, it adds the other path's size to itself
     * paths are rooted with "//" at the start to avoid ambiguity */
    raw_sizes.sort_by(|dir1, dir2| dir2.0.len().cmp(&dir1.0.len()));
    for i in 0..raw_sizes.len() {
        for j in i..raw_sizes.len() {
            if (raw_sizes[i].0).contains(raw_sizes[j].0) {
                directory_sizes_total[j] += raw_sizes[i].1;
            }
        }
    }

    /* PART ONE */
    const DIRECTORY_SIZE_THRESHOLD: usize = 100_000;

    let result_one = directory_sizes_total
        .clone()
        .iter()
        .filter(|dir_size| **dir_size <= DIRECTORY_SIZE_THRESHOLD)
        .sum::<usize>();

    println!(
        "PART ONE: {} is the sum of the sizes of directories smaller than {}.",
        result_one, DIRECTORY_SIZE_THRESHOLD
    );

    /* PART TWO */
    const DISK_SPACE_TOTAL: usize = 70_000_000;
    const DISK_SPACE_REQUIRED: usize = 30_000_000;

    let disk_space_used = *directory_sizes_total.last().unwrap();
    let disk_space_free = DISK_SPACE_TOTAL - disk_space_used;
    let disk_space_needed = DISK_SPACE_REQUIRED - disk_space_free;

    let result_two = *directory_sizes_total
        .iter()
        .find(|folder_size| **folder_size >= disk_space_needed)
        .expect("Error: there is no folder which's deletion would free up enough space");

    println!("PART TWO: {} is the size of the smallest directory which, if deleted, would free up enough space.", result_two);

    /* VERIFIED RESULTS */
    assert_eq!(result_one, 1447046);
    assert_eq!(result_two, 578710);
}
