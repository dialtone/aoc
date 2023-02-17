use std::collections::VecDeque;

pub struct Node {
    name: String,
    files: Vec<usize>,
    pub dirs: Vec<Node>,
    tsize: Option<usize>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: vec![],
            dirs: vec![],
            tsize: None,
        }
    }

    pub fn add_dir(&mut self, name: &str) {
        self.dirs.push(Node::new(name));
    }

    pub fn add_file(&mut self, size: usize) {
        self.files.push(size);
    }

    pub fn cd(&mut self, name: &str) -> Option<&mut Node> {
        self.dirs.iter_mut().find(|n| n.name == name)
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

    pub fn size(&mut self) -> usize {
        if let Some(tsize) = self.tsize {
            return tsize;
        }
        let local_size: usize = self.files.iter().sum();
        let children_size: usize = self.dirs.iter_mut().map(|v| v.size()).sum();
        let tsize = local_size + children_size;
        self.tsize = Some(tsize);
        tsize
    }
}

// year 22 day07 parse     time:   [62.322 µs 62.417 µs 62.516 µs] first version with hashmap
// year 22 day07 parse     time:   [45.368 µs 45.547 µs 45.749 µs] current version with vec
pub fn parse(input: &str) -> Node {
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
            let (filesize, _) = line.split_once(' ').unwrap();
            curr_node.add_file(filesize.parse::<usize>().unwrap());
            continue;
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
    root
}

// year 22 day07 part 1    time:   [66.228 µs 66.312 µs 66.404 µs] with hashmap
// year 22 day07 part 1    time:   [46.898 µs 47.033 µs 47.194 µs] with vec
pub fn part1(input: &str) -> usize {
    let mut root = parse(input);
    let mut q = VecDeque::new();
    q.push_back(&mut root);
    let mut res = 0;

    while let Some(node) = q.pop_front() {
        let node_size = node.size();
        if node_size <= 100000 {
            res += node_size;
        }

        for subdir in node.dirs.iter_mut() {
            q.push_front(subdir);
        }
    }
    res
}

// year 22 day07 part 2    time:   [67.164 µs 67.619 µs 68.145 µs] with hashmap
// year 22 day07 part 2    time:   [47.395 µs 47.857 µs 48.310 µs] with vec
pub fn part2(input: &str) -> usize {
    let mut root = parse(input);

    let total_size = root.size();
    let disk_capacity = 70_000_000;
    let space_needed = 30_000_000usize;

    let minimum_dir_size = space_needed - (disk_capacity - total_size);
    let mut found_dir = disk_capacity;

    let mut q = VecDeque::new();
    q.push_back(&mut root);

    while let Some(node) = q.pop_front() {
        let node_size = node.size();
        if node_size >= minimum_dir_size && found_dir > node_size {
            found_dir = node_size
        }

        for subdir in node.dirs.iter_mut() {
            q.push_front(subdir);
        }
    }
    found_dir
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
        assert_eq!(part2(input), 24933642);
    }

    #[test]
    fn day07() {
        let input = get_input(2022, 7).unwrap();
        assert_eq!(part1(&input), 1611443);
        assert_eq!(part2(&input), 2086088);
    }
}
