#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::fmt;
use std::io::stdin;
use std::iter;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Place {
    Outside,
    Path,
    Wall
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Walk(i32),
    Turn(i32),
}

#[derive(Debug, Clone, Copy)]
struct Actor {
    xpos : i32,
    ypos : i32,
    xdir : i32,
    ydir : i32,
}

#[derive(Debug)]
struct Config {
    m : Vec<Vec<Place>>,
    myrng : Vec<(usize,usize)>,
    mxrng : Vec<(usize,usize)>,
    xsize : usize,
    ysize : usize,
    cube_size : usize,
}

fn draw_map(cfg : &Config, actor : &Actor) {
    for y in 0..cfg.ysize {
        for x in 0..cfg.xsize {
            let c = if (x,y) == (actor.xpos as usize, actor.ypos as usize) {
                match (actor.xdir, actor.ydir) {
                    (1, 0) => '>',
                    (-1, 0) => '<',
                    (0, 1) => 'v',
                    (0, -1) => '^',
                    _ => unreachable!(),
                }
            } else {
                match cfg.m[y][x] {
                    Place::Outside => ' ',
                    Place::Wall => '#',
                    Place::Path => '.',
                }
            };
            print!("{c}");
        }
        println!();
    }
}

fn walk(mv : Move, cfg : &Config, actor : &Actor) -> Actor {
    let mut anew = *actor;
    match mv {
        Move::Turn(dir) => {
            let dirs : [(i32, i32); 4] = [
                (1, 0), (0, 1), (-1, 0), (0, -1)];
            let dpos = dirs.iter()
                .position(|&x| x == (actor.xdir, actor.ydir)).unwrap();
            let dpos_new = (dpos as i32 + 4 + dir) as usize % 4;
            anew.xdir = dirs[dpos_new].0;
            anew.ydir = dirs[dpos_new].1;
        }
        Move::Walk(mut steps) => {
            while steps != 0 {
                let mut xpnew = anew.xpos + actor.xdir;
                let mut ypnew = anew.ypos + actor.ydir;
                let ypos = actor.ypos as usize;
                let xpos = actor.xpos as usize;
                // Bounds check
                if actor.ydir == 0 {
                    if xpnew < cfg.mxrng[ypos].0 as i32{
                        xpnew = cfg.mxrng[ypos].1 as i32;
                    } else if xpnew > cfg.mxrng[ypos].1 as i32 {
                        xpnew = cfg.mxrng[ypos].0 as i32;
                    }
                } else if actor.xdir == 0 as i32 {
                    if ypnew < cfg.myrng[xpos].0 as i32 {
                        ypnew = cfg.myrng[xpos].1 as i32;
                    } else if ypnew > cfg.myrng[xpos].1 as i32 {
                        ypnew = cfg.myrng[xpos].0 as i32;
                    }
                } else {
                    unreachable!();
                }
                // check if wall
                match cfg.m[ypnew as usize][xpnew as usize] {
                    Place::Outside => unreachable!(),
                    Place::Wall => break,
                    Place::Path => steps -= 1,
                };

                anew.xpos = xpnew;
                anew.ypos = ypnew;
            }
        }
    }

    anew
}

#[derive(Debug, PartialEq)]
enum Face {
    Right,
    Down,
    Left,
    Up,
}

fn dir2face(dir : (i32, i32)) -> Face {
    match dir {
        (1, 0) => Face::Right,
        (0, 1) => Face::Down,
        (-1, 0) => Face::Left,
        (0, -1) => Face::Up,
        _ => unreachable!()
    }
}

fn face2dir(face : Face) -> (i32, i32) {
    match face {
        Face::Right => (1, 0),
        Face::Down => (0, 1),
        Face::Left => (-1, 0),
        Face::Up => (0, -1),
    }
}

