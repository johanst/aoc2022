#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn calc_prio(item : u32) -> u32 {
    if item < 'a' as u32 {
        item - 'A' as u32 + 27
    } else {
        item - 'a' as u32 + 1
    }
}

fn main() {
    let mut sum_prio = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                assert_eq!(line.len() % 2, 0);
                let m = line.len() / 2;
                let (l1, l2) = line.split_at(m);
                let a : HashSet<u32> = l1.as_bytes().iter().map(|c| *c as u32).collect();
                let b : HashSet<u32> = l2.as_bytes().iter().map(|c| *c as u32).collect();
                let intersection : Vec<u32> = a.intersection(&b).cloned().collect();
                assert_eq!(intersection.len(), 1);

                let prio = calc_prio(intersection.first().cloned().unwrap());
                assert!(prio > 0 && prio <= 52);
                println!("P{} * {:?}/{:?} -> {:?}", prio, a, b, intersection);

                sum_prio += prio;
            }
        }
    }

    println!("Total: {}", sum_prio);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
