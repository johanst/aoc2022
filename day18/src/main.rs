#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug, Default)]
struct Cube {
    pos : (i32, i32, i32),
    adj : Vec<usize>,
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

    let mut mcubes : HashMap<(i32,i32,i32), usize> = HashMap::new();
    let mut cubes : Vec<Cube> = Vec::new();
    for sc in v.iter() {
        let coords = sc.split(",")
            .map(|w| w.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let pos = (coords[0], coords[1], coords[2]);
        cubes.push(Cube {pos, adj: Vec::new()});
        mcubes.insert(pos, cubes.len() - 1);
    }
    for idx_from in 0..cubes.len() {
        let offset = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
        for (dx, dy, dz) in offset {
            let from_pos = cubes[idx_from].pos;
            let adj_pos = (from_pos.0 + dx, from_pos.1 + dy, from_pos.2 + dz);
            if let Some(idx_to) = mcubes.get(&adj_pos) {
                cubes[idx_from].adj.push(*idx_to);
            }
        }
    }

    let num_free = cubes.iter()
        .fold(0, |acc, cube| acc + (6 - cube.adj.len()));

    dbg!(num_free);

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

