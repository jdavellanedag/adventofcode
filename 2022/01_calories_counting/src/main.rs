use std::fs::read_to_string;

// Disclaimer, this is a ugly piece of code, I'm trying to learn tmux + vim + rust at the same time
fn main() {
   
    let data = read_lines("./input.txt").unwrap_or_else(|err| {
        panic!("Failed to read input file: {}", err);
    });
    
    let mut sum: u64 = 0;
    let mut total = Vec::new();

    for line in data {
        if !line.is_empty() {
            match line.parse::<u64>() {
                Ok(value) => sum += value,
                Err(err) => {
                    panic!("FAiled to parse line as u64: {}", err);
                },
            }
        } else {
            total.push(sum);
            sum = 0;
        }
    }
    total.push(sum); // This push the last element, could be better :)
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
