#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::io::stdin;

#[derive(Debug)]
enum Direction {
    U,
    R,
    D,
    L,
}

fn get_input(filename: &str) -> Vec<Vec<Option<Direction>>> {
    let lines = std::fs::read_to_string(filename).unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut m : Vec<Vec<Option<Direction>>> = Vec::new();
    for y in 1..(v.len() - 1) {
        m.push(Vec::new());
        for c in v[y].chars() {
            let spot = match c {
                '#' => continue,
                '^' => Some(Direction::U),
                '>' => Some(Direction::R),
                'v' => Some(Direction::D),
                '<' => Some(Direction::L),
                '.' => None,
                _ => unreachable!(),
            };
            m.last_mut().unwrap().push(spot);
        }
    }

    //dbg!(&v);
    //dbg!(&m);

    m
}

fn get_distance_y(m : &Vec<Vec<Option<Direction>>>,
                  x : i32,
                  yfrom : i32,
                  yto : i32) -> Option<u32> {

    let ylen = m.len() as i32;

    match m[yto as usize][x as usize] {
        Some(Direction::U) => Some(((ylen + yto - yfrom) % ylen) as u32),
        Some(Direction::D) => Some(((ylen + yfrom - yto) % ylen) as u32),
        _ => None,
    }
}

fn get_distance_x(m : &Vec<Vec<Option<Direction>>>,
                  y : i32,
                  xfrom : i32,
                  xto : i32) -> Option<u32> {

    let xlen = m[0].len() as i32;

    match m[y as usize][xto as usize] {
        Some(Direction::L) => Some(((xlen + xto - xfrom) % xlen) as u32),
        Some(Direction::R) => Some(((xlen + xfrom - xto) % xlen) as u32),
        _ => None,
    }
}

// return (distances in x direction, distance in y direction)
fn get_distances(m : &Vec<Vec<Option<Direction>>>,
                 pos : (i32, i32)) -> (Vec<u32>, Vec<u32>) {
    let ylen = m.len() as i32;
    let xlen = m[0].len() as i32;

    let mut ydists : Vec<u32> = Vec::new();
    // Get distances in y direction
    for y in 0..ylen {
        if let Some(dist) = get_distance_y(m, pos.0, pos.1, y) {
            ydists.push(dist);
        }
    }
    let mut xdists : Vec<u32> = Vec::new();
    for x in 0..xlen {
        if let Some(dist) = get_distance_x(m, pos.1, pos.0, x) {
            xdists.push(dist);
        }
    }

    (xdists, ydists)
}

fn dist_to_map(m : &Vec<Vec<Option<Direction>>>,
               dists : &(Vec<u32>, Vec<u32>)) -> (Vec<u8>, Vec<u8>) {
    let ylen = m.len();
    let xlen = m[0].len();

    let mut xmap = vec![0u8; xlen];
    for xdist in dists.0.iter() {
        xmap[*xdist as usize] += 1;
    }

    let mut ymap = vec![0u8; ylen];
    for ydist in dists.1.iter() {
        ymap[*ydist as usize] += 1;
    }

    (xmap, ymap)
}

fn get_dist_map(m : &Vec<Vec<Option<Direction>>>)
                -> Vec<Vec<(Vec<u8>, Vec<u8>)>> {
    let mut dist_map : Vec<Vec<(Vec<u8>, Vec<u8>)>> = Vec::new();
    let ylen = m.len() as i32;
    let xlen = m[0].len() as i32;
    for y in 0..ylen {
        dist_map.push(Vec::new());
        for x in 0..xlen {
            dist_map.last_mut().unwrap().push(
                dist_to_map(&m, &get_distances(&m, (x, y))));
        }
    }

    dist_map
}

