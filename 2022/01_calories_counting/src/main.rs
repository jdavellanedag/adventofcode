use std::fs::read_to_string;
use itertools::Itertools;

// Disclaimer, this is a ugly piece of code, I'm trying to learn tmux + vim + rust at the same time
fn main() {
   
    let data = read_lines("./input.txt").unwrap_or_else(|err| {
        panic!("Failed to read input file: {}", err);
    });
    
    let mut total = data
        .into_iter()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter_map(|(is_value, lines)| {
            if is_value {
                Some(lines.filter_map(|line| line.parse::<u64>().ok()).sum())
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();

    total.sort();
    
    // let n = 3;
    // let total_elves = total.len().saturating_sub(n);
    let highest_calories = total.split_off(total.len() - 3);
    
    println!("Result: {}", highest_calories.iter().sum::<u64>());

}

fn read_lines(filename: &str) -> Result<Vec<String>, std::io::Error> {
    read_to_string(filename)?
        .lines()
        .map(|line| Ok(line.to_string()))
        .collect()
}
