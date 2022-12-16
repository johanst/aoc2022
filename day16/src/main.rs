#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Step {
    totpot : u32,  // upper limit of possible max released pressure
    current : u32, // actual total released pressure
    step : u32,    // Step nbr, 30=final step
    pos : usize,   // node position (idx in idx2vid and map)
    valves : u64   // valve bitmask (bit0=valve idx0 released, bit1=valve idx1 released, etc...)
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
}

impl State {
    fn new(v : &Vec<&str>) -> Self {
        let mut st = State {
            vid2idx : HashMap::new(),
            idx2vid : Vec::new(),
            map : Vec::new(),
            rates_total : 0,
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

            // turn on valve if valve is not already on
            if self.map[step.pos].rate != 0 && (1 << step.pos) & step.valves != 0 {
                let mut step_next = step.clone();
                step_next.step += 1;
                step_next.current += self.get_current_released_pressure(
                    step.valves);
                step_next.valves |= 1 << step.pos;
                step_next.totpot = (30 - step_next.step) * self.rates_total + step_next.current;
                heap.push(step_next);
            }

            // paths to walk
            for path in self.map[step.pos].paths.iter() {
                let mut step_next = step.clone();
                step_next.step += 1;
                step_next.current += self.get_current_released_pressure(
                    step.valves);
                step_next.pos = *path;
                step_next.totpot = (30 - step_next.step) * self.rates_total + step_next.current;

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
    let lines = std::fs::read_to_string("ex.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let st = State::new(&v);
    st.find_max();

    //dbg!(&st);
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

