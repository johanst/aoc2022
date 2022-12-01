use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut max_calories = 0;
    let mut calories = 0;
    let mut calvec : Vec<u32> = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(cal) = line {
                if let Ok(cal) = cal.parse::<u32>() {
                    calories += cal;
                } else {
                    calvec.push(calories);
                    calories = 0;
                }
                max_calories = std::cmp::max(max_calories, calories);
            }
        }
    }

    if calories != 0 {
        calvec.push(calories);
    }

    println!("Max calories: {}", max_calories);

    calvec.sort_by(|a, b| b.cmp(a));
    let c3 : u32 = calvec.iter().take(3).sum();

    println!("Sum of 3 larges: {}", c3);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
