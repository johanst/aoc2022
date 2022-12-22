#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;
use std::iter;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Place {
    Outside,
    Path,
    Wall
}

#[derive(Debug)]
enum Move {
    Walk(i32),
    Turn(i32),
}

#[derive(Debug)]
struct Actor {
    xpos : i32,
    ypos : i32,
    xdir : i32,
    ydir : i32,
}

#[derive(Debug)]
struct Config {
    m : Vec<Vec<Place>>,
    myrng : Vec<(usize,usize)>,
    mxrng : Vec<(usize,usize)>,
    xsize : usize,
    ysize : usize,
}

fn draw_map(cfg : &Config, actor : &Actor) {
    for y in 0..cfg.ysize {
        for x in 0..cfg.xsize {
            let c = if (x,y) == (actor.xpos as usize, actor.ypos as usize) {
                match (actor.xdir, actor.ydir) {
                    (1, 0) => '>',
                    (-1, 0) => '<',
                    (0, 1) => 'v',
                    (0, -1) => '^',
                    _ => unreachable!(),
                }
            } else {
                match cfg.m[y][x] {
                    Place::Outside => ' ',
                    Place::Wall => '#',
                    Place::Path => '.',
                }
            };
            print!("{c}");
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
    let ysize = v.len() - 2;

    let xsize = v.iter().take(ysize).map(|r| r.len()).max().unwrap();
    //dbg!(ysize, xsize);

    let mut cfg = Config {
        m : Vec::new(),
        myrng : Vec::new(), // given x, what is ymin,ymax
        mxrng : Vec::new(), // given y, what is xmin,xmax
        xsize,
        ysize,
    };
    for (yidx, row) in v.iter().take(ysize).enumerate() {
        cfg.m.push(Vec::new());
        let mut xmin = usize::MAX;
        let mut xmax = usize::MIN;
        for (xidx, c) in row.chars().enumerate() {
            let place = match c {
                ' ' => Place::Outside,
                '.' => Place::Path,
                '#' => Place::Wall,
                _ => unreachable!(),
            };
            cfg.m[yidx].push(place);
            if place != Place::Outside {
                xmin = cmp::min(xmin, xidx);
                xmax = cmp::max(xmax, xidx);
            }
        }
        let xlen = cfg.m[yidx].len();
        if xlen != xsize {
            cfg.m[yidx].extend(iter::repeat(Place::Outside).take(xsize - xlen));
        }
        cfg.mxrng.push((xmin, xmax));
    }
    // Calc yranges
    for xidx in 0..xsize {
        let mut ymin = usize::MAX;
        let mut ymax = usize::MIN;
        for yidx in 0..ysize {
            if cfg.m[yidx][xidx] != Place::Outside {
                ymin = cmp::min(ymin, yidx);
                ymax = cmp::max(ymax, yidx);
            }
        }
        cfg.myrng.push((ymin, ymax));
    }
    // Get path
    let mut p : Vec<Move> = Vec::new();
    let mut n = 0;
    for c in v.last().unwrap().chars() {
        match c {
            'L' => {
                p.push(Move::Walk(n));
                p.push(Move::Turn(-1));
                n = 0;
            },
            'R' => {
                p.push(Move::Walk(n));
                p.push(Move::Turn(1));
                n = 0;
            },
            c => {
                n *= 10;
                n += c.to_digit(10).unwrap() as i32;
            }
        }
    }
    if n != 0 {
        p.push(Move::Walk(n));
    }

    let actor = Actor {
        xpos : cfg.mxrng[0].0 as i32,
        ypos : 0,
        xdir : 1,
        ydir : 0,
    };
    draw_map(&cfg, &actor);

    //dbg!(&p);

    //dbg!(cfg.m);
    //dbg!(cfg.mxrng);
    //dbg!(cfg.myrng);
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