fn cube_walk(mv : Move, cfg : &Config, actor : &Actor) -> Actor {
    let mut anew = *actor;
    match mv {
        Move::Turn(dir) => {
            let dirs : [(i32, i32); 4] = [
                (1, 0), (0, 1), (-1, 0), (0, -1)];
            let dpos = dirs.iter()
                .position(|&x| x == (actor.xdir, actor.ydir)).unwrap();
            let dpos_new = (dpos as i32 + 4 + dir) as usize % 4;
            anew.xdir = dirs[dpos_new].0;
            anew.ydir = dirs[dpos_new].1;
        }
        Move::Walk(mut steps) => {
            while steps != 0 {
                let mut yposnew = anew.ypos;
                let mut xposnew = anew.xpos;
                let mut ydirnew = anew.ydir;
                let mut xdirnew = anew.xdir;

                let y = anew.ypos;
                let x = anew.xpos;
                let f = dir2face((anew.xdir, anew.ydir));
                let l = cfg.cube_size as i32;

                if y == 0 && f == Face::Up {
                    // Side 1 up
                    xposnew = (l - 1) - (x - 2 * l);
                    yposnew = l;
                    (xdirnew, ydirnew) = face2dir(Face::Down);
                }
                else if x == l * 2 && y < l && f == Face::Left {
                    // Side 1 left
                    xposnew = l + y;
                    yposnew = l;
                    (xdirnew, ydirnew) = face2dir(Face::Down);
                }
                else if x == 3 * l - 1 && y < l && f == Face::Right {
                    // Side 1 right
                    xposnew = 4 * l - 1;
                    yposnew = 2 * l + (l - 1 - y);
                    (xdirnew, ydirnew) = face2dir(Face::Left);
                }
                else if x == 3 * l - 1 && y < 2 * l && f == Face::Right {
                    // Side 4 right
                    xposnew = 4 * l - 1 - (y - l);
                    yposnew = 2 * l;
                    (xdirnew, ydirnew) = face2dir(Face::Down);
                }
                else if x == 0 && f == Face::Left {
                    // Side 2 left
                    xposnew = 4 * l - 1 - (y - l);
                    yposnew = 3 * l - 1;
                    (xdirnew, ydirnew) = face2dir(Face::Up);
                }
                else if x < l && y == l && f == Face::Up {
                    // Side 2 up
                    xposnew = 3 * l - 1 - x;
                    yposnew = 0;
                    (xdirnew, ydirnew) = face2dir(Face::Down);
                }
                else if x < l && y == 2 * l - 1 && f == Face::Down {
                    // Side 2 down
                    xposnew = 3 * l - 1 - x;
                    yposnew = 3 * l - 1;
                    (xdirnew, ydirnew) = face2dir(Face::Up);
                }
                else if x < 2 * l && y == l && f == Face::Up {
                    // Side 3 up
                    xposnew = 2 * l;
                    yposnew = x - l;
                    (xdirnew, ydirnew) = face2dir(Face::Right);
                }
                else if x < 2 * l && y == 2 * l - 1 && f == Face::Down {
                    // Side 3 down
                    xposnew = 2 * l;
                    yposnew = 3 * l - 1 - (x - l);
                    (xdirnew, ydirnew) = face2dir(Face::Right);
                }
                else if x == 2 * l && y > 2 *l && f == Face::Left {
                    // Side 5 left
                    xposnew = 2 * l - 1 - (x - 2 * l);
                    yposnew = 2 * l - 1;
                    (xdirnew, ydirnew) = face2dir(Face::Up);
                }
                else if x < 3 * l && y == 3 * l - 1 && f == Face::Down {
                    // Side 5 down
                    xposnew = 3 * l - 1 - x;
                    yposnew = 2 * l - 1;
                    (xdirnew, ydirnew) = face2dir(Face::Up);
                }
                else if y == 2 * l && f == Face::Up {
                    // Side 6 up
                    xposnew = 3 * l - 1;
                    yposnew = 4 * l - 1 - x + l;
                    (xdirnew, ydirnew) = face2dir(Face::Left);
                }
                else if y == 3 * l - 1 && f == Face::Down {
                    // Side 6 down
                    xposnew = 0;
                    yposnew = 4 * l - 1 - x + l;
                    (xdirnew, ydirnew) = face2dir(Face::Right);
                }
                else if x == 4 * l - 1 && f == Face:: Right {
                    // Side 6 right
                    xposnew = 3 * l - 1;
                    yposnew = 4 * l - 1;
                    (xdirnew, ydirnew) = face2dir(Face::Left);
                } else {
                    xposnew += anew.xdir;
                    yposnew += anew.ydir;
                }

                // check if wall
                match cfg.m[yposnew as usize][xposnew as usize] {
                    Place::Outside => unreachable!(),
                    Place::Wall => break,
                    Place::Path => steps -= 1,
                };

                anew.xpos = xposnew;
                anew.ypos = yposnew;
                anew.xdir = xdirnew;
                anew.ydir = ydirnew;
            }
        }
    }

    anew
}

