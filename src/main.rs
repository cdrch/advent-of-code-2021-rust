use std::collections::HashMap;
use std::fs::File;
use std::i32::MAX;
use std::io::*;
use std::{convert, env};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 2 {
        println!(
            "Too many arguments! Please input only the id of the task to run. Ex: 1a, 3B, 6A, 22b"
        );
        return;
    } else if args.len() < 2 {
        println!("Too few arguments! Please input the id of the task to run. Ex: 1a, 3B, 6A, 22b");
        return;
    }
    // Get task id
    println!("Attempting to run task {}...", args[1]);
    let task_id = &env::args().nth(1).unwrap().to_lowercase();

    // Check if task id is valid
    if !check_if_task_id_is_valid(task_id) {
        println!("Invalid task id! Please input the id of the task to run. Ex: 1a, 3B, 6A, 22b");
        return;
    }

    // Select task
    match task_id.as_str() {
        "1a" => task_1a(),
        "1b" => task_1b(),
        "2a" => task_2a(),
        "2b" => task_2b(),
        "3a" => task_3a(),
        "3b" => task_3b(),
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

    let final_position = follow_submarine_move_orders(orders);
    println!("Final position: {}, {}", final_position.0, final_position.1);
    println!("Multiplied: {}", final_position.0 * final_position.1);
}

fn task_2b() {
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

    let final_position = follow_submarine_move_orders_advanced(orders);
    println!("Final position: {}, {}", final_position.0, final_position.1);
    println!("Multiplied: {}", final_position.0 * final_position.1);
}

fn task_3a() {
    let filepath = "data/day_3_input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut binary_numbers = Vec::new();

    // Iterate over each line
    for line in reader.lines() {
        let line = line.unwrap();
        // Parse the line as a slice of the String
        let binary_number = line.parse::<String>().unwrap();
        binary_numbers.push(binary_number);
    }

    let gamma_rate = find_gamma_rate(&binary_numbers, 12);
    let epsilon_rate = find_epsilon_rate(&binary_numbers, 12);

    println!(
        "Gamma rate: {} / {}",
        gamma_rate,
        convert_binary_to_decimal(gamma_rate.as_str())
    );
    println!(
        "Epsilon rate: {} / {}",
        epsilon_rate,
        convert_binary_to_decimal(epsilon_rate.as_str())
    );
    let power_consumption = convert_binary_to_decimal(gamma_rate.as_str())
        * convert_binary_to_decimal(epsilon_rate.as_str());
    println!("Power consumption: {}", power_consumption);
}

fn task_3b() {
    let filepath = "data/day_3_input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut binary_numbers = Vec::new();

    // Iterate over each line
    for line in reader.lines() {
        let line = line.unwrap();
        // Parse the line as a slice of the String
        let binary_number = line.parse::<String>().unwrap();
        binary_numbers.push(binary_number);
    }

    let oxygen_generator_rating = find_oxygen_generator_rating(&binary_numbers);
    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    let oxygen_generator_rating = convert_binary_to_decimal(oxygen_generator_rating.as_str());
    println!("Oxygen generator rating: {}", oxygen_generator_rating);

    let co2_scrubber_rating = find_co2_scrubber_rating(&binary_numbers);
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);
    let co2_scrubber_rating = convert_binary_to_decimal(co2_scrubber_rating.as_str());
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;
    println!("Life support rating: {}", life_support_rating);
}

