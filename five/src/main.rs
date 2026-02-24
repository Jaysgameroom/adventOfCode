use std::fs;
use std::env;

fn main() {
    let file_path = env::args().nth(1).expect("no argument");

    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let (ranges, ingredients) = contents.split_once("\n\n").expect("File not in two segments");

    println!("{ranges}");

    let ranges: Vec<(u64, u64)> = ranges
        .trim()
        .split_whitespace()
        .filter_map(|x| x.split_once("-"))
        .filter_map(|(a, b)| Some((a.parse::<u64>().ok()?, b.parse::<u64>().ok()?)))
        .collect();

    let ingredients = ingredients.
        split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok());


    let sum = ingredients.fold(0, |acc, ing| {
        acc + ranges.iter().any(|&(lower, upper)| {
            ing > lower && ing < upper
        }) as u64
    });

    println!("{sum}");





}


