use crate::support::*;
use itertools::*;
use std::rc::*;
use std::cell::*;
use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");

struct DirStruct
{
    parent: Option<Dir>,
    entries: HashMap<String, Entry>,
}

type Dir = Rc<RefCell<DirStruct>>;

#[derive(Clone)]
enum Entry
{
    File(usize),
    Dir(Dir),
}

fn parse(input: &str) -> Dir
{
    let root = Rc::new(RefCell::new(DirStruct { 
        parent: None,
        entries: HashMap::new()
    }));
    let mut cur = root.clone();

    for line in input_to_lines(input)
    {
        if line.starts_with("$ cd ")
        {
            let dir = line.split_at(5).1.to_string();

            if dir == ".."
            {
                let parent = cur.borrow().parent.to_owned().unwrap();
                cur = parent.clone();
            }
            else if dir == "/"
            {
                cur = root.clone()
            }
            else
            {
                let subfolder = cur.borrow().entries.get(&dir).cloned().unwrap();
                match subfolder
                {
                    Entry::File(_) => assert!(false),
                    Entry::Dir(d) => cur = d,
                }
            }
        }
        else if line == "$ ls"
        {
            // Do nothing - assume all other lines
            // are listing the current dir
        }
        else
        {
            let parts = line.split(' ').collect_vec();
            assert!(parts.len() == 2);

            if parts[0] == "dir"
            {
                let ndir = Rc::new(RefCell::new(DirStruct
                    {
                        parent: Some(cur.clone()),
                        entries: HashMap::new()
                    }));

                let old = cur.borrow_mut().entries.insert(
                    parts[1].to_string(),
                    Entry::Dir(ndir));
                assert!(old.is_none());
            }
            else
            {
                let old = cur.borrow_mut().entries.insert(
                    parts[1].to_string(),
                    Entry::File(parts[0].parse().unwrap()));
                assert!(old.is_none());
            }
        }
    }

    root
}

fn all_dirs(root: Dir) -> Vec<Dir>
{
    let mut result = Vec::new();

    fn append(result: &mut Vec<Dir>, dir: &Dir)
    {
        result.push(dir.clone());
        for (_name, entry) in &dir.borrow().entries
        {
            match entry
            {
                Entry::File(_) => (),
                Entry::Dir(dir) => append(result, &dir),
            }
        }
    }

    append(&mut result, &root);

    result
}

fn dir_size(dir: Dir) -> usize
{
    dir.borrow().entries.iter()
        .map(|(_name, entry)|
        {
            match entry
            {
                Entry::File(size) => size.clone(),
                Entry::Dir(dir) => dir_size(dir.clone()),
            }            
        })
        .sum()
}

fn part_1(input: &str) -> usize
{
    all_dirs(parse(input))
        .iter()
        .map(|dir| dir_size(dir.clone()))
        .filter(|&size| size <= 100000)
        .sum()
}

fn part_2(input: &str) -> usize
{
    let root = parse(input);
    let cur_root_size = dir_size(root.clone());
    let needed_space = 30000000 - (70000000 - cur_root_size);

    all_dirs(root)
        .iter()
        .map(|d| dir_size(d.clone()))
        .sorted()
        .filter(|&size| size >= needed_space)
        .next()
        .unwrap()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(7)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 95437,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 1444896,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 24933642,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 404395,
        })
}
