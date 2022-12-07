#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug)]
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

struct Tree {
    tree : HashMap<String, usize>,
    dirs : Vec<DirData>,
    files : Vec<FileData>,
}

fn get_path(path_as_vec : &Vec<String>) -> String {
    path_as_vec.join("/")
}

fn get_tree() -> Tree {
    let lines = std::fs::read_to_string("input.txt").unwrap();
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
                                println!("cd: create new {path}");
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

                                // update parent dirs
                                dirs[parent_dir].dirs.push(dir_index);
                            }
                        }
                    },
                    "ls" => {
                         println!("ls");
                    },
                    _ => (),
                }
            },
            "dir" => {
                let mut new_dir_path = cur_path.clone();
                new_dir_path.push(words[1].to_string());
                let path = get_path(&new_dir_path);
                if !tree.contains_key(&path) {
                    // create new entry
                    println!("dir: create new {path}");
                    let parent_dir = if cur_dir.is_empty() { 0usize } else {cur_dir[cur_dir.len()-1]};
                    let dir_node = DirData {
                        name : words[1].to_string(),
                        parent : parent_dir,
                        size : 0,
                        files : Vec::new(),
                        dirs : Vec::new()
                    };
                    dirs.push(dir_node);
                    let dir_index = dirs.len() - 1;
                    tree.insert(path, dir_index);

                    // update parent dirs
                    dirs[parent_dir].dirs.push(dir_index);
                }
            },
            fs => {
                let fs = fs.parse::<u64>().unwrap();
                let parent_dir = if cur_dir.is_empty() { 0usize } else {cur_dir[cur_dir.len()-1]};
                if files.iter().any(|fd| fd.name == words[1] && fd.parent == parent_dir) {
                    println!("File {} already exists", words[1]);
                    continue;
                }
                let file_node = FileData {
                    parent : parent_dir,
                    name : words[1].to_string(),
                    size : fs,
                };
                files.push(file_node);
                let file_index = files.len() - 1;

                dirs[parent_dir].files.push(file_index);

                println!("Create file {} size={fs} in dir {}", words[1], dirs[parent_dir].name);
            }
        }
    }

    //println!("{v:?}");
    //println!("Tree: {tree:?}");
    //println!("Dirs: {dirs:?}");
    //println!("Files: {files:?}");

    Tree {
        tree,
        dirs,
        files
    }
}

fn calc_size(tree : &mut Tree, dir_index : usize) -> u64 {
    let file_sizes : u64 = tree.dirs[dir_index].files.iter().map(
        |fd| tree.files[*fd].size).sum();
    let mut dir_sizes = 0u64;
    for i in 0..tree.dirs[dir_index].dirs.len() {
        let sub_dir_index = tree.dirs[dir_index].dirs[i];
        dir_sizes += calc_size(tree, sub_dir_index);
    }

    tree.dirs[dir_index].size = file_sizes + dir_sizes;
    file_sizes + dir_sizes
}

fn part1() {
    let mut tree = get_tree();
    calc_size(&mut tree, 0);

    println!("Tree: {:?}", tree.tree);
    println!("Dirs: {:?}", tree.dirs);
    println!("Files: {:?}", tree.files);

    let total_big : u64 = tree.dirs.iter().map(|dd| dd.size).filter(|size| *size <= 100_000).sum();

    println!("Total size, n <= 100000 = {total_big}");
}

fn part2() {
    let mut tree = get_tree();
    calc_size(&mut tree, 0);

    println!("Tree: {:?}", tree.tree);
    println!("Dirs: {:?}", tree.dirs);
    println!("Files: {:?}", tree.files);

    let mut sizes : Vec<u64> = tree.dirs.iter().map(|dd| dd.size).collect::<Vec<u64>>();
    let total = tree.dirs[0].size;
    let unused = 70_000_000 - total;
    let extra_needed = 30_000_000 - unused;
    sizes.sort();

    println!("Total: {total}");
    println!("Unused: {unused}");
    println!("Extra_Needed: {extra_needed}");
    println!("Sizes: {sizes:?}");

    let the_size = sizes.iter().find(|size| **size >= extra_needed).unwrap();
    println!("The size: {the_size}");
}

fn main() {
    //part1();
    part2();
}
