#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug)]
struct Num {
    n : i32,
    next : usize,
    prev : usize,
}

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

    let mut file : Vec<Num> = Vec::new();
    for (i, n) in v.iter().map(|s| s.parse::<i32>().unwrap()).enumerate() {
        let prev = if i == 0 { 0 } else { i - 1 };
        let next = i + 1;
        file.push(Num {n, next, prev});
    }
    file[0].prev = file.len() - 1;

    for idx in 0..file.len() {
        if file[idx].n > 0

    dbg!(file);

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

