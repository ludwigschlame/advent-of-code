use std::fs;

fn item_priority(item: char) -> u8 {
    match item as u8 {
        65..=90 => item as u8 - 64 + 26, /* uppercase letters */
        97..=122 => item as u8 - 96,     /* lowercase letters */
        _ => panic!("Error: invalid item"),
    }
}

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* SHARED */
    let mut lines = input.trim().lines().map(|line| line.trim());

    /* PART ONE */
    let mut priority_sum: u32 = 0;

    for rucksack in lines.clone() {
        let compartment_size = rucksack.len() / 2;
        let rucksack_compartments = (&rucksack[..compartment_size], &rucksack[compartment_size..]);

        for item in rucksack_compartments.0.chars() {
            if rucksack_compartments.1.find(item).is_some() {
                priority_sum += item_priority(item) as u32;
                break;
            }
        }
    }

    assert_eq!(priority_sum, 8349); /* verified result */
    println!(
        "PART ONE: {} is the sum of the incorrectly sorted items' priorities",
        priority_sum
    );

    /* PART TWO */
    let mut priority_sum = 0;

    for _ in 0..lines.clone().count() {
        let elf_group;
        if let (Some(elf1), Some(elf2), Some(elf3)) = (lines.next(), lines.next(), lines.next()) {
            elf_group = (elf1, elf2, elf3);
        } else {
            break;
        }

        for item in elf_group.0.chars() {
            if elf_group.1.find(item).is_some() && elf_group.2.find(item).is_some() {
                priority_sum += item_priority(item) as u32;
                break;
            }
        }
    }

    assert_eq!(priority_sum, 2681); /* verified result */
    println!(
        "PART TWO: {} is the sum of identification badges' priorities",
        priority_sum
    );
}
