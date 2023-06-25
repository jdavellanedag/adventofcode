use std::fs::read_to_string;

// Disclaimer, this is a ugly piece of code, I'm trying to learn tmux + vim + rust at the same time
fn main() {
    let data = read_lines("./input.txt");

    let mut sum: u64 = 0;
    let mut total = Vec::new();

    for line in data {
        if !line.trim().is_empty() {
            sum += &line.parse::<u64>().unwrap();
        } else {
            total.push(sum.clone());
            sum = 0;
        }
    }
    total.push(sum.clone()); // This push the last element, could be better :)

    println!("Elfe with more food is: {}", total.iter().max().unwrap());
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
