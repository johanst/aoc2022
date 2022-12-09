#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::cmp;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone)]
struct Move {
    direction : Direction,
    steps : i32
}

#[derive(Debug, Default)]
struct Map {
    visited : HashSet<(i32, i32)>,
    xmin : i32,
    xmax : i32,
    ymin : i32,
    ymax : i32,
}

fn part1() {
}


fn part2() {
}

fn walk(moves : &Vec<Move>) -> Map {
    let mut m = Map::default();
    let (mut hposx, mut hposy, mut tposx, mut tposy) = (0, 0, 0, 0);
    m.visited.insert((0, 0));

    for mov in moves.iter() {
        let path = match mov.direction {
            Direction::Up => (hposy - mov.steps..hposy).rev()
                .map(|item| (item, hposx))
                .collect::<Vec<(i32, i32)>>(),
            Direction::Right => (hposx + 1..hposx + 1 + mov.steps)
                .map(|item| (hposy, item))
                .collect::<Vec<(i32, i32)>>(),
            Direction::Left => (hposx - mov.steps..hposx).rev()
                .map(|item| (hposy, item))
                .collect::<Vec<(i32, i32)>>(),
            Direction::Down => (hposy + 1..hposy + 1 + mov.steps)
                .map(|item| (item, hposx))
                .collect::<Vec<(i32, i32)>>(),
        };
        //println!("{mov:?}");

        for pos in path.iter() {
            let (hprevy, hprevx) = (hposy, hposx);
            (hposy, hposx) = *pos;
            if (hposy - tposy).abs() > 1 || (hposx - tposx).abs() > 1 {
                (tposy, tposx) = (hprevy, hprevx);
                m.visited.insert((tposy, tposx));
            }
            //println!("   {pos:?}");
        };

        m.xmin = i32::min(m.xmin, hposx);
        m.xmax = i32::max(m.xmax, hposx);
        m.ymin = i32::min(m.ymin, hposy);
        m.ymin = i32::max(m.ymin, hposy);
    }

    m
}

fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;
    let moves = v.iter()
        .map(|s| {
            let mut tk = s.split_whitespace();
            //assert_eq!(tk.len(), 2);
            let direction = match tk.next().unwrap() {
                "U" => Direction::Up,
                "L" => Direction::Left,
                "R" => Direction::Right,
                "D" => Direction::Down,
                _ => unreachable!()
            };
            let steps = tk.next().unwrap().parse::<i32>().unwrap();
            Move{direction, steps}
        })
        .collect::<Vec<Move>>();

    let map = walk(&moves);

    //println!("{moves:?}");

    //println!("{map:?}");
    println!("{}", map.visited.len());

    part1();
    part2();
}
