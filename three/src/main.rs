use std::fs;

fn main() {
    // let file_path = "assets/input";
    //
    // let contents = fs::read_to_string(file_path)
    //     .expect("Failed to read file");
    //
    // println!("{contents}")
    //
    get_joltage("122213");
}

fn get_joltage(bank: &str){
    let bank: Vec<(usize, u32)> = bank.char_indices()
        .filter_map(|(x, y)| y.to_digit(10).map(|digit| (x, digit)))
        .collect();

    let first = bank[..bank.len() - 1].iter().max_by_key(|(_x, y)| y).unwrap();

    let second = bank[first.0..bank.len()].iter().max_by_key(|(_x, y)| y).unwrap();

    println!("{}{}", first.1, second.1)



}

