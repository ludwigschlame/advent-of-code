use std::collections::HashSet;
use std::fs;

/* returns Some(index) of first element after the marker or None, 
 * if no marker of size 'marker_length' with disjunct chars is found */
fn marker_idx(input: &str, marker_length: usize) -> Option<usize> {
    for (idx, window) in input
        .chars()
        .collect::<Vec<char>>()
        .windows(marker_length)
        .enumerate()
    {
        let marker_hash: HashSet<&char> = HashSet::from_iter(window.iter());
        if marker_hash.len() == marker_length {
            return Some(marker_length + idx);
        }
    }
    None
}

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* PART ONE */
    let result_one = marker_idx(&input, 4);

    match result_one {
        Some(result_one) => println!(
            "PART ONE: {} characters need to be processed before the first start-of-packet marker is detected.",
            result_one
        ),
        None => println!("PART ONE: No start-of-packet marker detected."),
    }

    /* PART TWO */
    let result_two = marker_idx(&input, 14);

    match result_two {
        Some(result_two) => println!(
            "PART TWO: {} characters need to be processed before the first start-of-message marker is detected.",
            result_two
        ),
        None => println!("PART TWO: No start-of-packet marker detected."),
    }

    /* VERIFIED RESULTS */
    assert_eq!(
        1275,
        result_one.expect("Error: Result for part one doesn't match expected output.")
    );
    assert_eq!(
        3605,
        result_two.expect("Error: Result for part two doesn't match expected output.")
    );
}
