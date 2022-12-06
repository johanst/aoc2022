#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;

fn get_sop(s : &str) -> u32 {
    let mut vd : VecDeque<char> = VecDeque::new();
    let mut count = 0;
    for c in s.chars() {
        count += 1;
        vd.push_back(c);
        if vd.len() == 5 {
            vd.pop_front();
            let vs : HashSet<char> = vd.iter().copied().collect::<HashSet<char>>();
            if vs.len() == 4 {
                // println!("{s} {count} {:?} {:?}", vs, vd);
                return count;
            }
        }
    }

    0
}

fn get_sop_and_som(s : &str) -> u32 {
    let mut vd : VecDeque<char> = VecDeque::new();
    let mut count = 0;
    let mut seq_len = 4usize;
    for c in s.chars() {
        count += 1;
        vd.push_back(c);
        if vd.len() == seq_len + 1 {
            vd.pop_front();
            let vs : HashSet<char> = vd.iter().copied().collect::<HashSet<char>>();
            if vs.len() == seq_len {
                // println!("{s} {count} {:?} {:?}", vs, vd);
                if seq_len == 14 {
                    return count;
                } else {
                    //vd.clear();
                    // WEIRD! seems start of packet can be part of start of message
                    seq_len = 14;
                }
            }
        }
    }

    count
}


fn part1() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let v = lines.split("\n").collect::<Vec<&str>>();

    for s in v.iter() {
        let sop = get_sop(s);
        println!("{sop}");
    }
}

fn part2() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let v = lines.split("\n").collect::<Vec<&str>>();

    for s in v.iter() {
        let sop = get_sop_and_som(s);
        println!("{sop}");
    }
}

fn main() {
    //part1();
    part2();
}
