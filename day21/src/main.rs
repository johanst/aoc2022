#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug, Clone)]
enum Operation {
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Lit(i64),
}

#[derive(Debug, Clone)]
struct OpWithValue {
    op : Operation,
    val : Option<i64>,
    tainted : bool,
}

struct OpExt {
    op : Operation,
    val : Option<i64>,
}

fn part1(mnam2id : &HashMap<String, usize>, ops : &mut Vec<OpWithValue>) {
    let root_idx = *mnam2id.get("root").unwrap();
    let result = calc(root_idx, ops);
    println!("Monkey result: {result}");
}

fn calc_with_humn(idx : usize, humn_idx : usize, humn_val : i64, ops : &Vec<OpWithValue>) -> i64 {
    let mut ops = ops.clone();
    ops[humn_idx].val = Some(humn_val);
    calc(idx, &mut ops)
 }

fn part2(mnam2id : &HashMap<String, usize>, mut ops : Vec<OpWithValue>) {
    let root_idx = *mnam2id.get("root").unwrap();
    let (lhs_idx, rhs_idx) = match ops[root_idx].op {
        Operation::Add(l, r) => (l, r),
        _ => unreachable!()
    };
    let humn_idx = *mnam2id.get("humn").unwrap();

    let mut humn_high = 4000000000000i64;
    let mut humn_low = 3000000000000i64;
    let rhs_val = calc_with_humn(rhs_idx, humn_idx, 0, &ops);
    dbg!(rhs_val);
    let mut lhs_val = 0;
    while (lhs_val != rhs_val) {
        let humn_middle = (humn_high + humn_low) / 2;
        lhs_val = calc_with_humn(lhs_idx, humn_idx, humn_middle, &ops);
        if lhs_val > rhs_val {
            humn_low = humn_middle;
        } else if lhs_val < rhs_val {
            humn_high = humn_middle;
        } else {
            println!("Say: {humn_middle}");
        }
    }
    dbg!(lhs_val);
    dbg!(rhs_val);

//    let humn_arr : [i64; 2] = [4000000000000, 3000000000000];
//    for humn_val in humn_arr {
//        let mut ops_l = ops.clone();
//        let mut ops_r = ops.clone();
//        ops_l[humn_idx].val = Some(humn_val);
//        ops_r[humn_idx].val = Some(humn_val);
//        let result_l = calc(lhs_idx, &mut ops_l);
//        let result_r = calc(rhs_idx, &mut ops_r);
//        println!("Humn = {humn_val} =>");
//        println!("   Left: {result_l}");
//        println!("  Right: {result_r}");
//    }
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
    let lines = std::fs::read_to_string("input.txt").unwrap();
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
            ops.push(OpWithValue {op: Operation::Lit(val), val: Some(val), tainted: false});
        } else {
            let (aidx, bidx) = (*mnam2id.get(w[1]).unwrap(), *mnam2id.get(w[3]).unwrap());
            let op = match w[2] {
                "+" => Operation::Add(aidx, bidx),
                "-" => Operation::Sub(aidx, bidx),
                "*" => Operation::Mul(aidx, bidx),
                "/" => Operation::Div(aidx, bidx),
                _ => unreachable!(),
            };
            ops.push(OpWithValue {op, val: None, tainted: false});
        }
    }

    //dbg!(&mnams);
    //dbg!(&mnam2id);
    //dbg!(&ops);
    //dbg!(v);

    let ops2 = ops.clone();
    //part1(&mnam2id, &mut ops);
    part2(&mnam2id, ops2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
    }
}