fn cube_walk_50(mv : Move, cfg : &Config, actor : &Actor) -> Actor {
    let mut anew = *actor;
    match mv {
        Move::Turn(dir) => {
            let dirs : [(i32, i32); 4] = [
                (1, 0), (0, 1), (-1, 0), (0, -1)];
            let dpos = dirs.iter()
                .position(|&x| x == (actor.xdir, actor.ydir)).unwrap();
            let dpos_new = (dpos as i32 + 4 + dir) as usize % 4;
            anew.xdir = dirs[dpos_new].0;
            anew.ydir = dirs[dpos_new].1;
        }
        Move::Walk(mut steps) => {
            while steps != 0 {
                let mut yposnew = anew.ypos;
                let mut xposnew = anew.xpos;
                let mut ydirnew = anew.ydir;
                let mut xdirnew = anew.xdir;

                let y = anew.ypos;
                let x = anew.xpos;
                let f = dir2face((anew.xdir, anew.ydir));
                let l = cfg.cube_size as i32;


                if y == 0 && x < 2 * l && f == Face::Up {
                    // Side 1 up
                    xposnew = 0;
                    yposnew = 3 * l + x - l;
                    (xdirnew, ydirnew) = face2dir(Face::Right);
                }
                else if x == l && y < l && f == Face::Left {
                    // Side 1 left
                    xposnew = 0;
                    yposnew = 2 * l + (l - 1 - y);
                    (xdirnew, ydirnew) = face2dir(Face::Right);
                }
                else if y == 0 && x >= 2 * l && f == Face::Up {
                    // Side 2 up
                    xposnew = x - 2 * l;
                    yposnew = 4 * l - 1;
                    (xdirnew, ydirnew) = face2dir(Face::Up);
                }
                else if x == 3 * l - 1 && f == Face::Right {
                    // Side 2 right
                    xposnew = 2 * l - 1;
                    yposnew = 2 * l + (l - 1 - y);
                    (xdirnew, ydirnew) = face2dir(Face::Left);
                }
                else if y == l - 1 && x >= 2 * l && f == Face::Down {
                    // Side 2 down
                    xposnew = 2 * l - 1;
                    yposnew = l + (x - 2 * l);
                    (xdirnew, ydirnew) = face2dir(Face::Left);
                }
                else if x == l && y >= l && y < 2 * l && f == Face::Left {
                    // Side 3 left
                    xposnew = y - l;
                    yposnew = 2 * l;
                    (xdirnew, ydirnew) = face2dir(Face::Down);
                }
                else if x == 2 * l - 1 && y >= l && y < 2 * l && f == Face::Right {
                    // Side 3 right
                    xposnew = 2 * l + (y - l);
                    yposnew = l - 1;
                    (xdirnew, ydirnew) = face2dir(Face::Up);
                }
                else if x < l && y == 2 * l && f == Face::Up {
                    // Side 4 up
                    xposnew = l;
                    yposnew = l + x;
                    (xdirnew, ydirnew) = face2dir(Face::Right);
                }
                else if x == 0 && y < 3 * l && f == Face::Left {
                    // Side 4 left
                    xposnew = l;
                    yposnew = 3 * l - 1 - y;
                    (xdirnew, ydirnew) = face2dir(Face::Right);
                }
                else if x == 2 * l - 1 && y >= 2 * l && y < 3 * l && f == Face::Right {
                    // Side 5 Right
                    xposnew = 3 * l - 1;
                    yposnew = 3 * l - 1 - y;
                    (xdirnew, ydirnew) = face2dir(Face::Left);
                }
                else if x >= l && y == 3 * l - 1 && f == Face::Down {
                    // Side 5 down
                    xposnew = l - 1;
                    yposnew = x - l + 3 * l;
                    (xdirnew, ydirnew) = face2dir(Face::Left);
                }
                else if x == 0 && y >= 3 * l && f == Face::Left {
                    // Side 6 left
                    xposnew = y - 3 * l + l;
                    yposnew = 0;
                    (xdirnew, ydirnew) = face2dir(Face::Down);
                }
                else if y == 4 * l - 1 && f == Face::Down {
                    // Side 6 down
                    xposnew = 2 * l + x;
                    yposnew = 0;
                    (xdirnew, ydirnew) = face2dir(Face::Down);
                }
                else if x == l - 1 && y >= 3 * l && f == Face::Right {
                    // Side 6 right
                    xposnew = y - 3 * l + l;
                    yposnew = 3 * l - 1;
                    (xdirnew, ydirnew) = face2dir(Face::Up);
                } else {
                    xposnew += anew.xdir;
                    yposnew += anew.ydir;
                }

                // check if wall
                match cfg.m[yposnew as usize][xposnew as usize] {
                    Place::Outside => {
                        dbg!(anew);
                        dbg!(xposnew, yposnew);
                        unreachable!()
                    },
                    Place::Wall => break,
                    Place::Path => steps -= 1,
                };

                anew.xpos = xposnew;
                anew.ypos = yposnew;
                anew.xdir = xdirnew;
                anew.ydir = ydirnew;
            }
        }
    }

    anew
}

