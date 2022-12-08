#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

#[derive(Debug, Default)]
struct Visibility {
    up : bool,
    left : bool,
    right : bool,
    down : bool
}

fn check_visible(m : &Vec<Vec<u32>>, v : &mut Vec<Vec<bool>>, y : usize, x : usize) -> bool {
    let lym = m.len();
    let lxm = m[0].len();

    if v[y][x] {
        return true;
    }
    if y == 0 || y == lym - 1 || x == 0 || x == lxm - 1 {
        v[y][x] = true;
        return true;
    }
    let mut visible = false;
    if m[y][x] > m[y-1][x] {
        visible |= check_visible(m, v, y - 1, x);
    }
    if m[y][x] > m[y][x - 1] {
        visible |= check_visible(m, v, y, x - 1);
    }
    if m[y][x] > m[y+1][x] {
        visible |= check_visible(m, v, y + 1, x);
    }
    if m[y][x] > m[y][x + 1] {
        visible |= check_visible(m, v, y, x+1);
    }

    v[y][x] = visible;
    visible
}

fn part1() {
    let lines = std::fs::read_to_string("ex.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut m : Vec<Vec<u32>> = Vec::new();
    for s in v.iter() {
        m.push(s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }

    let mut vis : Vec<Vec<bool>> = Vec::new();
    let lym = m.len();
    let lxm = m[0].len();
    for _ in 0..lym {
        vis.push(vec![false; lxm]);
    }

    for y in 0..lym {
        for x in 0..lxm {
            check_visible(&m, &mut vis, y, x);
        }
    }

    let mut count = 0;
    for y in 0..lym {
        for x in 0..lxm {
            if vis[y][x] {
                count += 1;
            }
        }
    }

    println!("{m:?}");
    println!("{vis:?}");
    println!("Number of trees: {count}");
}

fn part2() {
}

fn main() {
    part1();
    //part2();
}
