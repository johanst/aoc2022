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
struct Cube {
    pos : (i32, i32, i32),
    adj : Vec<usize>,
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

    let mut mcubes : HashMap<(i32,i32,i32), usize> = HashMap::new();
    let mut cubes : Vec<Cube> = Vec::new();
    let mut adj_cubes : HashSet<(i32,i32,i32)> = HashSet::new();
    for sc in v.iter() {
        let coords = sc.split(",")
            .map(|w| w.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let pos = (coords[0], coords[1], coords[2]);
        cubes.push(Cube {pos, adj: Vec::new()});
        mcubes.insert(pos, cubes.len() - 1);
    }
    for idx_from in 0..cubes.len() {
        let offset = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
        for (dx, dy, dz) in offset {
            let from_pos = cubes[idx_from].pos;
            let adj_pos = (from_pos.0 + dx, from_pos.1 + dy, from_pos.2 + dz);
            if let Some(idx_to) = mcubes.get(&adj_pos) {
                cubes[idx_from].adj.push(*idx_to);
            } else {
                adj_cubes.insert(adj_pos);
            }
        }
    }

    let num_free = cubes.iter()
        .fold(0, |acc, cube| acc + (6 - cube.adj.len()));

    dbg!(num_free);

    // part 2

    //dbg!(adj_cubes);
    let (mut xmax, mut ymax, mut zmax) = (0i32, 0i32, 0i32);
    for cube in cubes.iter() {
        xmax = cmp::max(xmax, cube.pos.0);
        ymax = cmp::max(ymax, cube.pos.1);
        zmax = cmp::max(zmax, cube.pos.2);
    }
    //dbg!(xmax, ymax, zmax);

    // known => true =
    let mut contained : HashSet<(i32,i32,i32)> = HashSet::new();
    let mut open_space : HashSet<(i32,i32,i32)> = HashSet::new();
    for (xstart, ystart, zstart) in adj_cubes {
        dbg!(xstart, ystart, zstart);
        let mut visited : HashSet<(i32,i32,i32)> = HashSet::new();
        let mut inspect_cubes : VecDeque<(i32,i32,i32)> = VecDeque::new();
        let mut inspect_cubes_set : HashSet<(i32,i32,i32)> = HashSet::new();
        inspect_cubes.push_back((xstart, ystart, zstart));
        // until we know the state of our
        let mut is_open_space = false;
        while !inspect_cubes.is_empty() {
            let (x, y, z) = inspect_cubes.pop_front().unwrap();
            inspect_cubes_set.remove(&(x, y, z));
            visited.insert((x, y, z));
            if x <= 0 || x >= xmax || y <= 0 || y >= ymax || z <= 0 || z >= zmax {
                is_open_space = true;
                continue;
            }
            let offset = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
            for (dx, dy, dz) in offset {
                if !(visited.contains(&(x + dx, y + dy, z + dz)) ||
                     mcubes.contains_key(&(x + dx, y + dy, z + dz)) ||
                     inspect_cubes_set.contains(&(x + dx, y + dy, z + dz))
                ) {
                    inspect_cubes.push_back((x + dx, y + dy, z + dz));
                    inspect_cubes_set.insert((x + dx, y + dy, z + dz));
                }
            }
        }

        if is_open_space {
            open_space.extend(visited);
        } else {
            contained.extend(visited);
        }
    }

    dbg!(&contained);
    dbg!(open_space.len());

    let mut num_surface_to_contained = 0;
    for idx_from in 0..cubes.len() {
        let offset = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
        let (x, y, z) = cubes[idx_from].pos;
        for (dx, dy, dz) in offset {
            let adj_pos = (x + dx, y + dy, z + dz);
            if contained.contains(&adj_pos) {
                num_surface_to_contained += 1;
            }
        }
    }

    dbg!(num_surface_to_contained);

    println!("Total number: {}", num_free - num_surface_to_contained);

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
    }
}

