#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug, Clone, Copy)]
struct Sensor {
    sx : i32,
    sy : i32,
    by : i32,
    bx : i32,
}

fn to_i32(s : &str) -> i32 {
    let sn = s.chars()
        .filter(|c| *c == '-' || c.is_numeric())
        .collect::<String>();
    sn.parse::<i32>().unwrap()
}

fn part1() {
}

fn part2() {
}

fn read_input(f : &str) -> Vec<Sensor> {
    let lines = std::fs::read_to_string(f).unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut m : Vec<Sensor> = Vec::new();
    for row in v.iter() {
        let w = row.split_whitespace().collect::<Vec<&str>>();
        m.push(Sensor {
            sx : to_i32(w[2]),
            sy : to_i32(w[3]),
            bx : to_i32(w[8]),
            by : to_i32(w[9]),
        });
    }

    m
}

fn main() {
    let m : Vec<Sensor> = read_input("ex.txt");
    //dbg!(m);

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

