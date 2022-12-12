#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    y: i32,
    x: i32
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(v : &Vec<&str>) {
    let mut m : Vec<Vec<i32>> = Vec::new();
    let mut vis : Vec<Vec<i32>> = Vec::new();
    let (mut sy, mut sx) = (0, 0);
    let (mut ey, mut ex) = (0, 0);
    for (y, r) in v.iter().enumerate() {
        m.push(Vec::new());
        vis.push(vec![i32::MAX; r.len()]);
        for (x, c) in r.chars().enumerate() {
            m[y].push(
                match c {
                    'S' => {
                        (sy, sx) = (y, x);
                        0i32
                    },
                    'E' => {
                        (ey, ex) = (y, x);
                        ('z' as u32 - 'a' as u32) as i32
                    },
                    _ => (c as u32 - 'a' as u32) as i32
                });
        }
    }

    let ylen = m.len() as i32;
    let xlen = m[0].len() as i32;
    let directions : [(i32, i32); 4] = [(-1,0), (0,-1), (0,1), (1,0)];

    let mut heap = BinaryHeap::new();
    vis[sy][sx] = 0;
    heap.push(State{cost:0, y: sy as i32, x: sx as i32});
    while let Some(State {cost, y, x}) = heap.pop() {
        if y as usize == ey && x as usize == ex {
            println!("Reached end, total cost: {cost}");
        }

//        if vis[y as usize][x as usize] <= cost {
//            // Been here before at lower cost
//            continue;
//        }

        for (dy, dx) in directions.iter() {
            let yy = y + dy;
            let xx = x + dx;
            if yy >= 0 && yy < ylen && xx >= 0 && xx < xlen {
                //dbg!(y, x, yy, xx);
                //let dcost = m[y][x] - m[yy][xx]
                let dcost = 1;
                if m[yy as usize][xx as usize] - m[y as usize][x as usize] > 1 {
                    // we cannot move up more than one step
                    continue;
                }

                let state_next = State {cost: cost + dcost, y: yy, x: xx};
                if state_next.cost < vis[yy as usize][xx as usize] {
                    // We have a new shorter path to yy,xx
                    heap.push(state_next);
                    // Record the cost
                    vis[yy as usize][xx as usize] = state_next.cost;
                }
            }
        }
    }

    //dbg!(m);
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

    part1(&v);
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
    }
}

