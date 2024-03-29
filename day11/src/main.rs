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
enum Operator {
    Add,
    Mul,
}

impl Default for Operator {
    fn default() -> Self {
        Operator::Add
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(u64),
    Old
}

impl Default for Operand {
    fn default() -> Self {
        Operand::Literal(0)
    }
}

#[derive(Debug, Default, Clone)]
struct Operation {
    lhs : Operand,
    op : Operator,
    rhs : Operand,
}

impl Operation {
    fn from_string(s : &str) -> Self {
        let s = s.split_whitespace()
            .skip_while(|item| *item != "=")
            .skip(1)
            .collect::<Vec<&str>>();
        assert_eq!(s.len(), 3);
        let lhs : Operand = s[0].parse::<u64>()
            .map_or_else(|_| Operand::Old, |n| Operand::Literal(n));
        let rhs : Operand = s[2].parse::<u64>()
            .map_or_else(|_| Operand::Old, |n| Operand::Literal(n));
        let op = match s[1] {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => unreachable!(),
        };
        Self {lhs, op, rhs}
    }

    fn calculate(&self, old : u64) -> u64 {
        let lhs = Self::operand(self.lhs, old);
        let rhs = Self::operand(self.rhs, old);
        match self.op {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }

    fn operand(op : Operand, old : u64) -> u64 {
        match op {
            Operand::Literal(v) => v,
            Operand::Old => old,
        }
    }
}

#[derive(Default, Debug)]
struct Monkey {
    items : VecDeque<u64>,
    op : Operation,
    div : u64,
    to_monkey_true : u64,
    to_monkey_false : u64,
}

fn part1(m : &mut Vec<Monkey>) {
    let mut mact : Vec<u64> = vec![0; m.len()];

    for _ in 0..20 {
        // a round
        for i in 0..m.len() {
            mact[i] += m[i].items.len() as u64;

            // for all items
            while !m[i].items.is_empty() {
                let mut item = m[i].items.pop_front().unwrap();
                item = m[i].op.calculate(item);
                item /= 3;
                let new_monkey = if item % m[i].div == 0 {
                    m[i].to_monkey_true as usize
                } else {
                    m[i].to_monkey_false as usize
                };
                m[new_monkey].items.push_back(item);
            }
        }

        // Print state
        //    for i in 0..m.len() {
        //        println!("Monkey {i}: {:?}", m[i].items);
        //    }
    }

    for i in 0..m.len() {
        println!("Monkey {i}: {:?}", mact[i]);
    }
    mact.sort_by(|a, b| b.cmp(a));

    println!("Monkey business: {}", mact[0] * mact[1]);
}

fn part2(m : &mut Vec<Monkey>, gdiv : u64) {
    let mut mact : Vec<u64> = vec![0; m.len()];

    for _ in 0..10000 {
        // a round
        for i in 0..m.len() {
            mact[i] += m[i].items.len() as u64;

            // for all items
            while !m[i].items.is_empty() {
                let mut item = m[i].items.pop_front().unwrap();
                item = m[i].op.calculate(item) % gdiv;
                let new_monkey = if item % m[i].div == 0 {
                    m[i].to_monkey_true as usize
                } else {
                    m[i].to_monkey_false as usize
                };
                m[new_monkey].items.push_back(item);
            }
        }

        // Print state
        //    for i in 0..m.len() {
        //        println!("Monkey {i}: {:?}", m[i].items);
        //    }
    }

    for i in 0..m.len() {
        println!("Monkey {i}: {:?}", mact[i]);
    }
    mact.sort_by(|a, b| b.cmp(a));

    println!("Monkey business: {}", mact[0] * mact[1]);
}

fn to_u64(s : &str) -> u64 {
    s.chars()
        .map(|c| c.to_digit(10))
        .take_while(|opt| opt.is_some())
        .fold(0u64, |acc, digit| acc * 10 + digit.unwrap() as u64)
}

fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;
    let mut monkeys : Vec<Monkey> = Vec::new();
    for mv in v.chunks_exact(7) {
        let mut m = Monkey::default();
        let mut itr = mv.iter();

        itr.next(); // Monkey x
        m.items = itr.next().unwrap()
            .split_whitespace()
            .skip(2)
            .map(|item| to_u64(item))
            .collect::<VecDeque<u64>>();
        m.op = Operation::from_string(itr.next().unwrap());
        m.div = itr.next().unwrap()
            .split_whitespace()
            .last().unwrap().parse::<u64>().unwrap();
        m.to_monkey_true = itr.next().unwrap()
            .split_whitespace()
            .last().unwrap().parse::<u64>().unwrap();
        m.to_monkey_false = itr.next().unwrap()
            .split_whitespace()
            .last().unwrap().parse::<u64>().unwrap();
        //dbg!(m);

        monkeys.push(m);
    }

    let gdiv : u64 = monkeys.iter().map(|m| m.div).product();

    //dbg!(v);

    //part1(&mut monkeys);
    part2(&mut monkeys, gdiv);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
        let op = Operation::from_string("Operation: new = 5 + 2");
        assert_eq!(op.calculate(18), 7);
    }
    #[test]
    fn test_lit_mul_lit() {
        let op = Operation::from_string("Operation: new = 5 * 2");
        assert_eq!(op.calculate(18), 10);
    }
    #[test]
    fn test_old_plus_lit() {
        let op = Operation::from_string("Operation: new = old + 2");
        assert_eq!(op.calculate(18), 20);
    }
    #[test]
    fn test_lit_mul_old() {
        let op = Operation::from_string("Operation: new = 5 * old");
        assert_eq!(op.calculate(18), 90);
    }
}

