use std::fs;

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* SHARED */
    let assignments = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split([',', '-'])
                .map(|interval_endpoint| {
                    interval_endpoint
                        .parse::<u32>()
                        .unwrap_or_else(|err| panic!("Error: NaN: '{}'", err))
                })
                .collect::<Vec<u32>>() /* [left_endpoint_1, right_endpoint_1, left_endpoint_2, right_endpoint_2] */
        });

    /* PART ONE */
    let contained_assignments_count: usize = assignments
        .clone() /* because assignments would be moved by filter() but is used again in part two */
        .filter(|assignment_pair| {
            /* check if both endpoints of either interval are contained in the other */
            (assignment_pair[0]..=assignment_pair[1]).contains(&assignment_pair[2])
                && (assignment_pair[0]..=assignment_pair[1]).contains(&assignment_pair[3])
                || (assignment_pair[2]..=assignment_pair[3]).contains(&assignment_pair[0])
                    && (assignment_pair[2]..=assignment_pair[3]).contains(&assignment_pair[1])
        })
        .count();

    assert_eq!(contained_assignments_count, 462); /* verified result */
    println!(
        "PART ONE: {} is the number of fully contained assignments.",
        contained_assignments_count
    );

    /* PART TWO */
    let overlapping_assignments_count: usize = assignments
        .filter(|assignment_pair| {
            /* check if at least one endpoint of either interval is contained in the other */
            (assignment_pair[0]..=assignment_pair[1]).contains(&assignment_pair[2])
                || (assignment_pair[0]..=assignment_pair[1]).contains(&assignment_pair[3])
                || (assignment_pair[2]..=assignment_pair[3]).contains(&assignment_pair[0])
                || (assignment_pair[2]..=assignment_pair[3]).contains(&assignment_pair[1])
        })
        .count();

    assert_eq!(overlapping_assignments_count, 835); /* verified result */
    println!(
        "PART TWO: {} is the number of overlapping assignments.",
        overlapping_assignments_count
    );
}
