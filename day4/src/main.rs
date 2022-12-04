#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() {
    let mut sum = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let mut v : Vec<Vec<u32>> = line.split(",").map(
                    |s| s.split("-").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>())
                    .collect();
                if v[0][0] > v[1][0] {
                    v.swap(0, 1);
                }
                // 1 contains b
                let f = v[0][1] >= v[1][1] || (v[0][0] == v[1][0] && v[0][1] <= v[1][1]);
                println!("{:?} - {}", v, f);
                if f {
                    sum += 1;
                }
            }
        }
    }

    println!("Sum: {}", sum);
}

fn part2() {
    let mut sum = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let mut v : Vec<Vec<u32>> = line.split(",").map(
                    |s| s.split("-").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>())
                    .collect();
                if v[0][0] > v[1][0] {
                    v.swap(0, 1);
                }
                // 1 contains b
                let f = v[0][1] >= v[1][1] || (v[0][0] == v[1][0] && v[0][1] <= v[1][1]);
                println!("{:?} - {}", v, f);
                if f {
                    sum += 1;
                }
            }
        }
    }

    println!("Sum: {}", sum);
}

fn main() {
    part1();
    part2();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
