#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;

fn part1() {
}

fn part2() {
}

fn snafu_to_dec(s : &str) -> u64 {
    let mut n = 0i64;
    let mut mul = 1;
    for c in s.chars().rev() {
        match c {
            c @ '0'..='2' => n += c.to_digit(10).unwrap() as i64 * mul,
            '-' => n -= mul,
            '=' => n -= 2 * mul,
            _ => unreachable!()
        };
        mul *= 5;
    }

    n as u64
}

fn dec_to_snafu(mut n : u64) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut out : Vec<char> = Vec::new();
    let mut tmp : Vec<u8> = Vec::new();
    while n != 0 {
        tmp.push((n % 5) as u8);
        n /= 5;
    }
    tmp.push(0);
    for idx in 0..tmp.len() {
        let n = tmp[idx];
        if n <= 2 {
            out.push(char::from_digit(n as u32, 10).unwrap());
        } else {
            if n == 3 {
                out.push('=');
            } else {
                out.push('-');
            }
            for idxh in idx+1..tmp.len() {
                tmp[idxh] += 1;
                if tmp[idxh] != 5 {
                    break;
                }
                tmp[idxh] = 0;
            }
        }
    }

    // dbg!(&out, &tmp);

    out.iter().rev().skip_while(|c| **c == '0').collect::<String>()
}

fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;
    let d : u64 = v.iter().map(|&s| snafu_to_dec(s)).sum();
    println!("Sum: {d}");
    let dsnafu = dec_to_snafu(d);
    println!("In snafu: {dsnafu}");

    part1();
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snafu_to_dec() {
        assert_eq!(snafu_to_dec("1"), 1);

        assert_eq!(snafu_to_dec("1121-1110-1=0"), 314159265);
        assert_eq!(snafu_to_dec("1-0---0"), 12345);
    }

    #[test]
    fn test_dec_to_snafu() {
        assert_eq!(dec_to_snafu(1), "1");

        assert_eq!(dec_to_snafu(314159265), "1121-1110-1=0");
        assert_eq!(dec_to_snafu(12345), "1-0---0");
    }
}

