use std::fs;

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* SHARED */
    let mut calorie_counts = input
        .split("\n\n")
        .map(|elf_items| {
            elf_items
                .lines()
                .map(|item| {
                    item.parse::<u32>()
                        .unwrap_or_else(|err| panic!("Error parsing number: {}", err))
                })
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    calorie_counts.sort_by(|item0, item1| item1.cmp(item0)); /* sort reversed */

    /* PART ONE */
    let top_calories = calorie_counts
        .first()
        .expect("Error getting first element of vec");

    println!(
        "PART ONE: The elf carrying the most carries {} calories.",
        top_calories
    );

    /* PART TWO */
    let top_three_calories = calorie_counts.iter().take(3).sum::<u32>();

    println!(
        "PART TWO: The top three elfs carry a combined amount of {} calories.",
        top_three_calories
    )
}
