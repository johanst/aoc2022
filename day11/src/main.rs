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
    Literal(u32),
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
        let lhs : Operand = s[0].parse::<u32>()
            .map_or_else(|_| Operand::Old, |n| Operand::Literal(n));
        let rhs : Operand = s[2].parse::<u32>()
            .map_or_else(|_| Operand::Old, |n| Operand::Literal(n));
        let op = match s[1] {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => unreachable!(),
        };
        Self {lhs, op, rhs}
    }

    fn calculate(&self, old : u32) -> u32 {
        let lhs = Self::operand(self.lhs, old);
        let rhs = Self::operand(self.rhs, old);
        match self.op {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }

    fn operand(op : Operand, old : u32) -> u32 {
        match op {
            Operand::Literal(v) => v,
            Operand::Old => old,
        }
    }
}

#[derive(Default, Debug)]
struct Monkey {
    items : VecDeque<u32>,
    op : Operation,
    div : u32,
    to_monkey_true : u32,
    to_monkey_false : u32,
}

fn part1() {
}

fn part2() {
}

fn to_u32(s : &str) -> u32 {
    s.chars()
        .map(|c| c.to_digit(10))
        .take_while(|opt| opt.is_some())
        .fold(0, |acc, digit| acc * 10 + digit.unwrap())
}

fn main() {
    let lines = std::fs::read_to_string("ex.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;
    for mv in v.chunks_exact(7) {
        let mut m = Monkey::default();
        let mut itr = mv.iter();

        itr.next(); // Monkey x
        m.items = itr.next().unwrap()
            .split_whitespace()
            .skip(2)
            .map(|item| to_u32(item))
            .collect::<VecDeque<u32>>();
        m.op = Operation::from_string(itr.next().unwrap());
        m.div = itr.next().unwrap()
            .split_whitespace()
            .last().unwrap().parse::<u32>().unwrap();
        m.to_monkey_true = itr.next().unwrap()
            .split_whitespace()
            .last().unwrap().parse::<u32>().unwrap();
        m.to_monkey_false = itr.next().unwrap()
            .split_whitespace()
            .last().unwrap().parse::<u32>().unwrap();
        dbg!(m);
    }

    //dbg!(v);

    part1();
    part2();
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

