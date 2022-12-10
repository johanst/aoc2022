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
enum Instruction {
    Addx(i32),
    Noop
}

fn acc(cycle : i32, x : i32, acc_prev : i32) -> i32 {
    if (cycle + 20) % 40 == 0 {
        println!("{}", x * cycle);
        acc_prev + (x * cycle)
    } else {
        acc_prev
    }
}

fn part1(prog : &Vec<Instruction>) {
    //let mut adds : VecDeque<i32> = VecDeque::new();
    let mut x = 1i32;
    let mut cycle = 0;
    let mut a = 0;
    for instr in prog.iter() {
        // let mutadd = if adds.len() > 1 { adds.pop_back().unwrap() } else { 0 };
        let add = match instr {
            Instruction::Addx(n) => *n,
            Instruction::Noop => 0i32,
        };
        cycle += 1;
        //-println!("{}    : {x}, {add}", cycle);
        a = acc(cycle, x, a);
        if add != 0 {
            cycle += 1;
            //println!("{}    : {x}, {add}", cycle);
            a = acc(cycle, x, a);
            x += add;
        }
    }

    println!("Sum: {a}");
}

fn set_sprite(cycle : i32, x : i32, row : &mut Vec<char>) {
    let hpos : i32 = (cycle - 1) % 40;
    let c = if hpos >= x - 1 && hpos <= x + 1 { '#' } else { '.' };
    row.push(c);
}

fn part2(prog : &Vec<Instruction>) {
    //let mut adds : VecDeque<i32> = VecDeque::new();
    let mut x = 1i32;
    let mut cycle = 0;
    let mut a = 0;
    let mut grid : Vec<Vec<char>> = Vec::new();
    for instr in prog.iter() {
        // let mutadd = if adds.len() > 1 { adds.pop_back().unwrap() } else { 0 };
        let add = match instr {
            Instruction::Addx(n) => *n,
            Instruction::Noop => 0i32,
        };
        if cycle % 40 == 0 {
            grid.push(Vec::new());
        }
        cycle += 1;
        //-println!("{}    : {x}, {add}", cycle);
        //a = acc(cycle, x, a);
        set_sprite(cycle, x, grid.last_mut().unwrap());
        if add != 0 {
            if cycle % 40 == 0 {
                grid.push(Vec::new());
            }
            cycle += 1;
            //println!("{}    : {x}, {add}", cycle);
            set_sprite(cycle, x, grid.last_mut().unwrap());
            //a = acc(cycle, x, a);
            x += add;
        }
    }

    for row in grid.iter() {
        for c in row.iter() {
            print!("{c}");
        }
        println!();
    }
}

fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;

    let prog : Vec<Instruction> = v.iter()
        .map(|s| {
            let mut tk = s.split_whitespace();
            match tk.next().unwrap() {
                "addx" => Instruction::Addx(tk.next().unwrap().parse::<i32>().unwrap()),
                "noop" => Instruction::Noop,
                _ => unreachable!()
            }
        })
        .collect::<Vec<Instruction>>();

    println!("{prog:?}");

    //part1(&prog);
    part2(&prog);
}
