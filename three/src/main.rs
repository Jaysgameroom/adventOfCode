use std::fs;

fn main() {
    let file_path = "assets/input";

    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");

    let banks: Vec<&str>  = contents.trim()
        .split("\n")
        .collect();

    let sum = banks.iter().fold(0, |x, y|  x + get_max_joltage(y));
    println!("{sum}")
}

fn get_max_joltage(bank: &str) -> u32 {
    println!("{bank}");
    let bank: Vec<u32> = bank.chars()
        .filter_map(|x| x.to_digit(10))
        .collect();

    let first = bank[..bank.len() - 1].iter().max().expect("couldnt find max");

    let pos = bank.iter().position(|x| x == first).expect("couldnt find pos");

    let second = bank[pos + 1..].iter().max().expect("couldnt find max");


    println!("{}{}", first, second);
   
    format!("{}{}", first, second).parse().expect("failed to parse")

}

