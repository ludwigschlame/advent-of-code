use std::fs;

fn get_tree_info(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> (bool, i32) {
    let dimension = forest.len();
    let mut tree_is_visible = false;
    let mut viewing_score = 1;
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),  /* right */
        (-1, 0), /* left */
        (0, 1),  /* down */
        (0, -1), /* up */
    ];

    /* from given tree look in all directions to see how far you can look (viewing distance)
     * and if you can see outside the forest (visibility from the outside) */
    'outer: for direction in directions {
        let mut x_look = x as i32 + direction.0;
        let mut y_look = y as i32 + direction.1;
        let mut viewing_distance = 1;

        /* while we are inside the forest bounds */
        while (x_look >= 0 && x_look < dimension as i32)
            && (y_look >= 0 && y_look < dimension as i32)
        {
            if forest[x_look as usize][y_look as usize] >= forest[x][y] {
                /* tree is not visible from this direction -> try the next
                 * also locks in the viewing distance in this direction */
                viewing_score *= viewing_distance;
                continue 'outer;
            }

            x_look += direction.0;
            y_look += direction.1;
            viewing_distance += 1;
        }
        /* reached the edge of the forest aka. the tree is visible
         * decrease visibility distance because we are one step outside the forest*/
        tree_is_visible = true;
        viewing_score *= viewing_distance - 1;
    }

    /* return if the tree is visible from the outside & it's scenic score */
    (tree_is_visible, viewing_score)
}

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* SHARED */
    let forest = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree_height| tree_height.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let dimension = forest.len(); /* forest is a square */

    let mut num_trees_visible = 0;
    let mut highest_scenic_score = 1;

    for x in 0..dimension {
        for y in 0..dimension {
            let tree_info = get_tree_info(&forest, x, y);

            if tree_info.0 {
                /* tree is visible from outside */
                num_trees_visible += 1;
            }

            highest_scenic_score = highest_scenic_score.max(tree_info.1);
        }
    }

    let result_one = num_trees_visible;
    let result_two = highest_scenic_score;

    println!(
        "PART ONE: {} trees are visible from outside the grid.",
        result_one
    );

    println!(
        "PART TWO: {} is the highest scenic score possible.",
        result_two
    );

    /* VERIFIED RESULTS */
    assert_eq!(result_one, 1845);
    assert_eq!(result_two, 230112);
}
