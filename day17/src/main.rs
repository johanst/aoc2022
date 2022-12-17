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

fn print_chamber(cha : &Vec<Vec<u8>>) {
    for row in cha.iter().rev() {
        for c in row.iter() {
            print!("{}", char::from_u32(*c as u32).unwrap());
        }
        println!();
    }
    println!("+-------+");
}

fn add_3_rows_to_chamber(cha : &mut Vec<Vec<u8>>) {
    let vc = "|-------|".as_bytes().to_vec();
    cha.push(vc.clone());
    cha.push(vc.clone());
    cha.push(vc);
}

fn main() {
    let lines = std::fs::read_to_string("shapes.txt").unwrap();
    let v = lines.split("\n\n").collect::<Vec<&str>>();
    let mut shapes : Vec<Vec<&str>> = Vec::new();
    for s in v.iter() {
        let mut shape = s.split("\n").collect::<Vec<&str>>();
        if shape[shape.len() - 1] == "" {
            shape.pop();
        }
        shape.reverse(); // y grows "upwards"
        shapes.push(shape);
    }
    dbg!(&shapes);

    let lines = std::fs::read_to_string("ex.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    assert_eq!(v.len(), 1);
    let v = v;

    let mut chamber : Vec<Vec<u8>> = Vec::new();
    add_3_rows_to_chamber(&mut chamber);
    print_chamber(&chamber);

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

