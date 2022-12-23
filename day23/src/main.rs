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
struct Range {
    xmin : i32,
    xmax : i32,
    ymin : i32,
    ymax : i32,
}

impl Default for Range {
    fn default() -> Self {
        Range {
            xmin : i32::MAX,
            xmax : i32::MIN,
            ymin : i32::MAX,
            ymax : i32::MIN,
        }
    }
}

fn get_range(elves : &HashSet<(i32, i32)>) -> Range {
    let mut rng = Range::default();
    for elf in elves.iter() {
        rng.xmin = cmp::min(rng.xmin, elf.0);
        rng.xmax = cmp::max(rng.xmax, elf.0);
        rng.ymin = cmp::min(rng.ymin, elf.1);
        rng.ymax = cmp::max(rng.ymax, elf.1);
    }
    rng
}

fn draw_elves(elves : &HashSet<(i32, i32)>) {
    let rng = get_range(elves);
    println!("-- xmin: {}, ymin: {}", rng.xmin, rng.ymin);
    for y in rng.ymin..=rng.ymax {
        for x in rng.xmin..=rng.xmax {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn try_move_in_direction(
    elves : &HashSet<(i32, i32)>,
    elf : (i32, i32),
    direction : usize) -> Option<(i32, i32)> {
    match direction {
        0 => {
            // north
            for x in -1..=1 {
                if elves.contains(&(elf.0 + x, elf.1 - 1)) {
                    println!("({},{}) cannot move north", elf.0, elf.1);
                    return None;
                }
            }
            println!("({},{}) -> ({},{}) (NORTH)", elf.0, elf.1, elf.0, elf.1 - 1);
            Some((elf.0, elf.1 - 1))
        },
        1 => {
            // south
            for x in -1..=1 {
                if elves.contains(&(elf.0 + x, elf.1 + 1)) {
                    println!("({},{}) cannot move south", elf.0, elf.1);
                    return None;
                }
            }
            println!("({},{}) -> ({},{}) (SOUTH)", elf.0, elf.1, elf.0, elf.1 + 1);
            Some((elf.0, elf.1 + 1))
        },
        2 => {
            // west
            for y in -1..=1 {
                if elves.contains(&(elf.0 - 1, elf.1 + y)) {
                    println!("({},{}) cannot move west", elf.0, elf.1);
                    return None;
                }
            }
            println!("({},{}) -> ({},{}) (WEST)", elf.0, elf.1, elf.0 - 1, elf.1);
            Some((elf.0 - 1, elf.1))
        },
        3 => {
            // east
            for y in -1..=1 {
                if elves.contains(&(elf.0 + 1, elf.1 + y)) {
                    println!("({},{}) cannot move east", elf.0, elf.1);
                    return None;
                }
            }
            println!("({},{}) -> ({},{}) (EAST)", elf.0, elf.1, elf.0 + 1, elf.1);
            Some((elf.0 + 1, elf.1))
        },
        _ => unreachable!(),
    }
}

fn move_elves(elves : &HashSet<(i32, i32)>, direction : usize) -> HashSet<(i32, i32)> {
    // map from -> to
    let mut elf_move : HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    // map to -> nbr of elves
    let mut elf_pos : HashMap<(i32, i32), u32> = HashMap::new();

    for elf in elves.iter() {
        let mut has_neighbours = false;
        'outer: for y in -1..=1 {
            for x in -1..=1 {
                //println!("   check ({},{})", elf.0 + x, elf.0 + y);
                if !(x == 0 && y == 0) && elves.contains(&(elf.0 + x, elf.1 + y)) {
                    has_neighbours = true;
                    break 'outer;
                }
            }
        };

        let mut pos_new = *elf;
        if has_neighbours {
            let mut offset = 0;
            pos_new = loop {
                if let Some(pos) = try_move_in_direction(
                    elves, *elf, (direction + offset) % 4) {
                    break pos;
                }
                offset += 1;
                offset %= 4;
                if offset == 0 {
                    break *elf;
                }
            }
        } else {
            println!("({},{}) has no neighbours", elf.0, elf.1);
        }

        elf_move.insert(*elf, pos_new);
        *elf_pos.entry(pos_new).or_insert(0) += 1;
    }

    let mut elves_new : HashSet<(i32, i32)> = HashSet::new();
    for (elf_old, elf_new) in elf_move.iter() {
        if *elf_pos.get(elf_new).unwrap() == 1 {
            elves_new.insert(*elf_new);
        } else {
            println!("({},{}) not allowed to move", elf_old.0, elf_old.1);
            elves_new.insert(*elf_old);
        }
    }

    elves_new
}

fn part1(elves : &HashSet<(i32, i32)>) {
    draw_elves(elves);
    let mut elves = elves.clone();
    let mut direction = 0;
    for _ in 0..3 {
        println!();
        elves = move_elves(&elves, direction);
        draw_elves(&elves);
        direction += 1;
        direction %= 4;
    }
}

fn part2() {
}

fn main() {
    let lines = std::fs::read_to_string("exsmall.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut elves : HashSet<(i32, i32)> = HashSet::new();
    for (y, r) in v.iter().enumerate() {
        for (x, c) in r.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    //draw_elves(&elves);

    //dbg!(v);

    part1(&elves);
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
    }
}

