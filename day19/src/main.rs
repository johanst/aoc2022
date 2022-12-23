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
    ore_rc_ore : u16,
    clay_rc_ore : u16,
    obs_rc_ore : u16,
    obs_rc_clay : u16,
    geo_rc_ore : u16,
    geo_rc_obs : u16,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Default, Clone)]
struct State {
    step : u16,
    ore : u16,
    clay : u16,
    obs : u16,
    geo : u16,
    ore_robots : u16,
    clay_robots : u16,
    obs_robots : u16,
    geo_robots : u16,
}

fn collect(st : &State, steps : u16) -> State {
    let mut st_next = st.clone();
    st_next.step += steps;
    st_next.ore += st_next.ore_robots * steps;
    st_next.clay += st_next.clay_robots * steps;
    st_next.obs += st_next.obs_robots * steps;
    st_next.geo += st_next.geo_robots * steps;

    st_next
}

fn get_max_geodes(cfg : &Config, st : State) -> u16 {
    let mut max_geo = st.geo + st.geo_robots * (24 - st.step);
    //dbg!(&st, max_geo);
    //let mut dummy : String = "".to_string();
    //stdin().read_line(&mut dummy);

    if st.obs_robots > 0 {
        // I can build more geo_robots if have the stuff and I have the
        // robot before step 24
        let obs_needed = if cfg.geo_rc_obs > st.obs {
            cfg.geo_rc_obs - st.obs } else { 0 };
        let ore_needed = if cfg.geo_rc_ore > st.ore {
            cfg.geo_rc_ore - st.ore } else { 0 };
        let steps_needed = cmp::max(
            ( obs_needed + st.obs_robots - 1 ) / st.obs_robots,
            ( ore_needed + st.ore_robots - 1 ) / st.ore_robots);
        if steps_needed + st.step + 1 < 24 {
            // steps_needed to collect + 1 for creating the robot
            let mut st_next = collect(&st, steps_needed + 1);
            st_next.ore -= cfg.geo_rc_ore;
            st_next.obs -= cfg.geo_rc_obs;
            st_next.geo_robots += 1;
            max_geo = cmp::max(max_geo, get_max_geodes(cfg, st_next));
        }
    }

    if st.clay_robots > 0 {
        // build an obsidian robot
        let clay_needed = if cfg.obs_rc_clay > st.clay {
            cfg.obs_rc_clay - st.clay } else { 0 };
        let ore_needed = if cfg.obs_rc_ore > st.ore {
            cfg.obs_rc_ore - st.ore } else { 0 };
        let steps_needed = cmp::max(
            ( ore_needed + st.ore_robots - 1 ) / st.ore_robots,
            ( clay_needed + st.clay_robots - 1 ) / st.clay_robots);
        if steps_needed + st.step + 1 < 24 {
            let mut st_next = collect(&st, steps_needed + 1);
            st_next.ore -= cfg.obs_rc_ore;
            st_next.clay -= cfg.obs_rc_clay;
            st_next.obs_robots += 1;
            max_geo = cmp::max(max_geo, get_max_geodes(cfg, st_next));
        }
    }

    if st.clay_robots <= 8 {
        // never build more than 8 clay robots....
        let ore_needed = if cfg.clay_rc_ore > st.ore {
            cfg.clay_rc_ore - st.ore } else { 0 };
        let steps_needed = ( ore_needed + st.ore_robots - 1 ) / st.ore_robots;
        if steps_needed + st.step + 1 < 24 {
            let mut st_next = collect(&st, steps_needed + 1);
            st_next.ore -= cfg.clay_rc_ore;
            st_next.clay_robots += 1;
            max_geo = cmp::max(max_geo, get_max_geodes(cfg, st_next));
        }
    }

    if st.ore_robots <= 8 {
        // never build more than 8 ore robots....
        let ore_needed = if cfg.ore_rc_ore > st.ore {
            cfg.ore_rc_ore - st.ore } else { 0 };
        let steps_needed = ( ore_needed + st.ore_robots - 1 ) / st.ore_robots;
        if steps_needed + st.step + 1 < 24 {
            let mut st_next = collect(&st, steps_needed + 1);
            st_next.ore -= cfg.ore_rc_ore;
            st_next.ore_robots += 1;
            max_geo = cmp::max(max_geo, get_max_geodes(cfg, st_next));
        }
    }

    max_geo
}

fn get_numeric(s: &str, idx : usize) -> u16 {
    let mut w = s.split_whitespace();
    if idx == 0 {
        return w.next().unwrap().parse::<u16>().unwrap();
    } else {
        return w.skip(idx).next().unwrap().parse::<u16>().unwrap();
    }
}

fn part1(cfgs : &Vec<Config>) {
    let mut quality_tot : u32 = 0;
    for (id, cfg) in cfgs.iter().enumerate() {
        let initial_state = State {
            ore_robots : 1,
            ..Default::default()
        };
        let max_geo : u32 = get_max_geodes(cfg, initial_state) as u32;
        println!("ID {}: Max geodes = {max_geo}", id + 1);
        quality_tot += (id + 1) as u32 * max_geo;
    }
    println!("Total quality: {quality_tot}");
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

    dbg!(&cfgs);
    dbg!(v);

    part1(&cfgs);
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
    }
}

