#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashMap;

struct FileData {
    parent : usize,
    name : String,
    size : u64,
}

#[derive(Debug)]
struct DirData {
    parent : usize,
    name : String,
    size : u64,
    files : Vec<usize>,
    dirs : Vec<usize>
}

fn get_path(path_as_vec : &Vec<String>) -> String {
    path_as_vec.join("/")
}

fn part1() {
    let lines = std::fs::read_to_string("ex.txt").unwrap();
    let v = lines.split("\n").collect::<Vec<&str>>();

    let mut files : Vec<FileData> = Vec::new();
    let mut dirs : Vec<DirData> = Vec::new();
    let mut tree : HashMap<String, usize> = HashMap::new();
    let mut cur_path : Vec<String> = Vec::new();
    let mut cur_dir : Vec<usize> = Vec::new();

    let root_node = DirData {
        name : "/".to_string(),
        parent : 0,
        size : 0,
        files : Vec::new(),
        dirs : Vec::new()
    };
    dirs.push(root_node);
    cur_dir.push(0);
    cur_path.push("/".to_string());
    tree.insert("/".to_string(), 0);

    let parent = 0usize;
    for line in v.iter() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        if words.len() == 0 {
            continue;
        }

        match words[0] {
            "$" => {
                match words[1] {
                    "cd" => {
                        if words[2] == ".." {
                            cur_path.pop();
                            cur_dir.pop();
                        } else if words[2] == "/" {
                            cur_path.clear();
                            cur_path.push("/".to_string());
                            cur_dir.clear();
                            cur_dir.push(0usize);
                        } else {
                            cur_path.push(words[2].to_string());
                            let path = get_path(&cur_path);
                            if let Some(&dir_index) = tree.get(&path) {
                                // entry exist, update cur_dir
                                cur_dir.push(dir_index);
                            } else {
                                // create new entry
                                let parent_dir = if cur_dir.is_empty() { 0usize } else {cur_dir[cur_dir.len()-1]};
                                let dir_node = DirData {
                                    name : words[2].to_string(),
                                    parent : parent_dir,
                                    size : 0,
                                    files : Vec::new(),
                                    dirs : Vec::new()
                                };
                                dirs.push(dir_node);
                                let dir_index = dirs.len() - 1;
                                cur_dir.push(dir_index);
                                tree.insert(path, dir_index);
                            }
                        }
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }

    //println!("{v:?}");
    println!("Tree: {tree:?}");
    println!("Dirs: {dirs:?}");
}

fn main() {
    part1();
    //part2();
}
