#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug, Default, Clone)]
struct Node {
    rate : u32,
    paths : Vec<usize>
}

#[derive(Debug)]
struct State {
    vid2idx : HashMap<String, usize>,
    idx2vid : Vec<String>,
    map : Vec<Node>,
}

impl State {
    fn new(v : &Vec<&str>) -> Self {
        let mut st = State {
            vid2idx : HashMap::new(),
            idx2vid : Vec::new(),
            map : Vec::new(),
        };
        for (n, row) in v.iter().enumerate() {
            let vid = row.split_whitespace().skip(1).next().unwrap();
            st.vid2idx.insert(vid.to_string(), n);
            st.idx2vid.push(vid.to_string());
        }

        st.map = vec![Node::default(); st.idx2vid.len()];

        for (n, row) in v.iter().enumerate() {
            let paths = row.split_whitespace()
                .skip(9)
                .map(|s| st.vid2idx.get(&s[0..2]).unwrap()).cloned()
                .collect::<Vec<usize>>();
            let rate = row.split_whitespace()
                .skip(4).next().unwrap()
                .split("=")
                .skip(1).next().unwrap()
                .split(";").take(1).next().unwrap()
                .parse::<u32>().unwrap();
            st.map[n] = Node { rate, paths };
        }

        st
    }

    fn vid2idx(&self, s : &str) -> usize {
        self.vid2idx.get(s).unwrap().clone()
    }

    fn idx2vid(&self, idx : usize) -> String {
        self.idx2vid[idx].clone()
    }
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

    let st = State::new(&v);

    dbg!(st);

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

