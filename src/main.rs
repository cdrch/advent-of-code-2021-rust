use std::fs::File;
use std::io::*;

fn main() {
    println!("Hello, world!");

    // Read in the file as array of ints
    let filepath = "data/day_1_input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    // Iterate over each line
    for line in reader.lines() {
        let line = line.unwrap();
        let num = line.parse::<i32>().unwrap();
        println!("{}", num);
    }

    // Track the previous line's value
    // Add to increase or decrease tally
}
