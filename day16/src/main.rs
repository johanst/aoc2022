#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

fn part1() {
}

fn part2() {
}

fn main() {
    let lines = std::fs::read_to_string("ex.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut vid2idx : HashMap<&str, usize> = HashMap::new();
    let mut idx2vid : Vec<&str> = Vec::new();
    for (n, row) in v.iter().enumerate() {
        let vid = row.split_whitespace().skip(1).next().unwrap();
        vid2idx.insert(vid, n);
        idx2vid.push(vid);
    }

    dbg!(vid2idx);
    dbg!(idx2vid);

    part1();
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
    }
}

