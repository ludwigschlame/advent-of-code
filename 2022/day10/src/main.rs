use std::fs;

struct Crt {
    width: usize,
    pixel_states: (char, char),
    display: Vec<char>,
}

impl Crt {
    fn new(width: usize, height: usize, pixel: (char, char)) -> Self {
        let display = vec![pixel.0; width * height];
        Self {
            width,
            pixel_states: pixel,
            display,
        }
    }

    fn draw(&self) {
        print!("\n|");
        for _ in 0..self.width {
            print!("-");
        }
        for (idx, pixel) in self.display.iter().enumerate() {
            if idx % self.width == 0 {
                print!("|\n|")
            }
            print!("{pixel}");
        }
        print!("|\n|");
        for _ in 0..self.width {
            print!("-");
        }
        println!("|");
    }

    fn set_pixel(&mut self, index: usize) {
        self.display[index] = self.pixel_states.1;
    }

    fn width(&self) -> usize {
        self.width
    }
}

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    /* SHARED */
    let mut crt = Crt::new(40, 6, (' ', '0'));
    let mut sprite_position: i32 = 1;

    let mut signal_strength: i32 = 0;
    let measurement_start: usize = 20;
    let measurement_interval: usize = 40;

    let commands = input
        .lines()
        .flat_map(|line| line.split_whitespace().collect::<Vec<&str>>());

    for (cycle, command) in commands.enumerate() {
        /* PART ONE */
        if measurement_start.abs_diff(cycle + 1) % measurement_interval == 0 {
            // cycle number offset by one due to instructions
            signal_strength += sprite_position * (cycle + 1) as i32;
        }

        /* PART TWO */
        if sprite_position.abs_diff((cycle % crt.width()) as i32) <= 1 {
            crt.set_pixel(cycle);
        }

        match command {
            "noop" => continue,
            "addx" => continue,
            val => {
                sprite_position += val
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("Expected: 'i32', found: '{}'", val))
            }
        }
    }

    let result_one = signal_strength;

    println!(
        "PART ONE: {} is the sum of the signal strength measurements.",
        result_one
    );
    print!("PART TWO:");
    crt.draw();

    /* VERIFIED RESULTS */
    assert_eq!(result_one, 13480);
    // assert_eq!(result_two, 0);
    /* part two should read "EGJBGCFK" in large letters on the crt display */
}
