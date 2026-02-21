use std::fs;

fn main() {
    let file_path = "assets/input";
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");

    let turns = contents.split("\n");

    let mut dial: i32 = 50;
    let mut count: i32 = 0;

    for turn in turns{
        let direction = &turn[..1];
        let distance: i32 = turn[1..].parse().expect("Failed to parse number");

        if direction == "L"{
            dial = dial - distance;
        } else if direction == "R"{
            dial = dial + distance;
        }
        
        dial = dial % 100;

        if dial == 0 {
            count = count + 1;
        }


        println!("{turn} {dial} {count}")
    }

    println!("{count}")

}