#[derive(Debug, Default, PartialEq, Eq)]
struct State {
    min_steps_tot : u32,
    steps : u32,
    xpos : i32,
    ypos : i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.min_steps_tot.cmp(&self.min_steps_tot)
            .then_with(|| self.steps.cmp(&other.steps))
            .then_with(|| self.xpos.cmp(&other.xpos))
            .then_with(|| self.ypos.cmp(&other.ypos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(m : &Vec<Vec<Option<Direction>>>) {
    let ylen = m.len() as i32;
    let xlen = m[0].len() as i32;
    let dm = get_dist_map(&m);

    // including (0, 0) as an option to wait
    let directions : [(i32, i32); 5] = [(-1,0), (0,-1), (0,1), (1,0), (0, 0)];
    let mut heap = BinaryHeap::new();
    heap.push(State {ypos : -1, ..Default::default()});
    while let Some(st) = heap.pop() {
        let (x, y) = (st.xpos, st.ypos);
        if y == ylen && x == xlen - 1 {
            println!("Reached end, total nbr of steps: {}", st.steps);
            return;
        }

        for (dy, dx) in directions.iter() {
            let yy = y + dy;
            let xx = x + dx;
            // Special case for exit, always ok
            if yy == ylen && xx == xlen - 1 {
                heap.push(
                    State {
                        min_steps_tot : st.steps + 1,
                        steps : st.steps + 1,
                        xpos : xx,
                        ypos : yy,
                    });
            }
            // Move
            if yy >= 0 && yy < ylen && xx >= 0 && xx < xlen {
                if dm[yy as usize][xx as usize].0[
                    ((st.steps + 1) as usize % xlen as usize)] == 0 &&
                    dm[yy as usize][xx as usize].1[
                        (st.steps + 1) as usize % ylen as usize] == 0 {
                        // no winds here so we can move
                        let min_steps_left =
                            ((ylen - yy) + (xlen - 1 - xx)) as u32;
                        heap.push(
                            State {
                                min_steps_tot : st.steps + min_steps_left,
                                steps : st.steps + 1,
                                xpos : xx,
                                ypos : yy,
                            });
                    }
            }
        }
    }

    panic!("didn't find a way out");
}

fn part2() {
}

fn main() {

    let m = get_input("input.txt");

    part1(&m);
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_distance() {
        let m = get_input("ex.txt");

        // #.######
        // #>>.<^<#
        // #.<..<<#
        // #>v.><>#
        // #<^v^^>#
        // ######.#

        assert!(get_distance_y(&m, 2, 0, 0).is_none());
        assert!(get_distance_x(&m, 0, 2, 2).is_none());

        assert_eq!(get_distance_x(&m, 0, 0, 0), Some(0));
        assert_eq!(get_distance_y(&m, 0, 0, 0), None);

        assert_eq!(get_distance_y(&m, 2, 0, 3), Some(1));
        assert_eq!(get_distance_x(&m, 0, 2, 3), Some(1));
        assert_eq!(get_distance_x(&m, 0, 2, 1), Some(1));
        assert_eq!(get_distance_x(&m, 2, 3, 0), Some(3));
        assert_eq!(get_distance_x(&m, 3, 4, 5), Some(5));
    }

    #[test]
    fn test_get_distances() {
        let m = get_input("ex.txt");

        // #.######
        // #>>.<^<#
        // #.<..<<#
        // #>v.><>#
        // #<^v^^>#
        // ######.#

        let mut xdists;
        let mut ydists;

        (xdists, ydists) = get_distances(&m, (0, 0));
        assert_eq!(xdists.len(), 4);
        assert_eq!(ydists.len(), 0);

        (xdists, ydists) = get_distances(&m, (1, 1));
        assert_eq!(xdists.len(), 3);
        assert_eq!(ydists.len(), 2);

        let xdists = xdists.iter().copied().collect::<HashSet<u32>>();
        let ydists = ydists.iter().copied().collect::<HashSet<u32>>();

        assert!(xdists.contains(&0));
        assert!(xdists.contains(&3));
        assert!(xdists.contains(&4));
        assert!(!xdists.contains(&2));
        assert!(ydists.contains(&2));
        assert!(ydists.contains(&3));
        assert!(!ydists.contains(&0));
    }

    #[test]
    fn test_get_dist_to_map() {
        let m = get_input("ex.txt");

        // #.######
        // #>>.<^<#
        // #.<..<<#
        // #>v.><>#
        // #<^v^^>#
        // ######.#

        let mut xmap;
        let ymap;
        (xmap, ymap) = dist_to_map(&m, &get_distances(&m, (1, 1)));

        assert_eq!(xmap.len(), 6);
        assert_eq!(ymap.len(), 4);

        assert_eq!(xmap[0], 1);
        assert_eq!(xmap[1], 0);
        assert_eq!(xmap[3], 1);

        assert_eq!(ymap[0], 0);
        assert_eq!(ymap[3], 1);

        (xmap, _) = dist_to_map(&m, &get_distances(&m, (2, 2)));
        assert_eq!(xmap[2], 2);
    }

    #[test]
    fn test_get_dist_map() {
        let m = get_input("ex.txt");

        // #.######
        // #>>.<^<#
        // #.<..<<#
        // #>v.><>#
        // #<^v^^>#
        // ######.#

        let dm = get_dist_map(&m);
        assert_eq!(dm[2][2].0[2], 2);
        assert_eq!(dm[0][4].1[0], 1);
        assert_eq!(dm[0][0].0[1], 0);
    }
}

