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

fn print_chamber(cha : &VecDeque<Vec<u8>>) {
    for row in cha.iter().rev() {
        for c in row.iter() {
            print!("{}", char::from_u32(*c as u32).unwrap());
        }
        println!();
    }

    let mut dummy : String = "".to_string();
    stdin().read_line(&mut dummy);
}

fn print_chamber_last_15(cha : &VecDeque<Vec<u8>>) {
    for row in cha.iter().rev().take(15) {
        for c in row.iter() {
            print!("{}", char::from_u32(*c as u32).unwrap());
        }
        println!();
    }

    let mut dummy : String = "".to_string();
    stdin().read_line(&mut dummy);
}

fn print_chamber_with_shape(cha : &VecDeque<Vec<u8>>,
                            s : &Vec<&str>,
                            xpos : i64,
                            ypos : i64) {
    //dbg!(s);
    let mut y = cha.len();
    for row in cha.iter().rev() {
        y -= 1;
        for (x, c) in row.iter().enumerate() {
            let y = y as i64;
            let x = x as i64;
            //dbg!(y, x);
            let cmod = if y >= ypos && y < ypos + s.len() as i64 &&
                x >= xpos && x < xpos + s[0].len() as i64 &&
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

fn add_row_to_chamber(cha : &mut VecDeque<Vec<u8>>) {
    let vc = "|.......|".as_bytes().to_vec();
    cha.push_back(vc);
}

fn get_no_of_empty_rows(cha : &VecDeque<Vec<u8>>) -> i64 {
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

fn can_move_to_pos(cha : &VecDeque<Vec<u8>>,
                   s : &Vec<&str>,
                   xpos : i64,
                   ypos : i64) -> bool
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

fn move_to_pos(cha : &mut VecDeque<Vec<u8>>,
               s : &Vec<&str>,
               xpos : i64,
               ypos : i64)
{
    for (dy, srow) in s.iter().enumerate() {
        for (dx, sc) in srow.chars().enumerate() {
            if sc == '#' {
                cha[dy + ypos as usize][dx + xpos as usize] = b'#';
            }
        }
    }
}

fn tower_height(cha : &VecDeque<Vec<u8>>) -> i64 {
    cha.len() as i64 - 1 - get_no_of_empty_rows(cha)
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

    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    assert_eq!(v.len(), 1);
    let v = v;
    dbg!(v[0].len());
    //dbg!(v[0]);

    let mut chamber : VecDeque<Vec<u8>> = VecDeque::new();
    let vc = "+-------+".as_bytes().to_vec();
    chamber.push_back(vc);

    //let knas = get_no_of_empty_rows(&chamber);
    //dbg!(knas);

    let mut rows_removed : u64 = 0;
    let mut rock_count : u64 = 0;
    let mut jet_count : u64 = 0;
    let mut shitr = shapes.iter().cycle();
    let mut jetitr = v[0].chars().cycle();
    // 13345
    // for _ in 0..2022 {
    //for _ in 0..(5 * 2 * v[0].len() + 1) {
    //for _ in 0..13345 {
    loop {
        let shape = shitr.next().unwrap();

        // make sure we have three empty rows and enough row for our shape
        let num_empty = get_no_of_empty_rows(&chamber);
        //dbg!(num_empty);
        let addc = 3 - get_no_of_empty_rows(&chamber) + shape.len() as i64;
        if addc < 0 {
            for _ in 0..-addc {
                chamber.pop_back();
            }
        } else {
            for _ in 0..addc {
                add_row_to_chamber(&mut chamber);
            }
        }
        let mut xpos = 3;

        let bottom = chamber.pop_front().unwrap();
        while chamber.len() > 128 {
            chamber.pop_front();
            rows_removed += 1
        }
        chamber.push_front(bottom);

        let mut ypos = chamber.len() as i64 - shape.len() as i64;

        //print_chamber_with_shape(&chamber, &shape, xpos, ypos);
        loop {
            // do the jet
            let jet = jetitr.next().unwrap();
            jet_count += 1;
            let dx = match jet {
                '<' => -1,
                '>' => 1,
                _ => unreachable!(),
            };
            if can_move_to_pos(&chamber, &shape, xpos + dx, ypos) {
                xpos += dx;
            }
            //print_chamber_with_shape(&chamber, &shape, xpos, ypos);

            // try to move down
            if !can_move_to_pos(&chamber, &shape, xpos, ypos - 1) {
                move_to_pos(&mut chamber, &shape, xpos, ypos);
                //print_chamber(&chamber);
                break;
            } else {
                ypos -= 1;
                //print_chamber_with_shape(&chamber, &shape, xpos, ypos);
            }

            //println!("Tower height: {}", tower_height(&chamber));
        }

        rock_count += 1;
        if rock_count % 5 == 0 && jet_count % v[0].len() as u64 == 0 {
            break;
        }

        if rock_count == 1_000_000_000_000 {
            break;
        }

        if rock_count % 1_000_000 == 0 {
            println!("{rock_count} {jet_count}");
        }

        //let jet = jetitr.next().unwrap();
    }

    print_chamber_last_15(&chamber);
    println!("Rock count: {}", rock_count);
    println!("Tower height: {}", tower_height(&chamber) + rows_removed as i64);

    // 79089 height for 5 * 10091 (jet length * shape )
    let ncyc : u64 = 1000000000000 / ( 5 * 10091 ); // 19819641
    let rcyc : u64 = 1000000000000 % ( 5 * 10091 ); // 13345
    // 20945 height for remainder 13345

    // 608 height for 5 * 2 * 40 ( jet length * shape)
    let exncyc : u64 = 1000000000000 / ( 5 * 2 * 40 ); // 2500000000
    let exrcyc : u64 = 1000000000000 % ( 5 * 2 * 40 ); //

    dbg!(ncyc, rcyc);
    dbg!(exncyc, exrcyc);
    let total : u64 = 19819641 * 79089 + 20945;
    dbg!(total);
    let extotal : u64 = 2500000000 * 608;
    dbg!(extotal);
    // 1567515607994 too low

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

