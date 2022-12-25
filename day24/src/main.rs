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

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct StateKey {
    steps : u32,
    xpos : i32,
    ypos : i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.min_steps_tot.cmp(&self.min_steps_tot)
            .then_with(|| other.steps.cmp(&self.steps))
            .then_with(|| other.xpos.cmp(&self.xpos))
            .then_with(|| other.ypos.cmp(&self.ypos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(m : &Vec<Vec<Option<Direction>>>,
                 steps_init : u32,
                 from : (i32, i32),
                 to : (i32, i32)) -> u32 {
    let ylen = m.len() as i32;
    let xlen = m[0].len() as i32;
    let dm = get_dist_map(&m);

    let mut vis : HashMap<StateKey, u32> = HashMap::new();

    let directions : [(i32, i32); 4] = [(-1,0), (0,-1), (0,1), (1,0)];
    let mut heap = BinaryHeap::new();
    heap.push(State {
        min_steps_tot : 0,
        steps : steps_init,
        xpos : from.0,
        ypos : from.1,
    });
    vis.insert(
        StateKey {
            steps : steps_init % (xlen as u32 * ylen as u32),
            xpos : from.0,
            ypos : from.1,
        }, 0);
    while let Some(st) = heap.pop() {
        //dbg!(&st, heap.len());
        let (x, y) = (st.xpos, st.ypos);
        if y == to.1 && x == to.0 {
            println!("Reached end, total nbr of steps: {}", st.steps);
            return st.steps;
        }

        for (dy, dx) in directions.iter() {
            let yy = y + dy;
            let xx = x + dx;
            // Special case for exit, always ok
            if yy == to.1 && xx == to.0 {
                heap.push(
                    State {
                        min_steps_tot : st.steps + 1,
                        steps : st.steps + 1,
                        xpos : xx,
                        ypos : yy,
                    });
            }
            // Move
            //dbg!(xx,yy,&vis);
            let skey = StateKey {
                steps : (st.steps + 1) % (xlen as u32 * ylen as u32),
                xpos : xx,
                ypos : yy,
            };
            if yy >= 0 && yy < ylen && xx >= 0 && xx < xlen {
                if  dm[yy as usize][xx as usize].0[
                    ((st.steps + 1) as usize % xlen as usize)] == 0 &&
                    dm[yy as usize][xx as usize].1[
                        (st.steps + 1) as usize % ylen as usize] == 0 &&
                    st.steps + 1 < *vis.get(&skey).unwrap_or(&(u32::MAX))
                {
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
                    vis.insert(skey, st.steps + 1);
                }
            }
        }
        // wait is also an option... and specifically at start pos
        let skey = StateKey {
            steps : (st.steps + 1) % (xlen as u32 * ylen as u32),
            xpos : x,
            ypos : y,
        };
        if (y == from.1 && x == from.0) ||
            (dm[y as usize][x as usize].0[
            ((st.steps + 1) as usize % xlen as usize)] == 0 &&
            dm[y as usize][x as usize].1[
                (st.steps + 1) as usize % ylen as usize] == 0) &&
            st.steps + 1 < *vis.get(&skey).unwrap_or(&(u32::MAX))
        {
            // no winds here so we can move
            let min_steps_left =
                ((ylen - y) + (xlen - 1 - x)) as u32;
            heap.push(
                State {
                    min_steps_tot : st.steps + min_steps_left,
                    steps : st.steps + 1,
                    xpos : x,
                    ypos : y,
                });
            vis.insert(skey, st.steps + 1);
        }
    }

    panic!("didn't find a way out");
}

#[derive(Debug, PartialEq, Clone)]
enum Action {
    Init,
    Move(usize),
    Wait,
    Done,
}

impl Default for Action {
    fn default() -> Self {
        Action::Init
    }
}

#[derive(Debug, Default, Clone)]
struct StateD {
    steps : u32,
    xpos : i32,
    ypos : i32,
    action : Action
}

fn part1_depth_first(m : &Vec<Vec<Option<Direction>>>) {
    let ylen = m.len() as i32;
    let xlen = m[0].len() as i32;
    let dm = get_dist_map(&m);

    let mut stack : Vec<StateD> = Vec::new();
    let mut stats : Vec<u64> = Vec::new();
    let mut rptcnt : u64 = 0;

//   let mut vis : HashMap<(i32,i32),u32> = HashMap::new();

    let directions : [(i32, i32); 4] = [(0,1), (1,0), (-1,0), (0,-1)];
    //    let mut heap = BinaryHeap::new();
    stack.push(StateD {ypos : -1, ..Default::default()});
    let mut min_steps : u32 = u32::MAX;
    //heap.push(State {ypos : -1, ..Default::default()});
    while let Some(mut st) = stack.pop() {
        //dbg!(&st, stack.len());
        //let mut dummy : String = "".to_string();
        //stdin().read_line(&mut dummy);
        while stats.len() < stack.len() + 1 {
            stats.push(0);
        }
        stats[stack.len()] += 1;
        rptcnt += 1;
        if rptcnt % 100_000_1000 == 0 {
            println!("Repeats: {}", rptcnt);
            dbg!(&stats);
        }

        let (x, y) = (st.xpos, st.ypos);
        if y == ylen && x == xlen - 1 {
            if st.steps < min_steps {
                println!("Reached end, total nbr of steps: {}", st.steps);
                min_steps = st.steps;
            }
            continue;
        }
        loop {
            let min_steps_left = ((ylen - y) + (xlen - 1 - x)) as u32;
            if min_steps_left + st.steps >= min_steps {
                break;
            }

            st.action = match st.action {
                Action::Init => Action::Move(0),
                Action::Move(0) => Action::Move(1),
                Action::Move(1) => Action::Wait,
                Action::Wait => Action::Move(2),
                Action::Move(2) => Action::Move(3),
                Action::Move(3) => Action::Done,
                _ => unreachable!(),
            };
            if st.action == Action::Done {
                break;
            }
            stack.push(st.clone());
            match st.action {
                Action::Move(dir) => {
                    let yy = y + directions[dir].0;
                    let xx = x + directions[dir].1;
                    let yidx = yy as usize;
                    let xidx = xx as usize;
                    let su = st.steps as usize;
                    let xmod = xlen as usize;
                    let ymod = ylen as usize;
                    //println!("  try move {yy}, {xx}");
                    if (yy == ylen && xx == xlen - 1) ||
                        (yy >= 0 && yy < ylen && xx >= 0 && xx < xlen &&
                        dm[yidx][xidx].0[(su + 1) % xmod] == 0 &&
                        dm[yidx][xidx].1[(su + 1) % ymod] == 0 &&
                         min_steps_left + st.steps < min_steps) {
                            //println!("    moving {yy}, {xx}");
                            st.steps += 1;
                            st.action = Action::Init;
                            st.xpos = xx;
                            st.ypos = yy;
                            stack.push(st);
                            break;
                        }
                },
                Action::Wait => {
                    let yidx = y as usize;
                    let xidx = x as usize;
                    let su = st.steps as usize;
                    let xmod = xlen as usize;
                    let ymod = ylen as usize;
                    if ((y == -1 && x == 0) ||
                        dm[yidx][xidx].0[(su + 1) % xmod] == 0 &&
                        dm[yidx][xidx].1[(su + 1) % ymod] == 0) &&
                        min_steps_left + st.steps < min_steps {
                            st.steps += 1;
                            st.action = Action::Init;
                            st.xpos = x;
                            st.ypos = y;
                            stack.push(st);
                            break;
                        }
                },
                _ => unreachable!(),
            }
        }
    }

    if min_steps == u32::MAX {
        panic!("didn't find a way out");
    }
}

fn part2() {
}

fn main() {

    let m = get_input("input.txt");
    let ylen = m.len() as i32;
    let xlen = m[0].len() as i32;
    let from = (0, -1);
    let to = (xlen - 1, ylen);

    //part1(474 too high)
    let steps_1 = shortest_path(&m, 0, from, to);
    let steps_2 = shortest_path(&m, steps_1, to, from);
    let steps_3 = shortest_path(&m, steps_2, from, to);

    println!("Total nbr of steps: {}", steps_3);
    //part1_depth_first(&m);
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

