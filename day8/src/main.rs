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

fn part1(m : &Vec<Vec<i32>>) {
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

#[derive(Debug, Default, Clone)]
struct TreeCount {
    up : i32,
    left : i32,
    right : i32,
    down : i32
}

impl TreeCount {
    fn scenic_score(&self) -> i32 {
        self.up * self.left * self.right * self.down
    }
}

fn count_trees(m : &Vec<Vec<i32>>, t : &mut Vec<Vec<TreeCount>>, y : usize, x : usize) {
    let lym = m.len();
    let lxm = m[0].len();
    let h_me = m[y][x];

    // up
    let mut d = 0;
    for yy in (0..y).rev() {
        d += 1;
        if m[yy][x] >= h_me {
            break;
        }
    }
    t[y][x].up = d;
    // down
    d = 0;
    for yy in y+1..lym {
        d += 1;
        if m[yy][x] >= h_me {
            break;
        }
    }
    t[y][x].down = d;
    // left
    d = 0;
    for xx in (0..x).rev() {
        d += 1;
        if m[y][xx] >= h_me {
            break;
        }
    }
    t[y][x].left = d;
    // right
    d = 0;
    for xx in x+1..lxm {
        d += 1;
        if m[y][xx] >= h_me {
            break;
        }
    }
    t[y][x].right = d;
}

fn part2(m : &Vec<Vec<i32>>) {
    let lym = m.len();
    let lxm = m[0].len();

    let mut t : Vec<Vec<TreeCount>> = Vec::new();
    for _ in 0..lym {
        t.push(vec![TreeCount::default(); lxm]);
    }

    for y in 0..lym {
        for x in 0..lxm {
            count_trees(m, &mut t, y, x);
        }
    }

    // println!("{t:?}");

    let mut sc : Vec<Vec<i32>> = Vec::new();
    for _ in 0..lym {
        sc.push(vec![0; lxm]);
    }
    for y in 0..lym {
        for x in 0..lxm {
            sc[y][x] = t[y][x].scenic_score();
        }
    }

    // println!("{sc:?}");

    let sum = sc.iter().flatten().max().unwrap();

    println!("Max scenic score: {sum}");
}

fn main() {
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

    //part1(&m);
    part2(&m);
}
