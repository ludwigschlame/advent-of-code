use std::fs;

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* SHARED */
    let games = input
        .trim()
        .lines()
        .map(|game| {
            game.split_whitespace()
                /* 0: rock/lose, 1: paper/draw, 2: scissors/win */
                .map(|rps_move| match rps_move {
                    "A" | "X" => 0,
                    "B" | "Y" => 1,
                    "C" | "Z" => 2,
                    _ => panic!("Error: invalid move"),
                })
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    /* PART ONE */
    let score = games
        .iter()
        .map(|instruction| {
            let game_result = (3 + 1 + instruction[1] - instruction[0]) % 3; /* 0: loss, 1: draw, 2: win */
            game_result * 3 + (instruction[1] + 1) /* calc score of the game */
        })
        .sum::<u32>();

    println!("PART ONE: {} would be my total score.", score);

    /* PART TWO */
    let score = games
        .iter()
        .map(|instruction| {
            let my_move = (instruction[0] + instruction[1] + 2) % 3; /* 0: rock, 1: paper, 2: scissors */
            instruction[1] * 3 + my_move + 1 /* calc score of the game */
        })
        .sum::<u32>();

    println!("PART TWO: {} would be my total score.", score);
}
