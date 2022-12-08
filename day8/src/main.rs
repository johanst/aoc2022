#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashMap;
use std::cmp;

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

#[derive(Debug, Default, Clone)]
struct Largest {
    up : i32,
    left : i32,
    right : i32,
    down : i32
}

fn is_visible(m : &Vec<Vec<i32>>, lg : &Vec<Vec<Largest>>, y : usize, x : usize) -> bool {
    m[y][x] > lg[y][x].up ||
        m[y][x] > lg[y][x].left ||
        m[y][x] > lg[y][x].right ||
        m[y][x] > lg[y][x].down
}

fn set_largest_from_here(m : &Vec<Vec<i32>>,
                         v : &mut Vec<Vec<Largest>>,
                         y : usize,
                         x : usize,
                         d : Direction) -> i32 {
    let lym = m.len();
    let lxm = m[0].len();

    let largest =
        match d {
            Direction::Up => {
                if v[y][x].up == 0 {
                    v[y][x].up = if y == 0 { -1 } else { set_largest_from_here(m, v, y - 1, x, d) };
                }
                cmp::max(m[y][x], v[y][x].up)
            },
            Direction::Left => {
                if v[y][x].left == 0 {
                    v[y][x].left = if x == 0 { -1 } else { set_largest_from_here(m, v, y, x - 1, d) };
                }
                cmp::max(m[y][x], v[y][x].left)
            },
            Direction::Right => {
                if v[y][x].right == 0 {
                    v[y][x].right = if x == lxm - 1 { -1 } else { set_largest_from_here(m, v, y, x + 1, d) };
                }
                cmp::max(m[y][x], v[y][x].right)
            },
            Direction::Down => {
                if v[y][x].down == 0 {
                    v[y][x].down = if y == lym - 1 { -1 } else { set_largest_from_here(m, v, y + 1, x, d) };
                }
                cmp::max(m[y][x], v[y][x].down)
            },
        };

    largest
}

fn part1() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut m : Vec<Vec<i32>> = Vec::new();
    for s in v.iter() {
        m.push(s.chars().map(|c| c.to_digit(10)
                             .map(|u| i32::try_from(u).unwrap()).unwrap())
               .collect::<Vec<i32>>());
    }

    let mut lg : Vec<Vec<Largest>> = Vec::new();
    let lym = m.len();
    let lxm = m[0].len();
    for _ in 0..lym {
        lg.push(vec![Largest::default(); lxm]);
    }

    for y in 0..lym {
        for x in 0..lxm {
            set_largest_from_here(&m, &mut lg, y, x, Direction::Up);
            set_largest_from_here(&m, &mut lg, y, x, Direction::Left);
            set_largest_from_here(&m, &mut lg, y, x, Direction::Right);
            set_largest_from_here(&m, &mut lg, y, x, Direction::Down);
        }
    }

    let mut count = 0;
    for y in 0..lym {
        for x in 0..lxm {
            if is_visible(&m, &lg, y, x) {
                println!("{y}:{x}");
                count += 1;
            }
        }
    }

    //println!("{m:?}");
    //println!("{lg:?}");
    println!("Number of trees: {count}");
}

fn part2() {
}

fn main() {
    part1();
    //part2();
}
