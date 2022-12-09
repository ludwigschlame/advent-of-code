use std::collections::HashSet;
use std::fs;

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* SHARED */
    let motions = input
        .trim()
        .lines()
        .map(|line| {
            let motion = line
                .split_once(' ')
                .unwrap_or_else(|| panic!("Expected: &str including ' ', found: '{}'", line));
            let direction = motion.0;
            let amount = motion
                .1
                .parse::<i32>()
                .unwrap_or_else(|err| panic!("Expected: i32, found:  '{}': {}", motion.1, err));
            (direction, amount)
        })
        .collect::<Vec<(&str, i32)>>();

    /* PART ONE */
    let rope_length = 2;
    let result_one = num_tail_positions(rope_length, &motions);
    println!("PART ONE: {result_one} positions where visited by the tail of the rope with {rope_length} knots.");

    /* PART TWO */
    let rope_length = 10;
    let result_two = num_tail_positions(rope_length, &motions);
    println!("PART TWO: {result_two} positions where visited by the tail of the rope with {rope_length} knots.");

    /* VERIFIED RESULTS */
    assert_eq!(result_one, 6376);
    assert_eq!(result_two, 2607);
}

fn num_tail_positions(rope_length: usize, motions: &Vec<(&str, i32)>) -> usize {
    if rope_length == 0 {
        panic!("length of rope must be at least one");
    }
    let mut rope = vec![(0, 0); rope_length];
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    for motion in motions {
        for _ in 0..motion.1 {
            update_rope(&mut rope, motion.0);
            visited_positions.insert(*rope.last().unwrap());
        }
    }

    visited_positions.len()
}

fn update_rope(rope: &mut Vec<(i32, i32)>, direction: &str) {
    /* move head according to direction from input */
    match direction {
        "R" => rope[0].0 += 1,
        "L" => rope[0].0 -= 1,
        "U" => rope[0].1 += 1,
        "D" => rope[0].1 -= 1,
        other => panic!("invalid direction '{other}'"),
    };

    /* update following knots according to their position relative to the knot in front of them */
    for i in 1..rope.len() {
        let delta = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);

        if delta.0.abs() > 1 || delta.1.abs() > 1 {
            rope[i] = (rope[i].0 + delta.0.signum(), rope[i].1 + delta.1.signum());
        }
    }
}
