#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug, Clone, PartialEq)]
enum Square {
    Empty,
    Rock,
    Sand
}

impl Square {
    fn to_char(&self) -> char{
        match self {
            Square::Empty => '.',
            Square::Rock => '#',
            Square::Sand => 'o',
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    map: Vec<Vec<Square>>,
}

impl Grid {
    fn draw(&self) {
        for y in 0..self.map.len() {
            for x in self.xmin..=self.xmax {
                print!("{}", self.map[y][x - self.xmin].to_char());
            }
            println!();
        }
    }
}

fn part1(vtmp: &Vec<Vec<(usize, usize)>>, mut g: Grid) {
    for _ in 0..g.ymax+1 {
        g.map.push(vec![Square::Empty; g.xmax - g.xmin + 1]);
    }

    for row in vtmp.iter() {
        for w in row.windows(2) {
            let (xs, ys) = w[0];
            let (xe, ye) = w[1];
            if xs == xe {
                if ys > ye {
                    (ye..=ys).for_each(|y| g.map[y][xs - g.xmin] = Square::Rock);
                } else {
                    (ys..=ye).for_each(|y| g.map[y][xs - g.xmin] = Square::Rock);
                }
            } else if ys == ye {
                if xs > xe {
                    (xe..=xs).for_each(|x| g.map[ys][x - g.xmin] = Square::Rock);
                } else {
                    (xs..=xe).for_each(|x| g.map[ys][x - g.xmin] = Square::Rock);
                }
            } else {
                unreachable!();
            }
            //dbg!(xs, ys, xe, ye);
        }
    }

    let mut n = 0;
    loop {
        let (mut x, mut y) = (500, 0);
        let resting = loop {
            // down
            if y == g.ymax {
                break false;
            }
            if g.map[y + 1][x - g.xmin] == Square::Empty {
                y += 1;
                continue;
            }

            // left
            if x == g.xmin {
                break false;
            }
            if g.map[y + 1][x - 1 - g.xmin] == Square::Empty {
                x -= 1;
                continue;
            }

            // right
            if x == g.xmax {
                break false;
            }
            if g.map[y + 1][x + 1 - g.xmin] == Square::Empty {
                x += 1;
                continue;
            }

            break true;
        };

        if !resting {
            break;
        }

        n += 1;
        g.map[y][x - g.xmin] = Square::Sand;
    };

    println!();
    g.draw();
    println!();

    println!("Units of sand: {n}");
}

fn part2(vtmp: &Vec<Vec<(usize, usize)>>, mut g: Grid) {
    dbg!(g.xmin, g.xmax, g.ymin, g.ymax);
    g.xmin = 500 - (g.ymax + 3);
    g.xmax = 500 + (g.ymax + 3);

    for _ in 0..g.ymax+1 {
        g.map.push(vec![Square::Empty; g.xmax - g.xmin + 1]);
    }

    for row in vtmp.iter() {
        for w in row.windows(2) {
            let (xs, ys) = w[0];
            let (xe, ye) = w[1];
            if xs == xe {
                if ys > ye {
                    (ye..=ys).for_each(|y| g.map[y][xs - g.xmin] = Square::Rock);
                } else {
                    (ys..=ye).for_each(|y| g.map[y][xs - g.xmin] = Square::Rock);
                }
            } else if ys == ye {
                if xs > xe {
                    (xe..=xs).for_each(|x| g.map[ys][x - g.xmin] = Square::Rock);
                } else {
                    (xs..=xe).for_each(|x| g.map[ys][x - g.xmin] = Square::Rock);
                }
            } else {
                unreachable!();
            }
            //dbg!(xs, ys, xe, ye);
        }
    }

    let l = g.map[0].len();
    g.map.push(vec![Square::Empty; l]);
    g.map.push(vec![Square::Rock; l]);

    let mut n = 0;

    loop {
        let (mut x, mut y) = (500, 0);
        let resting = loop {
            // down
            if g.map[y + 1][x - g.xmin] == Square::Empty {
                y += 1;
                continue;
            }

            // left
            if g.map[y + 1][x - 1 - g.xmin] == Square::Empty {
                x -= 1;
                continue;
            }

            // right
            if g.map[y + 1][x + 1 - g.xmin] == Square::Empty {
                x += 1;
                continue;
            }

            break true;
        };

        n += 1;
        g.map[y][x - g.xmin] = Square::Sand;

        if y == 0 && x == 500 {
            break;
        }
    };

    //println!();
    //g.draw();
    //println!();

    println!("Units of sand: {n}");
}

fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
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
        let rocks = row.split_whitespace()
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
        vtmp.push(rocks);
    }


    //dbg!(&g);

    //part1(&vtmp, g.clone());
    part2(&vtmp, g);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
    }
}

