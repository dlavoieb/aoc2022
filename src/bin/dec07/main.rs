use std::cmp::min;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug)]
enum LineType {
    ChangeDir(String),
    ListDir,
    DirEntry(String),
    File(i32, String)
}

#[derive(Debug)]
struct Directory {
    files: Vec<File>,
    subdirs: Vec<String>,
}

impl Directory {
    fn new() -> Self {
        Directory{files: Vec::new(), subdirs: Vec::new()}
    }
}

fn main() {
    let mut lines = aoc2022::read_file("src/bin/dec07/adventofcode.com_2022_day_7_input.txt");

    let mut root: HashMap<PathBuf, Directory> = HashMap::new();
    let mut cdw = PathBuf::from("/");
    root.insert(cdw.clone(), Directory::new());

    for line in lines {
        let data = get_line_data(line);
        match data {
            LineType::ChangeDir(newDir) => {
                if newDir == ".." {
                    cdw.pop();
                } else if newDir == "/" {
                    cdw = PathBuf::from("/");
                } else {
                    cdw.push(newDir);
                }
            }
            LineType::ListDir => {}
            LineType::DirEntry(name) => {
                let mut new_dir = cdw.clone();
                new_dir.push(&name);
                root.insert(new_dir, Directory::new());
                root.get_mut(&cdw).unwrap().subdirs.push(name);
            }
            LineType::File(size, name) => {root.get_mut(&cdw).unwrap().files.push(File{name, size})}
        }
    }

    let root_path = PathBuf::from("/");
    let root_size = dir_size(&root[&root_path], &root_path, &root);
    let free_space = 70000000 - root_size;
    let needed_space = 30000000 - free_space;

    let mut lowest_possible = root_size;


    let mut total_size = 0;
    for (path, dir) in &root {
        let size = dir_size(dir, path, &root);
        if size <= 100000 {
            total_size += size;
        }
        if size >= needed_space {
            lowest_possible = min(lowest_possible, size);
        }
    }
    println!("{}", total_size);
    println!("{}", lowest_possible);

}

fn get_line_data(line: String) -> LineType {
    if line.starts_with("$ cd") {
        let dir = &line[5..line.len()];
        LineType::ChangeDir(String::from(dir))
    } else if line.starts_with("$ ls")
    {
        LineType::ListDir
    } else if line.starts_with("dir ") {
        let dir = &line[4..line.len()];
        LineType::DirEntry(String::from(dir))
    } else {
        let parts:Vec<&str> = line.split(" ").collect();
        LineType::File(parts[0].parse::<i32>().unwrap_or(0), String::from(parts[1]))
    }
}

fn dir_size(dir: &Directory, path: &PathBuf, root: &HashMap<PathBuf, Directory>) -> i32 {
    let mut total = 0;
    for file in &dir.files {
        total += file.size;
    }

    for subdir in &dir.subdirs {
        let mut dir = path.clone();
        dir.push(subdir);
        total += dir_size(&root[&dir], &dir, root);
    }

    total
}