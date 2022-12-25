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

fn get_top_heights(cha : &VecDeque<Vec<u8>>) -> u64 {
    let mut v : [u8; 7] = [255; 7];
    for col in 1..8 {
        for (idx, r) in cha.iter().rev().enumerate() {
            if r[col] == b'#' {
                v[col - 1] = idx as u8;
                break;
            }
        }
    }
    let min = *v.iter().min().unwrap();
    let mut r = 0;
    for x in v.iter() {
        r *= 256;
        r +=  (*x - min) as u64
    }

    r
}

#[derive(Debug, Clone, Default)]
struct TowerInfo {
    height : i64,
    rock_count : u64,
    jet_count : u64,
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

    let mut twi0 = TowerInfo::default();
    let mut twi1 = TowerInfo::default();
    let mut twi2 = TowerInfo::default();

    //let knas = get_no_of_empty_rows(&chamber);
    //dbg!(knas);

    let mut rows_removed : u64 = 0;
    let mut rock_count : u64 = 0;
    let mut jet_count : u64 = 0;
    let mut shitr = shapes.iter().cycle();
    let mut jetitr = v[0].chars().cycle();

    // map jet_cycle -> map shape cycle
    let mut cycle_finder : Vec<HashMap<(u32, u64),TowerInfo>> =
        vec![HashMap::new(); v[0].len()];
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

        let height_map = get_top_heights(&chamber);

        let jet_idx : usize = jet_count as usize % v[0].len();
        let rock_key : (u32, u64) = (
            (rock_count % 5) as u32, height_map);
        if cycle_finder[jet_idx].contains_key(&rock_key) {
            twi0 = cycle_finder[jet_idx].get(&rock_key).unwrap().clone();
            twi1 = TowerInfo {
                height : tower_height(&chamber) + rows_removed as i64,
                rock_count,
                jet_count
            };
            //dbg!(jet_count, rock_count, tower_height(&chamber));
            //dbg!(cycle_finder[jet_idx].get(&rock_key).unwrap());
            break;
        } else {
            cycle_finder[jet_idx].insert(
                rock_key,
                TowerInfo {
                    height : tower_height(&chamber) + rows_removed as i64,
                    rock_count,
                    jet_count
                });
        }

        if rock_count >= 98 + 42 && twi2.height == 0 {
            twi2 = TowerInfo {
                height : tower_height(&chamber) + rows_removed as i64,
                rock_count,
                jet_count
            };
        }

//        if rock_count == 98 {
//            println!("At rock count 98, heights = {:X}", height_map);
//            print_chamber_last_15(&chamber);
//        }

    }

    print_chamber_last_15(&chamber);
    println!("Rock count: {}", rock_count);
    println!("Tower height: {}", tower_height(&chamber) + rows_removed as i64);

    dbg!(&twi0);
    dbg!(&twi1);
    dbg!(&twi2);

    let ncyc : u64 = (1000000000000 - twi0.rock_count) / (
        twi1.rock_count - twi0.rock_count);
    let rcyc : u64 = (1000000000000 - twi0.rock_count) % (
        twi1.rock_count - twi0.rock_count);

    dbg!(ncyc, rcyc);
//    dbg!(exncyc, exrcyc);
//    let total : u64 = 19819641 * 79089 + 20945;
//    dbg!(total);
//    let extotal : u64 = 2500000000 * 608;
//    dbg!(extotal);
    //    // 1567515607994 too low

    let total : u64 = twi0.height as u64 +
        ncyc * (twi1.height - twi0.height) as u64 +
        (twi2. height - twi0.height) as u64;
    dbg!(total);

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