// Returns true if the task id is valid
fn check_if_task_id_is_valid(task_id: &String) -> bool {
    // This was only done in bulk because GitHub Copilot is crazy convenient
    // And also crazy misleading...had to fix a bug here
    task_id == "1a"
        || task_id == "1b"
        || task_id == "2a"
        || task_id == "2b"
        || task_id == "3a"
        || task_id == "3b"
        || task_id == "4a"
        || task_id == "4b"
        || task_id == "5a"
        || task_id == "5b"
        || task_id == "6a"
        || task_id == "6b"
        || task_id == "7a"
        || task_id == "7b"
        || task_id == "8a"
        || task_id == "8b"
        || task_id == "9a"
        || task_id == "9b"
        || task_id == "10a"
        || task_id == "10b"
        || task_id == "11a"
        || task_id == "11b"
        || task_id == "12a"
        || task_id == "12b"
        || task_id == "13a"
        || task_id == "13b"
        || task_id == "14a"
        || task_id == "14b"
        || task_id == "15a"
        || task_id == "15b"
        || task_id == "16a"
        || task_id == "16b"
        || task_id == "17a"
        || task_id == "17b"
        || task_id == "18a"
        || task_id == "18b"
        || task_id == "19a"
        || task_id == "19b"
        || task_id == "20a"
        || task_id == "20b"
        || task_id == "21a"
        || task_id == "21b"
        || task_id == "22a"
        || task_id == "22b"
        || task_id == "23a"
        || task_id == "23b"
        || task_id == "24a"
        || task_id == "24b"
        || task_id == "25a"
        || task_id == "25b"
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

// Depth, Horizontal Position, Aim Direction
struct SubmarinePositionAdvanced(i32, i32, i32);

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

fn follow_submarine_move_orders(input: Vec<SubmarineMoveOrder>) -> SubmarinePosition {
    let mut position = SubmarinePosition(0, 0);
    for order in input {
        position = move_submarine(position, order);
    }
    position
}

fn follow_submarine_move_orders_advanced(
    input: Vec<SubmarineMoveOrder>,
) -> SubmarinePositionAdvanced {
    let mut position = SubmarinePositionAdvanced(0, 0, 0);
    for order in input {
        position = move_submarine_advanced(position, order);
    }
    position
}

fn move_submarine(
    position: SubmarinePosition,
    move_order: SubmarineMoveOrder,
) -> SubmarinePosition {
    match move_order.0 {
        SubmarineMoveDirection::Up => SubmarinePosition(position.0 - move_order.1, position.1),
        SubmarineMoveDirection::Down => SubmarinePosition(position.0 + move_order.1, position.1),
        SubmarineMoveDirection::Forward => SubmarinePosition(position.0, position.1 + move_order.1),
    }
}

fn move_submarine_advanced(
    position: SubmarinePositionAdvanced,
    move_order: SubmarineMoveOrder,
) -> SubmarinePositionAdvanced {
    match move_order.0 {
        SubmarineMoveDirection::Up => {
            SubmarinePositionAdvanced(position.0, position.1, position.2 - move_order.1)
        }
        SubmarineMoveDirection::Down => {
            SubmarinePositionAdvanced(position.0, position.1, position.2 + move_order.1)
        }
        SubmarineMoveDirection::Forward => SubmarinePositionAdvanced(
            position.0 + move_order.1 * position.2,
            position.1 + move_order.1,
            position.2,
        ),
    }
}

// Find the most common value in a number, saved as string
// Intended for use with a binary number, but should work for any system
fn find_most_common_bit_at_position(input: &Vec<String>, position: usize) -> char {
    let mut counts = Vec::new();
    // let mut counts = HashMap::new();

    for binary_number in input {
        let bit = binary_number.chars().nth(position).unwrap();
        counts.push(bit);
        // let count = counts.entry(bit).or_insert(0);
        // *count += 1; // Dereference count and increment
    }

    let mut bit_options = vec![0, 0];
    for (i, bit) in counts.iter().enumerate() {
        // println!("{}: {}", i, &bit);
        let bit_as_number = bit.to_digit(2).unwrap();
        bit_options[bit_as_number as usize] += 1; // Is this the correct way to convert char to usize?
    }

    let mut most_common_bit = 1;
    for (i, bit) in bit_options.iter().enumerate() {
        let j = i as usize; // Is this necessary?
        if &bit_options[most_common_bit] < &bit_options[j] {
            most_common_bit = i;
        }
    }

    most_common_bit.to_string().as_str().chars().nth(0).unwrap() // Is this the correct way to convert to char?
}

// Find the most least value in a number, saved as string
// Intended for use with a binary number, but should work for any system
fn find_least_common_bit_at_position(input: &Vec<String>, position: usize) -> char {
    let mut counts = Vec::new();
    // let mut counts = HashMap::new();

    for binary_number in input {
        let bit = binary_number.chars().nth(position).unwrap();
        counts.push(bit);
        // let count = counts.entry(bit).or_insert(0);
        // *count += 1; // Dereference count and increment
    }

    let mut bit_options = vec![0, 0];
    for (i, bit) in counts.iter().enumerate() {
        // println!("{}: {}", i, &bit);
        let bit_as_number = bit.to_digit(2).unwrap();
        bit_options[bit_as_number as usize] += 1; // Is this the correct way to convert char to usize?
    }

    let mut least_common_bit = 0;
    for (i, bit) in bit_options.iter().enumerate() {
        let j = i as usize; // Is this necessary?
        if &bit_options[least_common_bit] > &bit_options[j] {
            least_common_bit = i;
        }
    }

    least_common_bit
        .to_string()
        .as_str()
        .chars()
        .nth(0)
        .unwrap() // Is this the correct way to convert to char?
}

fn find_gamma_rate(input: &Vec<String>, length: usize) -> String {
    let mut rate: String = "".to_string();
    for i in 0..length {
        let bit = find_most_common_bit_at_position(&input, i);
        rate.push(bit);
    }
    rate
}

fn find_epsilon_rate(input: &Vec<String>, length: usize) -> String {
    let mut rate: String = "".to_string();
    for i in 0..length {
        let bit = find_least_common_bit_at_position(&input, i);
        rate.push(bit);
    }
    rate
}

fn convert_binary_to_decimal(input: &str) -> i32 {
    // println!("{}", input);
    let mut result = 0;
    for (i, bit) in input.graphemes(true).rev().enumerate() {
        let parsed_bit = bit.parse::<i32>().unwrap();
        result += (parsed_bit) * 2_i32.pow(i as u32);
        // println!("{}", result);
    }
    result
}

fn find_oxygen_generator_rating(input: &Vec<String>) -> String {
    // Start with full list of binary numbers
    // TODO: Is this necessary?
    let mut binary_numbers: Vec<String> = input.iter().map(|line| line.to_string()).collect();
    let mut i = 0;
    while binary_numbers.len() > 1 {
        // println!("binary_numbers: {:?}", binary_numbers);
        // Get most common value at current position
        let most_common_bit = find_most_common_bit_at_position(&binary_numbers, i);
        // print!("{}", most_common_bit);
        // println!("most_common_bit: {}", most_common_bit);
        // println!("i: {}", i);
        // Discard any numbers that don't match
        binary_numbers = binary_numbers
            .iter()
            // This line is fine
            .filter(|&binary_number| &binary_number[i..(i + 1)] == most_common_bit.to_string())
            .map(|binary_number| binary_number.to_string())
            .collect();
        // Repeat with next position
        i += 1;
    }
    println!("");
    binary_numbers[0].to_string()
}

fn find_co2_scrubber_rating(input: &Vec<String>) -> String {
    // Start with full list of binary numbers
    // TODO: Is this necessary?
    let mut binary_numbers: Vec<String> = input.iter().map(|line| line.to_string()).collect();
    let mut i = 0;
    while binary_numbers.len() > 1 {
        // Get least common value at current position
        let least_common_bit = find_least_common_bit_at_position(&binary_numbers, i);
        // Discard any numbers that don't match
        binary_numbers = binary_numbers
            .iter()
            .filter(|&binary_number| &binary_number[i..i + 1] == least_common_bit.to_string())
            .map(|binary_number| binary_number.to_string())
            .collect();
        // Repeat with next position
        i += 1;
    }
    binary_numbers[0].to_string()
}