fn calc_password(a : Actor) -> i32 {
    let col = a.xpos + 1;
    let row = a.ypos + 1;
    let dir = match (a.xdir, a.ydir) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!()
    };
    dbg!(col, row, dir);
    1000 * row + 4 * col + dir
}

fn part1(p : &Vec<Move>, cfg : &Config, actor : &Actor) {
    let mut a = *actor;
    for mv in p {
        a = walk(*mv, cfg, &a);
        //draw_map(cfg, &a);
        //println!();
    }

    let password = calc_password(a);

    println!("Password: {password}");
}

fn part2(p : &Vec<Move>, cfg : &Config, actor : &Actor) {
    let mut a = *actor;
    for mv in p {
        //dbg!(mv);
        //dbg!(&a, &mv);
        a = cube_walk(*mv, cfg, &a);
        //draw_map(cfg, &a);
        //println!();
    }

    let password = calc_password(a);

    println!("Password: {password}");
}

fn part2_50(p : &Vec<Move>, cfg : &Config, actor : &Actor) {
    // 26403 is too low
    // 114151 is too low

    let mut a = *actor;
    for mv in p {
        //dbg!(mv);
        //dbg!(&a, &mv);
        a = cube_walk_50(*mv, cfg, &a);
        //draw_map(cfg, &a);
        //println!();
    }

    let password = calc_password(a);

    println!("Password: {password}");
}

fn main() {
    let input = "input.txt";
    //let input = "input.txt";

    let lines = std::fs::read_to_string(input).unwrap();
    let mut v = lines.split("\n").collect::<Vec<&str>>();
    assert!(!v.is_empty());
    if v[v.len() - 1] == "" {
        v.pop();
    }
    let v = v;
    let ysize = v.len() - 2;

    let xsize = v.iter().take(ysize).map(|r| r.len()).max().unwrap();
    //dbg!(ysize, xsize);
    let cube_size = if input == "ex.txt" { 4 } else { 50 };

    let mut cfg = Config {
        m : Vec::new(),
        myrng : Vec::new(), // given x, what is ymin,ymax
        mxrng : Vec::new(), // given y, what is xmin,xmax
        xsize,
        ysize,
        cube_size,
    };
    for (yidx, row) in v.iter().take(ysize).enumerate() {
        cfg.m.push(Vec::new());
        let mut xmin = usize::MAX;
        let mut xmax = usize::MIN;
        for (xidx, c) in row.chars().enumerate() {
            let place = match c {
                ' ' => Place::Outside,
                '.' => Place::Path,
                '#' => Place::Wall,
                _ => unreachable!(),
            };
            cfg.m[yidx].push(place);
            if place != Place::Outside {
                xmin = cmp::min(xmin, xidx);
                xmax = cmp::max(xmax, xidx);
            }
        }
        let xlen = cfg.m[yidx].len();
        if xlen != xsize {
            cfg.m[yidx].extend(iter::repeat(Place::Outside).take(xsize - xlen));
        }
        cfg.mxrng.push((xmin, xmax));
    }
    // Calc yranges
    for xidx in 0..xsize {
        let mut ymin = usize::MAX;
        let mut ymax = usize::MIN;
        for yidx in 0..ysize {
            if cfg.m[yidx][xidx] != Place::Outside {
                ymin = cmp::min(ymin, yidx);
                ymax = cmp::max(ymax, yidx);
            }
        }
        cfg.myrng.push((ymin, ymax));
    }
    // Get path
    let mut p : Vec<Move> = Vec::new();
    let mut n = 0;
    for c in v.last().unwrap().chars() {
        match c {
            'L' => {
                p.push(Move::Walk(n));
                p.push(Move::Turn(-1));
                n = 0;
            },
            'R' => {
                p.push(Move::Walk(n));
                p.push(Move::Turn(1));
                n = 0;
            },
            c => {
                n *= 10;
                n += c.to_digit(10).unwrap() as i32;
            }
        }
    }
    if n != 0 {
        p.push(Move::Walk(n));
    }

    let actor = Actor {
        xpos : cfg.mxrng[0].0 as i32,
        ypos : 0,
        xdir : 1,
        ydir : 0,
    };
    //draw_map(&cfg, &actor);

    //dbg!(&p);

    //dbg!(cfg.m);
    //dbg!(cfg.mxrng);
    //dbg!(cfg.myrng);
    //dbg!(v);

    //part1(&p, &cfg, &actor);
    if cube_size == 4 {
        part2(&p, &cfg, &actor);
    } else if cube_size == 50 {
        part2_50(&p, &cfg, &actor);
    } else {
        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit_plus_lit() {
    }
}
