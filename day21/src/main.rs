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
enum Operation {
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Lit(i64),
}

#[derive(Debug)]
struct OpWithValue {
    op : Operation,
    val : Option<i64>,
}

fn part1() {
}

fn part2() {
}

fn calc(idx : usize, ops : &mut Vec<OpWithValue>) -> i64 {
    if let Some(val) = ops[idx].val {
        return val;
    } else {
        let (a, b) = match ops[idx].op {
            Operation::Add(a, b) => (a, b),
            Operation::Sub(a, b) => (a, b),
            Operation::Mul(a, b) => (a, b),
            Operation::Div(a, b) => (a, b),
            _ => unreachable!(),
        };
        let aval = calc(a, ops);
        let bval = calc(b, ops);
        let result = match ops[idx].op {
            Operation::Add(a, b) => aval + bval,
            Operation::Sub(a, b) => aval - bval,
            Operation::Mul(a, b) => aval * bval,
            Operation::Div(a, b) => aval / bval,
            _ => unreachable!(),
        };
        ops[idx].val = Some(result);
        result
    }
}

fn main() {
    let lines = std::fs::read_to_string("ex.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut mnams : Vec::<String> = Vec::new();
    let mut mnam2id : HashMap<String, usize> = HashMap::new();
    for (idx, r) in v.iter().enumerate() {
        let mnam = r.split(":").next().unwrap().to_string();
        mnam2id.insert(mnam.clone(), mnams.len());
        mnams.push(mnam);
    }

    let mut ops : Vec<OpWithValue> = Vec::new();
    for r in v.iter() {
        let w = r.split_whitespace().collect::<Vec<&str>>();
        if w.len() < 4 {
            let val = w[1].parse::<i64>().unwrap();
            ops.push(OpWithValue {op: Operation::Lit(val), val: Some(val)});
        } else {
            let (aidx, bidx) = (*mnam2id.get(w[1]).unwrap(), *mnam2id.get(w[3]).unwrap());
            let op = match w[2] {
                "+" => Operation::Add(aidx, bidx),
                "-" => Operation::Sub(aidx, bidx),
                "*" => Operation::Mul(aidx, bidx),
                "/" => Operation::Div(aidx, bidx),
                _ => unreachable!(),
            };
            ops.push(OpWithValue {op, val: None});
        }
    }

    //dbg!(&mnams);
    //dbg!(&mnam2id);
    //dbg!(&ops);
    //dbg!(v);

    let root_idx = *mnam2id.get("root").unwrap();
    let result = calc(root_idx, &mut ops);
    println!("Monkey result: {result}");

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

