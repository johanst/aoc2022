#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

enum NodeType {
    Node(usize),
    Val(u32),
}

struct Node {
    left : NodeType,
    right : NodeType
}

impl Node {
    fn to_string(&self, ns : &Vec<Node>) -> String {
        "".to_string()
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

    let mut data: Vec::<(Value, Value)> = Vec::new();
    for d in v.chunks_exact(3) {
        let mut itr = d.iter();
        let a = serde_json::from_str(itr.next().unwrap()).unwrap();
        let b = serde_json::from_str(itr.next().unwrap()).unwrap();
        data.push((a, b));
    }

    dbg!(data);

    part1();
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string() {
        let data = "[[1],[2,3,4]]";
        let v: Value = serde_json::from_str(data).unwrap();
        let s: String = serde_json::to_string(&v).unwrap();
        assert_eq!(data, s);
    }
}

