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

fn print_prefix(d: u32) {
    for _ in 0..d {
        print!("  ");
    }
    print!("- Compare ");
}

fn print_mixed_type_prefix(d: u32) {
    for _ in 0..d+1 {
        print!("  ");
    }
    print!("- Mixed types; convert ");
}

fn compare(a: &Value, b: &Value, d: u32) -> Ordering {
    match (a, b) {
        (Value::Number(an), Value::Number(bn)) => {
            let (an, bn) = (an.as_u64().unwrap(), bn.as_u64().unwrap());
            print_prefix(d);
            println!("{an} vs {bn}");
            an.cmp(&bn)
        },
        (Value::Number(an), Value::Array(bv)) => {
            print_prefix(d);
            let n = an.as_u64().unwrap();
            let sb = serde_json::to_string(b).unwrap();
            println!("{n} vs {sb}");

            print_mixed_type_prefix(d);
            println!("{n} to [{n}] and retry comparison");

            let av : Value = Value::Array(vec![a.clone(); 1]);
            compare(&av, b, d)
        },
        (Value::Array(av), Value::Number(bn)) => {
            print_prefix(d);
            let n = bn.as_u64().unwrap();
            let sa = serde_json::to_string(a).unwrap();
            println!("{sa} vs {n}");

            print_mixed_type_prefix(d);
            println!("{n} to [{n}] and retry comparison");

            let bv : Value = Value::Array(vec![b.clone(); 1]);
            compare(a, &bv, d)
        },
        (Value::Array(av), Value::Array(bv)) => {
            print_prefix(d);
            let sa = serde_json::to_string(a).unwrap();
            let sb = serde_json::to_string(b).unwrap();
            println!("{sa} vs {sb}");

            let mut avitr = av.iter();
            let mut bvitr = bv.iter();

            let mut anxt = avitr.next();
            let mut bnxt = bvitr.next();
            while anxt.is_some() && bnxt.is_some() {
                let cmp = compare(anxt.unwrap(), bnxt.unwrap(), d + 1);
                if cmp == Ordering::Less {
                    return Ordering::Less;
                } else if cmp == Ordering::Greater {
                    return Ordering::Greater;
                }
                anxt = avitr.next();
                bnxt = bvitr.next();
            }
            if anxt.is_none() {
                if bnxt.is_none() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            } else {
                Ordering::Greater
            }
        },
        _ => Ordering::Equal
    }
}

fn part1() {
}

fn part2() {
}

fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
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

    let mut vcorrect:Vec<usize> = Vec::new();
    for (i, (a, b)) in data.iter().enumerate() {
        println!("== Pair {} ==", i + 1);
        let order = compare(a, b, 0);
        let in_order = order == Ordering::Less || order == Ordering::Equal;
        println!("{}: {in_order}", i + 1);
        if in_order {
            vcorrect.push(i + 1);
        }
    }
    println!("Sum of indices: {}", vcorrect.iter().sum::<usize>());

    // dbg!(data);
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

