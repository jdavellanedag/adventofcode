use std::fs::read_to_string;

fn main() {
    let data = read_lines("./input.txt").unwrap_or_else(|err| {
        panic!("Failed to read input: {}", err);
    });
}

// Inputs values
// Someone's else
// A (Rock)
// B (Paper)
// C (Scissors)
//
// Yours
// X (Rock)     1
// Y (Paper)    2
// Z (Scissors) 3
//
// Loose: 0
// Draw: 3
// Win: 6

fn caluculate_score(another: &str, mine: &str) -> Result<u64, std::io::Error> {
    Ok(1)
}

fn read_lines(filename: &str) -> Result<Vec<String>, std::io::Error> {
    read_to_string(filename)?
        .lines()
        .map(|line| Ok(line.to_string()))
        .collect()
}
