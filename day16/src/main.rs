#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::cmp::Reverse;
use std::fmt;
use std::io::stdin;
use std::iter;

// for finding max pressure
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Step {
    totpot : u32,  // upper limit of possible max released pressure
    current : u32, // actual total released pressure
    step : u32,    // Step nbr, 30=final step
    pos : usize,   // node position (idx in idx2vid and map)
    valves : u64   // valve bitmask (bit0=valve idx0 released, bit1=valve idx1 released, etc...)
}

// for finding shortest path between valve positions
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Pos {
    step : u32,
    pos : usize
}

#[derive(Debug, Default, Clone)]
struct Node {
    rate : u32,
    paths : Vec<usize>
}

#[derive(Debug)]
struct State {
    vid2idx : HashMap<String, usize>,
    idx2vid : Vec<String>,
    map : Vec<Node>,
    rates_total : u32,
    sp : HashMap<(usize,usize), u32>,
    idx_with_valves : Vec<usize>,
}

impl State {
    fn new(v : &Vec<&str>) -> Self {
        let mut st = State {
            vid2idx : HashMap::new(),
            idx2vid : Vec::new(),
            map : Vec::new(),
            rates_total : 0,
            sp : HashMap::new(),
            idx_with_valves : Vec::new(),

        };
        for (n, row) in v.iter().enumerate() {
            let vid = row.split_whitespace().skip(1).next().unwrap();
            st.vid2idx.insert(vid.to_string(), n);
            st.idx2vid.push(vid.to_string());
        }

        st.map = vec![Node::default(); st.idx2vid.len()];

        for (n, row) in v.iter().enumerate() {
            let paths = row.split_whitespace()
                .skip(9)
                .map(|s| st.vid2idx.get(&s[0..2]).unwrap()).cloned()
                .collect::<Vec<usize>>();
            let rate = row.split_whitespace()
                .skip(4).next().unwrap()
                .split("=")
                .skip(1).next().unwrap()
                .split(";").take(1).next().unwrap()
                .parse::<u32>().unwrap();
            st.map[n] = Node { rate, paths };
        }

        st.rates_total = st.map.iter().map(|n| n.rate).sum();

        st.idx_with_valves = st.map.iter().enumerate()
            .filter_map(|(n, nd)| if nd.rate != 0 { Some(n) } else { None })
            .collect::<Vec<usize>>();
        let start_pos = st.vid2idx("AA");
        for a in st.idx_with_valves.iter().cloned().chain(iter::once(start_pos)) {
            for b in st.idx_with_valves.iter().cloned() {
                if a != b {
                    st.sp.insert((a, b), st.shortest_path(a, b));
                }
            }
        }

        st
    }

    fn vid2idx(&self, s : &str) -> usize {
        self.vid2idx.get(s).unwrap().clone()
    }

    fn idx2vid(&self, idx : usize) -> String {
        self.idx2vid[idx].clone()
    }

    fn get_current_released_pressure(&self, mut valves : u64) -> u32 {
        let mut p = 0;
        for node in self.map.iter() {
            if valves & 1 == 1 {
                p += node.rate;
            }
            valves >>= 1;
        }
        p
    }

    fn shortest_path(&self, a : usize, b : usize) -> u32 {
        let mut vis : HashMap<usize, u32> = HashMap::new();
        vis.insert(a, 0);
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(Pos{step:0,pos:a}));
        while let Some(Reverse(pos)) = heap.pop() {
            //dbg!(&pos);
            //let mut dummy : String = "".to_string();
            //stdin().read_line(&mut dummy);

            if pos.pos == b {
                return pos.step;
            }

            for path in self.map[pos.pos].paths.iter() {
                //dbg!(path);
                if pos.step < vis.get(path).cloned().unwrap_or(u32::MAX) {
                    heap.push(Reverse(Pos{step:pos.step+1,pos:*path}));
                    vis.insert(*path, pos.step+1);
                }
            }
        }

        u32::MAX
    }

    fn find_max(&self) {
        let mut heap = BinaryHeap::new();
        heap.push(Step {
            pos: self.vid2idx("AA"),
            ..Default::default()
        });

        //let max_theoretical
        while let Some(step) = heap.pop() {
            //dbg!(&step);
            //let mut dummy : String = "".to_string();
            //stdin().read_line(&mut dummy);

            if step.step == 30 {
                println!("Reached step 30, released pressure: {}", step.current);
                return;
            }

            let mut path_chosen = false;
            for path in self.idx_with_valves.iter().filter(|p| **p != step.pos) {
                if (1 << *path) & step.valves != 0 {
                    // valve already on
                    continue;
                }
                // dbg
                if self.sp.get(&(step.pos, *path)).is_none() {
                    dbg!(&step, *path);
                    dbg!(self.idx2vid(step.pos), self.idx2vid(*path));
                }
                // end dbg
                let sd : u32 = self.sp.get(&(step.pos, *path)).cloned().unwrap();
                if step.step + sd + 1 >= 30 {
                    // too far, won't buy us anything
                    continue;
                }

                path_chosen = true;
                // cost of moving and turning on valve = sd + 1
                let mut step_next = step.clone();
                step_next.pos = *path;
                step_next.step += sd + 1;
                step_next.current += self.get_current_released_pressure(
                    step.valves) * (sd + 1);
                step_next.valves |= 1 << *path;
                step_next.totpot = (30 - step_next.step) * self.rates_total + step_next.current;

                //println!("-----");
                //println!("Step {}: Moving from {} to {} and releasing valve in {} steps while releasing {} pressure",
                //         step.step,
                //         self.idx2vid(step.pos),
                //         self.idx2vid(step_next.pos),
                //         sd + 1,
                //         self.get_current_released_pressure(
                //             step.valves) * (sd + 1)
                //);
                //dbg!(&step);
                //dbg!(&step_next);
                //let mut dummy : String = "".to_string();
                //stdin().read_line(&mut dummy);

                heap.push(step_next);

            }

            // if we couldn't turn on any new valve, let's just stay here
            if !path_chosen {
                let mut step_next = step.clone();
                step_next.step = 30;
                step_next.current += self.get_current_released_pressure(
                    step.valves) * (30 - step.step);
                step_next.totpot = step_next.current;

                heap.push(step_next);
            }
        }
    }
}

fn part1() {
}

fn part2() {
}

fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let st = State::new(&v);
    //dbg!(&st);
    st.find_max();
    //let sp = st.shortest_path(st.vid2idx("AA"),st.vid2idx("EE"));

    //let p = st.get_current_released_pressure(0b11_0000_1100);
    //dbg!(p);

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

