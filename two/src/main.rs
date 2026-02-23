use std::fs;


fn main() {
    let file_path = "assets/input";
    let contents = fs::read_to_string(file_path)
        .expect("failed to read file");

    let pairs: Vec<Vec<&str>> = contents
        .split(",")
        .map(|x| x.split("-")
            .collect())
        .collect();


    let mut sum: i64 = 0;
    for pair in pairs {
        println!("{}, {}", pair[0], pair[1]);
        sum += sum_range(pair[0].trim().parse().expect("not a number"), pair[1].trim().parse().expect("panicking"));
    }

    println!("{sum}");
    
    

}

fn sum_range(start: i64, end: i64) -> i64 {
    
    let mut sum: i64 = 0;

    for i in start..=end {
        if check_number(i){
            sum += i
        }
    }

    sum
}

fn check_number(num: i64) -> bool {
    let rep = num.to_string();

    for i in 1..=rep.len()/2 {
        if &rep[..i] == &rep[i..] {
            return true;
        } }

    false
}
