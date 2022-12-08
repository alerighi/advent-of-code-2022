use crate::problem::AoCProblem;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

const MAX_SIZE: usize = 100_000;
const TOTAL_SIZE: usize = 70_000_000;
const REQUIRED_SPACE: usize = 30_000_000;

#[derive(Debug)]
struct Directory {
    entries: HashMap<String, Entry>,
    parent: Weak<RefCell<Directory>>,
}

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
enum Entry {
    File(File),
    Directory(Rc<RefCell<Directory>>),
}

#[derive(Debug)]
pub struct AoCDay7 {
    root: Rc<RefCell<Directory>>,
    cwd: Option<Rc<RefCell<Directory>>>,
}

fn part_1_solve(max_size: usize, dir: &Rc<RefCell<Directory>>) -> (usize, usize) {
    let mut size = 0;
    let mut max = 0;
    for (_, entry) in &dir.borrow().entries {
        match entry {
            Entry::File(f) => {
                size += f.size;
            }
            Entry::Directory(d) => {
                let (d_size, d_max) = part_1_solve(max_size, d);
                size += d_size;
                max += d_max;
            }
        }
    }
    if size < max_size {
        max += size;
    }
    (size, max)
}

fn part_2_solve(min_size: usize, dir: &Rc<RefCell<Directory>>) -> (usize, usize) {
    let mut size = 0;
    let mut min = usize::MAX;
    for (_, entry) in &dir.borrow().entries {
        match entry {
            Entry::File(f) => {
                size += f.size;
            }
            Entry::Directory(d) => {
                let (d_size, d_min) = part_2_solve(min_size, d);
                size += d_size;
                if d_size > min_size {
                    min = min.min(d_size);
                }
                min = min.min(d_min);
            }
        }
    }
    (size, min)
}

impl Default for AoCDay7 {
    fn default() -> Self {
        let root = Rc::new_cyclic(|this| {
            RefCell::new(Directory {
                entries: HashMap::new(),
                parent: this.clone(),
            })
        });
        AoCDay7 {
            root: root.clone(),
            cwd: Some(root),
        }
    }
}

impl AoCDay7 {
    fn exists(&self, name: &str) -> bool {
        self.cwd
            .as_ref()
            .unwrap()
            .borrow_mut()
            .entries
            .contains_key(name)
    }

    fn mkdir(&self, name: &str) {
        self.cwd.as_ref().unwrap().borrow_mut().entries.insert(
            name.into(),
            Entry::Directory(Rc::new(RefCell::new(Directory {
                entries: HashMap::new(),
                parent: Rc::downgrade(&self.cwd.as_ref().unwrap()),
            }))),
        );
    }

    fn cd(&mut self, name: &str) {
        let cwd = self.cwd.take();
        match cwd.unwrap().borrow().entries.get(name) {
            None => unreachable!(),
            Some(Entry::File(_)) => panic!("try to CD to a file"),
            Some(Entry::Directory(dir)) => {
                self.cwd = Some(dir.clone());
            }
        }
    }

    fn touch(&mut self, name: &str, size: usize) {
        self.cwd
            .as_ref()
            .unwrap()
            .borrow_mut()
            .entries
            .insert(name.into(), Entry::File(File { size }));
    }
}

impl AoCProblem for AoCDay7 {
    fn parse_line(&mut self, line: String) {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    self.cwd = self.cwd.take().unwrap().borrow().parent.upgrade();
                } else {
                    let dir_name = parts[2];

                    if !self.exists(dir_name) {
                        self.mkdir(dir_name);
                    }

                    self.cd(dir_name);
                }
            }
        } else {
            if parts[0] != "dir" {
                let size: usize = parts[0].parse().unwrap();
                let file_name = parts[1];
                self.touch(file_name, size);
            }
        }
    }

    fn solve_part1(&self) -> String {
        part_1_solve(MAX_SIZE, &self.root).1.to_string()
    }

    fn solve_part2(&self) -> String {
        let occupied_size = part_1_solve(100_000, &self.root).0;
        let free_space = TOTAL_SIZE - occupied_size;
        let to_free = REQUIRED_SPACE - free_space;
        part_2_solve(to_free, &self.root).1.to_string()
    }
}
