use std::fs;
use std::env;

fn main() {
    let file_path = env::args().nth(1).expect("First argument Is given");
    let contents = fs::read_to_string(file_path).expect("File exists and is readable");

    let length = contents.split("\n").nth(0).unwrap().split_whitespace().count();
    let contents: Vec<_> = contents.split_whitespace().collect();





    let mut sum = 0;
    for i in 0..length {
        let slice: Vec<_> = contents.iter().skip(i).step_by(length).collect();
        sum += compute_slice(slice);
    }

    println!("{sum}")



}

fn compute_slice(slice: Vec<&&str>) -> u64 {
    let operation = slice[slice.len() - 1];

    let answer;
    if *operation == "*" {
        answer = slice[0..slice.len() - 1].iter().fold(1, |acc, x| {acc * x.parse::<u64>().unwrap()});
    } else {
        answer = slice[0..slice.len() - 1].iter().fold(0, |acc, x| {acc + x.parse::<u64>().unwrap()});
    }
    answer
}

