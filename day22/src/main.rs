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
    dbg!(ysize, xsize);

    let mut myrng : Vec<(usize,usize)> = Vec::new(); // given x, what is ymin,ymax
    let mut mxrng : Vec<(usize,usize)> = Vec::new(); // given y, what is xmin,xmax
    let mut m : Vec<Vec<Place>> = Vec::new();
    for (yidx, row) in v.iter().take(ysize).enumerate() {
        m.push(Vec::new());
        let mut xmin = usize::MAX;
        let mut xmax = usize::MIN;
        for (xidx, c) in row.chars().enumerate() {
            let place = match c {
                ' ' => Place::Outside,
                '.' => Place::Path,
                '#' => Place::Wall,
                _ => unreachable!(),
            };
            m[yidx].push(place);
            if place != Place::Outside {
                xmin = cmp::min(xmin, xidx);
                xmax = cmp::max(xmax, xidx);
            }
        }
        let xlen = m[yidx].len();
        if xlen != xsize {
            m[yidx].extend(iter::repeat(Place::Outside).take(xsize - xlen));
        }
        mxrng.push((xmin, xmax));
    }
    // Calc yranges
    for xidx in 0..xsize {
        let mut ymin = usize::MAX;
        let mut ymax = usize::MIN;
        for yidx in 0..ysize {
            if m[yidx][xidx] != Place::Outside {
                ymin = cmp::min(ymin, yidx);
                ymax = cmp::max(ymax, yidx);
            }
        }
        myrng.push((ymin, ymax));
    }

    //dbg!(m);
    //dbg!(mxrng);
    //dbg!(myrng);
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

