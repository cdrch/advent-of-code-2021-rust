use std::fs::File;
use std::io::*;

fn main() {
    println!("Hello, world!");

    // Read in the file as array of ints
    let filepath = "data/day_1_input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    // Iterate over each line
    for line in reader.lines() {
        let line = line.unwrap();
        let num = line.parse::<i32>().unwrap();
        // println!("{}", num);
        numbers.push(num);
    }

    let depth_increases = count_depth_increases(numbers);
    println!("{}", depth_increases);
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
