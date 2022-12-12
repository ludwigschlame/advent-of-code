use std::collections::VecDeque;
use std::fs;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>, // worry levels of items the monkey is currently holding
    operation: String,    // worry level changes as monkey inspects items
    test: (u64, usize, usize), // if  worry level % test.0 == 0: { test.1 } else: { test.2 }
}

impl Monkey {
    fn new(items: VecDeque<u64>, operation: String, test: (u64, usize, usize)) -> Self {
        Monkey {
            items,
            operation,
            test,
        }
    }

    fn num_items(&self) -> usize {
        self.items.len()
    }

    fn get_worry_level(&mut self) -> u64 {
        let current_worry_level = self.items.pop_front().unwrap();
        self.update_worry_level(current_worry_level)
    }

    fn update_worry_level(&self, worry_level: u64) -> u64 {
        let op = self.operation.split_whitespace().collect::<Vec<&str>>();
        match op[0] {
            "+" => worry_level + op[1].parse::<u64>().unwrap(),

            "*" => match op[1] {
                "old" => worry_level * worry_level,
                other => worry_level * other.parse::<u64>().unwrap(),
            },
            other => panic!("unknown operation: '{other}'"),
        }
    }

    fn test_item(&self, worry_level: u64) -> usize {
        if worry_level % self.test.0 == 0 {
            self.test.1
        } else {
            self.test.2
        }
    }

    fn receive_item(&mut self, worry_level: u64) {
        self.items.push_back(worry_level);
    }
}

fn main() {
    /* SETUP */
    let path = "input";
    let input =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("Error reading '{}': {}", path, err));

    let mut monkeys = parse_input(&input);
    let monkey_count = monkeys.len();

    /* PART ONE */
    let rounds = 20;
    let mut monkeys_part_one = monkeys.clone();
    let mut inspection_count: Vec<usize> = vec![0; monkey_count];

    for _ in 0..rounds {
        for monkey_idx in 0..monkey_count {
            for _ in 0..monkeys_part_one[monkey_idx].num_items() {
                inspection_count[monkey_idx] += 1;
                let worry_level = monkeys_part_one[monkey_idx].get_worry_level() / 3;
                let receiving_monkey = monkeys_part_one[monkey_idx].test_item(worry_level);
                monkeys_part_one[receiving_monkey].receive_item(worry_level);
            }
        }
    }

    inspection_count.sort();
    inspection_count.reverse();

    let result_one: usize = inspection_count.iter().take(2).product();

    println!(
        "PART ONE: {} is the level of monkey business after {} rounds.",
        result_one, rounds
    );

    /* PART TWO */
    let rounds = 10_000;
    let mut inspection_count: Vec<usize> = vec![0; monkey_count];
    // take worry_level modulo the product of all test values
    // -> doesn't change test outcome vut keeps worry_levels in u64
    let modulo = monkeys.iter().map(|monkey| monkey.test.0).product::<u64>();

    for _ in 0..rounds {
        for monkey_idx in 0..monkey_count {
            for _ in 0..monkeys[monkey_idx].num_items() {
                inspection_count[monkey_idx] += 1;
                let worry_level = monkeys[monkey_idx].get_worry_level() % modulo;
                let receiving_monkey = monkeys[monkey_idx].test_item(worry_level);
                monkeys[receiving_monkey].receive_item(worry_level);
            }
        }
    }

    inspection_count.sort();
    inspection_count.reverse();

    let result_two: usize = inspection_count.iter().take(2).product();

    println!(
        "PART TWO: {} is the level of monkey business after {} rounds.",
        result_two, rounds
    );

    /* VERIFIED RESULTS */
    assert_eq!(result_one, 58056);
    assert_eq!(result_two, 15048718170);
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let input_monkeys = input.split("\n\n").collect::<Vec<&str>>();
    let monkey_count = input_monkeys.len();
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(monkey_count);

    for input_monkey in input_monkeys {
        let monkey_lines = input_monkey.split('\n').collect::<Vec<&str>>();
        let starting_items = monkey_lines[1]
            .split("Starting items: ")
            .flat_map(|maybe_items| {
                maybe_items
                    .split(',')
                    .flat_map(|maybe_item| maybe_item.trim().parse::<u64>())
                    .collect::<Vec<u64>>()
            })
            .collect::<VecDeque<u64>>();
        let operation = monkey_lines[2]
            .split("Operation: new = old ")
            .collect::<Vec<&str>>()[1]
            .to_string();
        let test_value = monkey_lines[3]
            .split("Test: divisible by ")
            .flat_map(|maybe_test| maybe_test.parse::<u64>())
            .collect::<Vec<u64>>()[0];
        let test_true = monkey_lines[4]
            .split("throw to monkey ")
            .flat_map(|monkey_id| monkey_id.parse::<usize>())
            .collect::<Vec<usize>>()[0];
        let test_false = monkey_lines[5]
            .split("throw to monkey ")
            .flat_map(|monkey_id| monkey_id.parse::<usize>())
            .collect::<Vec<usize>>()[0];

        monkeys.push(Monkey::new(
            starting_items,
            operation,
            (test_value, test_true, test_false),
        ));
    }
    monkeys
}
