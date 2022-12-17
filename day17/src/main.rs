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

fn print_chamber(cha : &Vec<Vec<u8>>) {
    for row in cha.iter().rev() {
        for c in row.iter() {
            print!("{}", char::from_u32(*c as u32).unwrap());
        }
        println!();
    }

    let mut dummy : String = "".to_string();
    stdin().read_line(&mut dummy);
}

fn print_chamber_with_shape(cha : &Vec<Vec<u8>>,
                            s : &Vec<&str>,
                            xpos : i32,
                            ypos : i32) {
    //dbg!(s);
    let mut y = cha.len();
    for row in cha.iter().rev() {
        y -= 1;
        for (x, c) in row.iter().enumerate() {
            let y = y as i32;
            let x = x as i32;
            //dbg!(y, x);
            let cmod = if y >= ypos && y < ypos + s.len() as i32 &&
                x >= xpos && x < xpos + s[0].len() as i32 &&
                s[(y - ypos) as usize].as_bytes()[(x - xpos) as usize] == b'#'  {
                '@'
            } else {
                char::from_u32(*c as u32).unwrap()
            };
            print!("{}", cmod);
        }
        println!();
    }

    let mut dummy : String = "".to_string();
    stdin().read_line(&mut dummy);
}

fn add_row_to_chamber(cha : &mut Vec<Vec<u8>>) {
    let vc = "|.......|".as_bytes().to_vec();
    cha.push(vc);
}

fn get_no_of_empty_rows(cha : &Vec<Vec<u8>>) -> i32 {
    let vc = "|.......|".as_bytes().to_vec();
    let mut count = 0;
    for row in cha.iter().rev() {
        if *row == vc {
            count += 1;
        } else {
            return count;
        }
    }
    count
}

fn can_move_to_pos(cha : &Vec<Vec<u8>>,
                   s : &Vec<&str>,
                   xpos : i32,
                   ypos : i32) -> bool
{
    for (dy, srow) in s.iter().enumerate() {
        for (dx, sc) in srow.chars().enumerate() {
            if sc == '#' && cha[dy + ypos as usize][dx + xpos as usize] != b'.' {
                return false;
            }
        }
    }

    return true;
}

fn move_to_pos(cha : &mut Vec<Vec<u8>>,
               s : &Vec<&str>,
               xpos : i32,
               ypos : i32)
{
    for (dy, srow) in s.iter().enumerate() {
        for (dx, sc) in srow.chars().enumerate() {
            if sc == '#' {
                cha[dy + ypos as usize][dx + xpos as usize] = b'#';
            }
        }
    }
}

fn main() {
    let lines = std::fs::read_to_string("shapes.txt").unwrap();
    let v = lines.split("\n\n").collect::<Vec<&str>>();
    let mut shapes : Vec<Vec<&str>> = Vec::new();
    for s in v.iter() {
        let mut shape = s.split("\n").collect::<Vec<&str>>();
        if shape[shape.len() - 1] == "" {
            shape.pop();
        }
        shape.reverse(); // y grows "upwards"
        shapes.push(shape);
    }
    //dbg!(&shapes);

    let lines = std::fs::read_to_string("ex.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    assert_eq!(v.len(), 1);
    let v = v;

    let mut chamber : Vec<Vec<u8>> = Vec::new();
    let vc = "+-------+".as_bytes().to_vec();
    chamber.push(vc);

    //let knas = get_no_of_empty_rows(&chamber);
    //dbg!(knas);

    let mut shitr = shapes.iter().cycle();
    let mut jetitr = v[0].chars().cycle();
    for _ in 0..2022 {
        let shape = shitr.next().unwrap();

        // make sure we have three empty rows and enough row for our shape
        let num_empty = get_no_of_empty_rows(&chamber);
        dbg!(num_empty);
        let addc = 3 - get_no_of_empty_rows(&chamber) + shape.len() as i32;
        if addc < 0 {
            for _ in 0..-addc {
                chamber.pop();
            }
        } else {
            for _ in 0..addc {
                add_row_to_chamber(&mut chamber);
            }
        }
        let mut xpos = 3;
        let mut ypos = chamber.len() as i32 - shape.len() as i32;

        print_chamber_with_shape(&chamber, &shape, xpos, ypos);
        loop {
            // do the jet
            let jet = jetitr.next().unwrap();
            let dx = match jet {
                '<' => -1,
                '>' => 1,
                _ => unreachable!(),
            };
            if can_move_to_pos(&chamber, &shape, xpos + dx, ypos) {
                xpos += dx;
            }
            print_chamber_with_shape(&chamber, &shape, xpos, ypos);

            // try to move down
            if !can_move_to_pos(&chamber, &shape, xpos, ypos - 1) {
                move_to_pos(&mut chamber, &shape, xpos, ypos);
                print_chamber(&chamber);
                break;
            } else {
                ypos -= 1;
                print_chamber_with_shape(&chamber, &shape, xpos, ypos);
            }
        }

        //let jet = jetitr.next().unwrap();
    }

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

