use std::env;
use std::fs::File;
use std::io::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() > 3 {
        println!(
            "Too many arguments! Please input only the id of the task to run. Ex: 1a, 3B, 6A, 22b"
        );
        return;
    } else if args.len() < 3 {
        println!("Too few arguments! Please input the id of the task to run. Ex: 1a, 3B, 6A, 22b");
        return;
    }
    // Get task id
    println!("Running task {}...", args[2]);
    let task_id = &env::args().nth(2).unwrap().to_lowercase();

    // Check if task id is valid
    // This was only done in bulk because GitHub Copilot is crazy convenient
    if !check_if_task_id_is_valid(task_id) {
        println!("Invalid task id! Please input the id of the task to run. Ex: 1a, 3B, 6A, 22b");
        return;
    }

    // Select task
    match task_id.as_str() {
        "1a" => task_1a(),
        "1b" => task_1b(),
        "2a" => task_2a(),
        _ => println!("Task not implemented yet!"),
    }
}

fn task_1a() {
    let filepath = "data/day_1_input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    // Iterate over each line
    for line in reader.lines() {
        let line = line.unwrap();
        let num = line.parse::<i32>().unwrap();
        numbers.push(num);
    }

    let depth_increases = count_depth_increases(numbers);
    println!("{}", depth_increases);
}

fn task_1b() {
    let filepath = "data/day_1_input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    // Iterate over each line
    for line in reader.lines() {
        let line = line.unwrap();
        let num = line.parse::<i32>().unwrap();
        numbers.push(num);
    }

    let depth_increases = count_depth_increases_with_sliding_window(numbers, 3);
    println!("{}", depth_increases);
}

fn task_2a() {
    let filepath = "data/day_2_input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut orders: Vec<SubmarineMoveOrder> = Vec::new();

    // Iterate over each line
    for line in reader.lines() {
        let line = line.unwrap();
        // Parse the line as a slice of the String
        let order = parse_submarine_move_order(&line[..]);
        orders.push(order);
    }
}

// Returns true if the task id is valid
fn check_if_task_id_is_valid(task_id: &String) -> bool {
    task_id != "1a"
        && task_id != "1b"
        && task_id != "2a"
        && task_id != "2b"
        && task_id != "3a"
        && task_id != "3b"
        && task_id != "4a"
        && task_id != "4b"
        && task_id != "5a"
        && task_id != "5b"
        && task_id != "6a"
        && task_id != "6b"
        && task_id != "7a"
        && task_id != "7b"
        && task_id != "8a"
        && task_id != "8b"
        && task_id != "9a"
        && task_id != "9b"
        && task_id != "10a"
        && task_id != "10b"
        && task_id != "11a"
        && task_id != "11b"
        && task_id != "12a"
        && task_id != "12b"
        && task_id != "13a"
        && task_id != "13b"
        && task_id != "14a"
        && task_id != "14b"
        && task_id != "15a"
        && task_id != "15b"
        && task_id != "16a"
        && task_id != "16b"
        && task_id != "17a"
        && task_id != "17b"
        && task_id != "18a"
        && task_id != "18b"
        && task_id != "19a"
        && task_id != "19b"
        && task_id != "20a"
        && task_id != "20b"
        && task_id != "21a"
        && task_id != "21b"
        && task_id != "22a"
        && task_id != "22b"
        && task_id != "23a"
        && task_id != "23b"
        && task_id != "24a"
        && task_id != "24b"
        && task_id != "25a"
        && task_id != "25b"
}

fn count_depth_increases(input: Vec<i32>) -> i32 {
    let mut increases = -1; // Start at -1 to account for the first value increasing from 0
    let mut last_depth = 0;
    for depth in input {
        if depth > last_depth {
            increases += 1;
        }
        last_depth = depth;
    }
    increases
}

fn count_depth_increases_with_sliding_window(input: Vec<i32>, window_size: usize) -> i32 {
    let mut increases = -1; // Start at -1 to account for the first value increasing from 0
    let mut last_depth = 0;
    let mut counter = 0;

    for (i, depth) in input.iter().enumerate() {
        // Check if a new window is possible
        counter += 1;
        if i + window_size - 1 >= input.len() || counter > 100000000 {
            break;
        }
        // Create sliding window
        let mut window = Vec::new();
        for n in 0..window_size {
            let item = i + n;
            window.push(input[item]);
        }
        let sum: i32 = window.iter().sum();

        // Check if the window is increased from previous
        if sum > last_depth {
            increases += 1;
        }

        // Always save last depth
        last_depth = sum;
    }

    increases
}

// Depth, Horizontal Position
struct SubmarinePosition(i32, i32);

// Direction, Distance
struct SubmarineMoveOrder(SubmarineMoveDirection, i32);

enum SubmarineMoveDirection {
    Up,
    Down,
    Forward,
}

fn parse_submarine_move_order(input: &str) -> SubmarineMoveOrder {
    let mut split = input.split_whitespace();
    let direction = match split.next().unwrap() {
        "up" => SubmarineMoveDirection::Up,
        "down" => SubmarineMoveDirection::Down,
        "forward" => SubmarineMoveDirection::Forward,
        _ => panic!("Invalid direction!"),
    };
    let distance = split.next().unwrap().parse::<i32>().unwrap();
    SubmarineMoveOrder(direction, distance)
}

fn move_submarine(
    position: SubmarinePosition,
    move_order: SubmarineMoveOrder,
) -> SubmarinePosition {
    match move_order.0 {
        SubmarineMoveDirection::Up => SubmarinePosition(position.0 + move_order.1, position.1),
        SubmarineMoveDirection::Down => SubmarinePosition(position.0 - move_order.1, position.1),
        SubmarineMoveDirection::Forward => SubmarinePosition(position.0, position.1 + move_order.1),
    }
}
/*
fn move_forward(position: SubmarinePosition, amount: i32) -> SubmarinePosition {
    let mut new_position = position;
    new_position.horizontal += amount;
    new_position
}

fn move_down(position: SubmarinePosition, amount: i32) -> SubmarinePosition {
    let mut new_position = position;
    new_position.depth += amount;
    new_position
}

fn move_up(position: SubmarinePosition, amount: i32) -> SubmarinePosition {
    let mut new_position = position;
    new_position.depth -= amount;
    new_position
}*/
