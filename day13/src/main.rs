#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::*;
use std::fmt;
use std::io::stdin;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Clone, Debug)]
enum Node {
    Nodes(Vec<usize>),
    Val(u64)
}

fn build_node(v: &serde_json::Value, nodes: &mut Vec<Node>, p: usize) {
    match v {
        Value::Number(n) => {
            let nnode = Node::Val(n.as_u64().unwrap());
            nodes.push(nnode);
            let idx = nodes.len() - 1;
            match (nodes[p]) {
                Node::Nodes(ref mut nref) => nref.push(idx),
                _ => unreachable!(),
            }
        },
        Value::Array(arr) => {
            nodes.push(Node::Nodes(Vec::new()));
            let idx = nodes.len() - 1;
            if p != usize::MAX {
                match (nodes[p]) {
                    Node::Nodes(ref mut nref) => nref.push(idx),
                    _ => unreachable!(),
                }
            }
            for val in arr.iter() {
                build_node(val, nodes, idx);
            }
        },
        _ => unreachable!(),
    };
}



//fn compare(a: &Value, b: &Value) -> Ordering {
//    match a {
//        Value::Number(an) => {
//            let an = an.as_u64().unwrap();
//            match b {
//                Value::Number(bn) => an.cmp(&bn.as_u64().unwrap()),
//                Value::Array(barr) => {
//                    if barr.is_empty()
//                _ => Ordering::Less,
//            }
//        },
//        _ => Ordering::Less,
//    }
//}

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

    // dbg!(data);

    let mut m: Vec<Node> = Vec::new();

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

