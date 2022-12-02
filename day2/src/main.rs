use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(round : &Vec<(u32,u32)>) {
    let mut score = 0;
    for (opp, me) in round.iter() {
        match ((me + 3)- *opp) % 3 {
            0 => score += 3, // draw
            1 => score += 6, // win
            _ => (),
        }
        // my choice
        score += 1 + me;
    }

    println!("Score: {}", score);
}

fn part2(round : &Vec<(u32,u32)>) {
    let mut score = 0;
    for (opp, me) in round.iter().map(
        |(opp, win)| {
            match win {
                0 => (*opp, (*opp + 2) % 3), // lose
                1 => (*opp, *opp), // draw
                2 => (*opp, (*opp + 1) % 3), // win
                _ => unreachable!(),
            }
        }) {
        match ((me + 3) - opp) % 3 {
            0 => score += 3, // draw
            1 => score += 6, // win
            _ => (),
        }
        // my choice
        score += 1 + me;
    }

    println!("Score: {}", score);
}

fn main() {
    let mut round : Vec<(u32,u32)> = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(row) = line {
                let b = row.as_bytes();
                assert_eq!(row.len(), 3);
                let (opp, me) : (u32, u32) = ((b[0usize] -'A' as u8).into(), (b[2usize] - 'X' as u8).into());
                println!("{} - {}", opp, me);

                round.push((opp, me));
            }
        }
    }

    part1(&round);
    part2(&round);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
