#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use std::collections::VecDeque;

fn print_top(stack : &Vec<VecDeque<char>>) {
    let s : String = stack.iter().map(|vd| vd.front().unwrap()).collect();
    println!("Top stack: {s}");
}

fn part1() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let v = lines.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(v.len(), 2);
    let mut crates = v[0].split("\n").collect::<Vec<&str>>();
    let mut moves = v[1].split("\n").collect::<Vec<&str>>();
    let width = crates[crates.len() - 1].split_whitespace().count();

    crates.pop();
    moves.pop();
    let mut stack : Vec<VecDeque<char>> = Vec::new();
    for i in 0..width {
        stack.push(VecDeque::new());
    }
    for r in crates.iter() {
        for i in 0..width {
            let c = r.chars().nth(i*4+1).unwrap();
            if c != ' ' {
                stack[i].push_back(c);
            }
        }
    }

    for m in moves.iter() {
        let ms = m.split_whitespace().collect::<Vec<&str>>();
        let n = ms[1].parse::<u32>().unwrap();
        let from = ms[3].parse::<u32>().unwrap();
        let to = ms[5].parse::<u32>().unwrap();
        //println!("{} {} {}", n, from, to);
        for i in 0..n {
            let a = stack[from as usize - 1].pop_front().unwrap();
            stack[to as usize - 1].push_front(a);
        }
        //println!("{stack:?}");
    }

    print_top(&stack);
    //println!("{width:?}");
    //println!("{crates:?}");
    //println!("{moves:?}");
}

fn part2() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let v = lines.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(v.len(), 2);
    let mut crates = v[0].split("\n").collect::<Vec<&str>>();
    let mut moves = v[1].split("\n").collect::<Vec<&str>>();
    let width = crates[crates.len() - 1].split_whitespace().count();

    crates.pop();
    moves.pop();
    let mut stack : Vec<VecDeque<char>> = Vec::new();
    for i in 0..width {
        stack.push(VecDeque::new());
    }
    for r in crates.iter() {
        for i in 0..width {
            let c = r.chars().nth(i*4+1).unwrap();
            if c != ' ' {
                stack[i].push_back(c);
            }
        }
    }

    for m in moves.iter() {
        let ms = m.split_whitespace().collect::<Vec<&str>>();
        let n = ms[1].parse::<u32>().unwrap();
        let from = ms[3].parse::<u32>().unwrap();
        let to = ms[5].parse::<u32>().unwrap();
        //println!("{} {} {}", n, from, to);
        let mut tmp : Vec<char> = Vec::new();
        for i in 0..n {
            let a = stack[from as usize - 1].pop_front().unwrap();
            tmp.push(a);
        }
        for a in tmp.iter().rev() {
            stack[to as usize - 1].push_front(*a);
        }
        //println!("{stack:?}");
    }

    print_top(&stack);
    //println!("{width:?}");
    //println!("{crates:?}");
    //println!("{moves:?}");
}

fn main() {
    //part1();
    part2();
}
