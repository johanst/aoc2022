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
struct Range {
    xmin : i32,
    xmax : i32,
    ymin : i32,
    ymax : i32,
}

impl Default for Range {
    fn default() -> Self {
        Range {
            xmin : i32::MAX,
            xmax : i32::MIN,
            ymin : i32::MAX,
            ymax : i32::MIN,
        }
    }
}

fn get_range(elves : &HashSet<(i32, i32)>) -> Range {
    let mut rng = Range::default();
    for elf in elves.iter() {
        rng.xmin = cmp::min(rng.xmin, elf.0);
        rng.xmax = cmp::max(rng.xmax, elf.0);
        rng.ymin = cmp::min(rng.ymin, elf.1);
        rng.ymax = cmp::max(rng.ymax, elf.1);
    }
    rng
}

fn draw_elves(elves : &HashSet<(i32, i32)>) {
    let rng = get_range(elves);
    for y in rng.ymin..=rng.ymax {
        for x in rng.xmin..=rng.xmax {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
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

    let mut elves : HashSet<(i32, i32)> = HashSet::new();
    for (y, r) in v.iter().enumerate() {
        for (x, c) in r.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    draw_elves(&elves);

    //dbg!(v);

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

