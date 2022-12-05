use std::fs;

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* SHARED */
    let input_split = input.split("\n\n").collect::<Vec<&str>>(); /* split into [setup, instructions] */
    let starting_condition = input_split.first().expect("Error: input has bad format");
    let rearrangement_procedure = input_split.get(1).expect("Error: input has bad format");

    let num_stacks: usize = starting_condition
        .lines()
        .last()
        .expect("Error: input has bad format")
        .split_whitespace()
        .count();

    let mut supply_stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    /* parse starting condition, reverse to push the top crates last */
    for line in starting_condition.lines().rev() {
        /* ignore stack numbers in last line */
        if line.chars().any(char::is_numeric) {
            continue;
        }

        /* split at four spaces because of 'gaps' in the crate towers */
        let supply_stacks_slice = line.split("    ").flat_map(|continuos_crates| {
            /* split at one space to separate crates without a gap between them */
            continuos_crates.split(' ').map(|bracketed_id| {
                /* empty strings correspond to a 'gap' in the crate towers */
                if bracketed_id.is_empty() {
                    None
                } else {
                    /* index 1 gets the crate id skipping the opening bracket */
                    Some(bracketed_id.chars().collect::<Vec<char>>()[1])
                }
            })
        });

        /* create starting condition in vector of vectors corresponding to the stacks of crates */
        for (stack_idx, crate_id) in supply_stacks_slice.enumerate() {
            if let Some(crate_id) = crate_id {
                supply_stacks[stack_idx].push(crate_id);
            }
        }
    }

    /* parse instructions */
    let instructions = rearrangement_procedure.lines().map(|instruction| {
        instruction
            .split_whitespace()
            .filter_map(|word| word.parse().ok())
            .collect::<Vec<usize>>()
    });

    /* PART ONE */
    /* execute instructions */
    let mut supply_stacks_one = supply_stacks.clone();

    for instruction in instructions.clone() {
        for _ in 0..instruction[0] {
            let tmp_crate = supply_stacks_one[instruction[1] - 1].pop().expect("Error: tried to pop from empty vec");
            supply_stacks_one[instruction[2] - 1].push(tmp_crate);
        }
    }

    /* read result */
    let mut top_crates = "".to_string();
    for stack in supply_stacks_one.iter_mut() {
        top_crates.push(stack.pop().expect("Error: tried to pop from empty vec"));
    }

    assert_eq!(top_crates, "QNHWJVJZW"); /* verified result */
    println!(
        "PART ONE: {} are the crates which will end up on top.",
        top_crates
    );

    /* PART TWO (twist: when moving multiple crates, their order is no longer reversed) */
    /* execute instructions */
    let mut supply_stacks_two = supply_stacks.clone();

    for instruction in instructions.clone() {
        let mut tmp_crates = vec![];
        for _ in 0..instruction[0] {
            tmp_crates.push(supply_stacks_two[instruction[1] - 1].pop().expect("Error: tried to pop from empty vec"));
        }
        tmp_crates.reverse();
        supply_stacks_two[instruction[2] - 1].append(&mut tmp_crates);
    }

    /* read result */
    let mut top_crates = "".to_string();
    for stack in supply_stacks_two.iter_mut() {
        top_crates.push(stack.pop().expect("Error: tried to pop from empty vec"));
    }

    assert_eq!(top_crates, "BPCZJLFJW"); /* verified result */
    println!(
        "PART TWO: {} are the crates which will end up on top.",
        top_crates
    );
}
