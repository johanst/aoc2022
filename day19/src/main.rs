#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug, Default)]
struct Config {
    ore_rc_ore : u32,
    clay_rc_ore : u32,
    obs_rc_ore : u32,
    obs_rc_clay : u32,
    geo_rc_ore : u32,
    geo_rc_obs : u32,
}

fn get_numeric(s: &str, idx : usize) -> u32 {
    let mut w = s.split_whitespace();
    if idx == 0 {
        return w.next().unwrap().parse::<u32>().unwrap();
    } else {
        return w.skip(idx).next().unwrap().parse::<u32>().unwrap();
    }
}

fn part1() {
}

fn part2() {
}

fn main() {
    let lines = std::fs::read_to_string("ex.txt").unwrap();
    let v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    let mut cfgs : Vec<Config> = Vec::new();
    for w in v.chunks(6) {
        let mut witr = w.iter();
        witr.next();
        let mut cfg = Config::default();
        cfg.ore_rc_ore = get_numeric(witr.next().unwrap(), 4);
        cfg.clay_rc_ore = get_numeric(witr.next().unwrap(), 4);
        let obsrob = witr.next().unwrap();
        cfg.obs_rc_ore = get_numeric(obsrob, 4);
        cfg.obs_rc_clay = get_numeric(obsrob, 7);
        let georob = witr.next().unwrap();
        cfg.geo_rc_ore = get_numeric(georob, 4);
        cfg.geo_rc_obs = get_numeric(georob, 7);

        cfgs.push(cfg);
    }

    dbg!(cfgs);
    dbg!(v);

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

