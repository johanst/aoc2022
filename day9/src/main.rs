#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;

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
    knots : Vec<(i32, i32)>,
    xmin : i32,
    xmax : i32,
    ymin : i32,
    ymax : i32,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hm : HashMap<(i32, i32), char> = HashMap::new();
        hm.insert((0, 0), 's');
        for (n, yx) in self.knots.iter().enumerate().rev() {
            let c = if n == 0 { 'H' } else { char::from_digit(n as u32, 10).unwrap() };
            hm.insert(*yx, c);
        }
        for y in self.ymin..=self.ymax {
            for x in self.xmin..=self.xmax {
                let c : char = hm.get(&(y, x)).cloned().unwrap_or_else(|| '.');
                print!("{}", c);
            }
            println!();
        }

        Ok(())
    }
}

fn walk(moves : &Vec<Move>, nf : usize) -> Map {
    let mut m = Map::default();
    m.visited.insert((0, 0));
    m.knots = vec![(0, 0); nf + 1];

    for mov in moves.iter() {
        let path = match mov.direction {
            Direction::Up => (m.knots[0].0 - mov.steps..m.knots[0].0).rev()
                .map(|item| (item, m.knots[0].1))
                .collect::<Vec<(i32, i32)>>(),
            Direction::Right => (m.knots[0].1 + 1..m.knots[0].1 + 1 + mov.steps)
                .map(|item| (m.knots[0].0, item))
                .collect::<Vec<(i32, i32)>>(),
            Direction::Left => (m.knots[0].1 - mov.steps..m.knots[0].1).rev()
                .map(|item| (m.knots[0].0, item))
                .collect::<Vec<(i32, i32)>>(),
            Direction::Down => (m.knots[0].0 + 1..m.knots[0].0 + 1 + mov.steps)
                .map(|item| (item, m.knots[0].1))
                .collect::<Vec<(i32, i32)>>(),
        };
        println!("{mov:?}");

        let (cury, curx) = m.knots[0];
        for pos in path.iter().map(|dpos| (cury + dpos.1, curx + dpos.0)) {
            let (mut hprevy, mut hprevx) = pos;
            m.knots[0] = 
            for i in 0..nf {
//                let (hprevy, hprevx) = (m.knots[i].0 + dpos.0, m.knots[i].1 +);
//                m.knots[i] = leadpos;
//                if (m.knots[i].0 - m.knots[i+1].0).abs() > 1 ||
//                    (m.knots[i].1 - m.knots[i+1].1).abs() > 1 {
//                    m.knots[i+1] = (hprevy, hprevx);
//                    if i == nf - 1 {
//                        m.visited.insert(m.knots[i+1]);
//                    }
//                }
//                leadpos = m.knots[i+1];
//
//            }
//            m.knots[0] = *pos;
            println!("   {pos:?}");
        };
        println!("{m}");

        m.xmin = i32::min(m.xmin, m.knots[0].1);
        m.xmax = i32::max(m.xmax, m.knots[0].1);
        m.ymin = i32::min(m.ymin, m.knots[0].0);
        m.ymin = i32::max(m.ymin, m.knots[0].0);
    }

    m
}

fn part1(moves : &Vec<Move>) {
    let map = walk(&moves, 1);

    println!("{moves:?}");

    println!("{map:?}");
    println!("{}", map.visited.len());
}


fn part2(moves : &Vec<Move>) {
    let map = walk(&moves, 9);

    println!("{moves:?}");

    println!("{map:?}");
    println!("{}", map.visited.len());
}

fn main() {
    let lines = std::fs::read_to_string("ex.txt").unwrap();
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

    //part1(&moves);
    part2(&moves);
}
