#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

#[derive(Debug)]
struct Num {
    n : i64,
    next : usize,
    prev : usize,
}

fn modulo(x : i64, n : usize) -> usize {
   (((x % n as i64) + n as i64) % n as i64) as usize
}

fn part1() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut file : Vec<Num> = Vec::new();
    let mut idx_0 = 0;
    for (i, n) in v.iter().map(|s| s.parse::<i64>().unwrap()).enumerate() {
        if n == 0 {
            idx_0 = i;
        }
        let prev = if i == 0 { 0 } else { i - 1 };
        let next = i + 1;
        file.push(Num {n, next, prev});
    }

    let flen = file.len();
    file[0].prev = flen - 1;
    file[flen - 1].next = 0;

    //print_seq(idx_0, &file);

    mix(&mut file);

    //let mut idx = idx_0;
    //for _ in 0..flen {
    //    print!("{} ", file[idx].n);
    //    idx = file[idx].next;
    //}
    //println!();

    let n_1000 = get_nth(1000, idx_0, &file);
    let n_2000 = get_nth(2000, idx_0, &file);
    let n_3000 = get_nth(3000, idx_0, &file);

    dbg!(n_1000, n_2000, n_3000);
    println!("Sum of coords: {}", n_1000 + n_2000 + n_3000);
    // 6664 too low
}

fn mix(file : &mut Vec<Num>) {
    let flen = file.len();
    for idx in 0..flen {
        if file[idx].n.abs() as usize % (flen - 1) == 0 {
            continue;
        }

        let old_next_idx = file[idx].next;
        let old_prev_idx = file[idx].prev;
        // unlink from old
        file[old_prev_idx].next = old_next_idx;
        file[old_next_idx].prev = old_prev_idx;

        let new_prev_idx = if file[idx].n > 0 {
            // right
            let mut idx_cur = idx;
            for _ in 0..(file[idx].n as usize % (flen - 1)) {
                idx_cur = file[idx_cur].next;
            }
            idx_cur
        } else {
            // left
            let mut idx_cur = file[idx].prev;
            //dbg!(-file[idx].n);
            for _ in 0..(file[idx].n.abs()) as usize % (flen - 1) {
                idx_cur = file[idx_cur].prev;
                //dbg!(idx_cur);
            }
            idx_cur
        };
        assert!(new_prev_idx != idx);
        if new_prev_idx == idx {
            continue;
        }
        //dbg!(new_prev_idx);
        let new_next_idx = file[new_prev_idx].next;

        // link to new
        file[idx].next = new_next_idx;
        file[idx].prev = new_prev_idx;
        file[new_prev_idx].next = idx;
        file[new_next_idx].prev = idx;

        //println!("{idx}, n = {}", file[idx].n);
        //print_seq(idx_0, &file);
        //dbg!(&file);
        //println!();
    }
}

fn part2() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let mut file : Vec<Num> = Vec::new();
    let mut idx_0 = 0;
    for (i, n) in v.iter().map(|s| s.parse::<i64>().unwrap()).enumerate() {
        if n == 0 {
            idx_0 = i;
        }
        let prev = if i == 0 { 0 } else { i - 1 };
        let next = i + 1;
        file.push(Num {n: n * 811589153, next, prev});
    }

    let flen = file.len();
    file[0].prev = flen - 1;
    file[flen - 1].next = 0;

    for _ in 0..10 {
        mix(&mut file);
    }

    //print_seq(idx_0, &file);


    //let mut idx = idx_0;
    //for _ in 0..flen {
    //    print!("{} ", file[idx].n);
    //    idx = file[idx].next;
    //}
    //println!();

    let n_1000 = get_nth(1000, idx_0, &file);
    let n_2000 = get_nth(2000, idx_0, &file);
    let n_3000 = get_nth(3000, idx_0, &file);

    dbg!(n_1000, n_2000, n_3000);
    println!("Sum of coords: {}", n_1000 + n_2000 + n_3000);
    // 6664 too low
}

fn get_nth(n : usize, idx_0: usize, file: &Vec<Num>) -> i64{
    let mut idx_cur = idx_0;
    for _ in 0..(n % file.len()) {
        idx_cur = file[idx_cur].next;
    }
    file[idx_cur].n
}

fn print_seq(idx_0: usize, file: &Vec<Num>) {
    let mut idx = idx_0;
    for _ in 0..file.len() {
        print!("{} ", file[idx].n);
        idx = file[idx].next;
    }
    println!();
}

fn main() {

    //part1();
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mod() {
        assert_eq!(((-2 % 7) + 7) % 7, 5);
        assert_eq!(modulo(-13, 7), 1);
    }
}

