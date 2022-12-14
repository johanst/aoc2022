#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug, Clone)]
enum Square {
    Empty,
    Path,
    Stone
}

impl Square {
    fn to_char(&self) -> char{
        match self {
            Square::Empty => '.',
            Square::Path => '#',
            Square::Stone => 'o',
        }
    }
}

#[derive(Debug)]
struct Grid {
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    map: Vec<Vec<Square>>,
}

impl Grid {
    fn draw(&self) {
        for y in 0..=self.ymax {
            for x in self.xmin..=self.xmax {
                print!("{}", self.map[y][x - self.xmin].to_char());
            }
            println!();
        }
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

    let mut g = Grid {
        xmin: usize::MAX,
        xmax: 0,
        ymin: usize::MAX,
        ymax: 0,
        map: Vec::new()
    };
    let mut vtmp: Vec<Vec<(usize, usize)>> = Vec::new();
    for row in v.iter() {
        let paths = row.split_whitespace()
            .filter(|s| *s != "->")
            .map(|s| {
                let mut sitr = s.split(",");
                let x: usize = sitr.next().unwrap().parse::<usize>().unwrap();
                let y: usize = sitr.next().unwrap().parse::<usize>().unwrap();
                g.xmin = cmp::min(x, g.xmin);
                g.xmax = cmp::max(x, g.xmax);
                g.ymin = cmp::min(y, g.ymin);
                g.ymax = cmp::max(y, g.ymax);
                (x, y)
            })
            .collect::<Vec<(usize, usize)>>();
        vtmp.push(paths);
    }

    for _ in 0..g.ymax+1 {
        g.map.push(vec![Square::Empty; g.xmax - g.xmin + 1]);
    }

    for row in vtmp.iter() {
        for w in row.windows(2) {
            let (xs, ys) = w[0];
            let (xe, ye) = w[1];
            if xs == xe {
                if ys > ye {
                    (ye..=ys).for_each(|y| g.map[y][xs - g.xmin] = Square::Path);
                } else {
                    (ys..=ye).for_each(|y| g.map[y][xs - g.xmin] = Square::Path);
                }
            } else if ys == ye {
                if xs > xe {
                    (xe..=xs).for_each(|x| g.map[ys][x - g.xmin] = Square::Path);
                } else {
                    (xs..=xe).for_each(|x| g.map[ys][x - g.xmin] = Square::Path);
                }
            } else {
                unreachable!();
            }
            //dbg!(xs, ys, xe, ye);
        }
    }

    //dbg!(&g);
    g.draw();

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

