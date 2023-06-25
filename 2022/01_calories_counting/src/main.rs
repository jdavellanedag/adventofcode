use std::fs::read_to_string;
use itertools::Itertools;

// Disclaimer, this is a ugly piece of code, I'm trying to learn tmux + vim + rust at the same time
fn main() {
   
    let data = read_lines("./input.txt").unwrap_or_else(|err| {
        panic!("Failed to read input file: {}", err);
    });
    
    let total = data
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

    if let Some(max_value) = total.iter().max() {
        println!("Elfe with more food has {} total calories", max_value);
    } else {
        println!("No data found");
    }

}

fn read_lines(filename: &str) -> Result<Vec<String>, std::io::Error> {
    read_to_string(filename)?
        .lines()
        .map(|line| Ok(line.to_string()))
        .collect()
}
