use std::{collections::HashMap, usize};

pub fn solve (input: &str) -> String 
{
    let mut path: Vec<String> = Vec::new();
    let mut root = Directory { /*name: String::from("root"),*/ directories: HashMap::new(), files: HashMap::new() };

    let mut current_dir = &mut root;

    for line in input.lines() {
        if line == "$ cd /" {
            continue
        } else if line.starts_with("$ cd") {
            let dir_name = &line[5..];
            if dir_name == ".." {
                path.pop();
                current_dir = &mut root;
                for dir in &mut path {
                    println!("cd .. vers {}", dir);
                    current_dir = current_dir.directories.get_mut(dir).expect("If there's no subfolder with that name in the current directory, we missed an ls command");
                }
            } else {
                path.push(String::from(dir_name));
                println!("cd vers {}", dir_name);
                current_dir = current_dir.directories.get_mut(dir_name).expect("If there's no subfolder with that name in the current directory, we missed an ls command");
            }
        } else if line.starts_with("$ ls") {
            continue
        } else if line.starts_with("dir"){
            let dir_name = &line[4..];

            current_dir.directories.insert(
                String::from(dir_name),
                Directory { /*name: String::from(dir_name),*/ directories: HashMap::new(), files: HashMap::new() }
            );
        } else if line.trim().len() > 0 {
            // We're on a file definition, try to parse its size and get its name
            let (size, file_name) = line.split_once(' ').unwrap();

            current_dir.files.insert(
                String::from(file_name),
                File { /*name: String::from(file_name),*/ size: size.parse().unwrap()}
            );
        }
    }

    let part_1 = recursive_part1(&root);
    let mut part_2: usize = 0;

    let root_size = root.get_size();
    let missing_space = root_size + 30000000 - 70000000;
    if missing_space > 0 {
        part_2 = recursive_part2(&root, missing_space);
    }

    format!("Part 1: {}, Part 2: {}", part_1, part_2)
}

fn recursive_part1(dir: &Directory) -> usize {
    let mut size = 0;
    for sub_dir in dir.directories.values() {
        size = size + recursive_part1(sub_dir);
    }

    let dir_size = dir.get_size();
    if dir_size <= 100000 {
        size = size + dir_size;
    }

    size
}

fn recursive_part2(dir: &Directory, threshold: usize) -> usize {
    let mut size = dir.get_size();
    
    for sub_dir in dir.directories.values() {
        let smallest = recursive_part2(sub_dir, threshold);

        if smallest >= threshold && smallest < size {
            size = smallest;
        }
    }

    size
}

#[derive(Debug)]
struct Directory {
    //name: String,
    directories: HashMap<String, Directory>,
    files: HashMap<String, File>
}

impl Directory {
    fn get_size(&self) -> usize {
        let dir_sum: usize = self.directories.values().map(|dir| dir.get_size()).sum();
        let file_sum: usize = self.files.values().map(|file| file.size).sum();
        dir_sum + file_sum
    }
}

#[derive(Debug)]
struct File {
    //name: String,
    size: usize
}

#[cfg(test)]
mod tests {
    use super::{*, super::input};

    #[test]
    fn day7_smallinput() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(solve(input), "Part 1: 95437, Part 2: 24933642");
    }

    #[test]
    fn day7_biginput() {
        let input = input::get_input();

        assert_eq!(solve(input.as_str()), "Part 1: 1915606, Part 2: 5025657");
    }
}