#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug, Clone, Copy)]
struct Sensor {
    sx : i32,
    sy : i32,
    by : i32,
    bx : i32,
}

fn hamming_dist(a : (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn to_i32(s : &str) -> i32 {
    let sn = s.chars()
        .filter(|c| *c == '-' || c.is_numeric())
        .collect::<String>();
    sn.parse::<i32>().unwrap()
}

fn part1(m: &Vec<Sensor>, ypos: i32) {
    // xr = (xmin, xmax)
    let mut xr: Vec<(i32, i32)> = Vec::new();
    for sd in m.iter() {
        let hd_edge = hamming_dist((sd.sx, sd.sy), (sd.bx, sd.by));
        let hd_ypos = hamming_dist((sd.sx, sd.sy), (sd.sx, ypos));
        let dist = hd_edge - hd_ypos;
        if dist >= 0 {
            xr.push((sd.sx - dist, sd.sx + dist));
        }
    }

    let mut xr_mrg: Vec<(i32, i32)> = Vec::new();
    //dbg!(&xr);
    //dbg!(&xr_mrg);
    while !xr.is_empty() {
        let (mut xmin, mut xmax) = xr.pop().unwrap();
        while let Some(idx) = xr.iter().position(
            |(xxmin, xxmax)| {
                if (xmin < *xxmin && xmax < *xxmin) || (*xxmin < xmin && *xxmax < xmin) {
                    return false;
                }
                xmin = cmp::min(xmin, *xxmin);
                xmax = cmp::max(xmax, *xxmax);
                true
            }) {
            xr.swap_remove(idx);
        }
        xr_mrg.push((xmin ,xmax));
        //dbg!(&xr);
        //dbg!(&xr_mrg);
    }

    dbg!(&xr_mrg);

    let beacons = m.iter()
        .filter_map(|sd| if sd.by == ypos { Some(sd.bx) } else { None })
        .collect::<HashSet<i32>>();
    let no_beacons = xr_mrg.iter()
        .fold(0,
              |acc, (xmin, xmax)| {
                  let beacons_in_range : i32 = beacons.iter().filter(|bx| *bx >= xmin && *bx <= xmax).count().try_into().unwrap();
                  acc + xmax - xmin + 1 - beacons_in_range
              });

    dbg!(beacons);
    dbg!(no_beacons);
}

fn part2() {
}

fn read_input(f : &str) -> Vec<Sensor> {
    let lines = std::fs::read_to_string(f).unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut m : Vec<Sensor> = Vec::new();
    for row in v.iter() {
        let w = row.split_whitespace().collect::<Vec<&str>>();
        m.push(Sensor {
            sx : to_i32(w[2]),
            sy : to_i32(w[3]),
            bx : to_i32(w[8]),
            by : to_i32(w[9]),
        });
    }

    m
}

fn main() {
    //let m : Vec<Sensor> = read_input("ex.txt");
    //part1(&m, 10);

    let m : Vec<Sensor> = read_input("input.txt");
    part1(&m, 2000000);

    //dbg!(m);


    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
    }
}

