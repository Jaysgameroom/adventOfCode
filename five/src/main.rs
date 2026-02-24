use std::fs;
use std::env;
use std::ops::RangeInclusive;

fn main() {
    let file_path = env::args().nth(1).expect("no argument");

    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let (ranges, ingredients) = contents.split_once("\n\n").expect("File not in two segments");

    let ranges = ranges.split_whitespace().filter_map(|x| extract_range(x)).flatten();

    for x in ranges {
        println!("{x}")
    }



}

fn extract_range(string: &str) -> Option<RangeInclusive<u64>> {
    let (first, second) = string.split_once("-")?;
    let start: u64 = first.trim().parse().ok()?;
    let end: u64 = second.trim().parse().ok()?;

    Some(start..=end)

}

