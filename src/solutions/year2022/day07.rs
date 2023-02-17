use std::collections::{HashMap, VecDeque};

struct Node {
    name: String,
    files: Vec<(String, usize)>,
    pub dirs: HashMap<String, Node>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: vec![],
            dirs: HashMap::new(),
        }
    }

    pub fn add_dir(&mut self, name: &str) {
        self.dirs.insert(name.to_string(), Node::new(name));
    }

    pub fn add_file(&mut self, name: &str, size: usize) {
        self.files.push((name.to_string(), size));
    }

    pub fn cd(&mut self, name: &str) -> Option<&mut Node> {
        self.dirs.get_mut(name)
    }

    pub fn get_path(&mut self, path: &Vec<&str>) -> Option<&mut Node> {
        let mut curr_node = self;
        for segment in path {
            if let Some(node) = curr_node.cd(segment) {
                curr_node = node;
            } else {
                return None;
            }
        }
        Some(curr_node)
    }

    pub fn size(&self) -> usize {
        let local_size: usize = self.files.iter().map(|(_, s)| s).sum();
        let children_size: usize = self.dirs.values().map(|v| v.size()).sum();
        local_size + children_size
    }
}

pub fn part1(input: &str) -> usize {
    let mut root = Node::new("/");
    let mut curr_node = &mut root;
    let mut ls_mode = false;

    let mut current_path: Vec<&str> = vec![];

    for line in input.lines() {
        if ls_mode && line.starts_with('$') {
            ls_mode = false;
        }

        if ls_mode {
            if let Some(dirname) = line.strip_prefix("dir ") {
                curr_node.add_dir(dirname);
                continue;
            }
            let (filesize, filename) = line.split_once(' ').unwrap();
            curr_node.add_file(filename, filesize.parse::<usize>().unwrap());
        }

        if let Some(new_dir) = line.strip_prefix("$ cd ") {
            if new_dir == "/" {
                curr_node = &mut root;
            } else if new_dir == ".." {
                let _ = current_path.pop();
                curr_node = root.get_path(&current_path).unwrap();
            } else {
                curr_node = curr_node.cd(new_dir).unwrap();
                current_path.push(new_dir);
            }
            continue;
        }

        if line == "$ ls" {
            ls_mode = true;
            continue;
        }
    }

    let mut q = VecDeque::new();
    q.push_back(&root);
    let mut res = 0;

    while let Some(node) = q.pop_front() {
        let node_size = node.size();
        if node_size <= 100000 {
            res += node_size;
        }

        for subdir in node.dirs.values() {
            q.push_front(subdir);
        }
    }
    res
}

// year 22 day06 part 2    time:   [18.139 µs 18.239 µs 18.344 µs]
pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day07_test() {
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
        assert_eq!(part1(input), 95437);
        // assert_eq!(part2(input), 26);
    }

    #[test]
    fn day07() {
        let input = get_input(2022, 7).unwrap();
        assert_eq!(part1(&input), 1611443);
        assert_eq!(part2(&input), 2263);
    }
}
