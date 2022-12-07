use advent_2022::{read_aoc_lines, InputError, InputIterator, OptionUtils};
use anyhow::Result;
use std::{collections::HashMap, mem::take};

struct Dir {
    size: i64,
    contents: HashMap<String, Entry>,
}

impl Dir {
    fn new() -> Dir {
        Dir {
            size: -1,
            contents: Default::default(),
        }
    }
}

enum Entry {
    File { size: i64 },
    Dir(usize),
}


fn update_dir_sizes(e: &Entry, dir_store: &mut Vec<Dir>) -> i64 {
    match e {
        Entry::File { size } => *size,
        Entry::Dir(d) => {
            let contents = take(&mut dir_store[*d].contents);
            let size = contents.values().map(|e| update_dir_sizes(e, dir_store)).sum();
            dir_store[*d].size = size;
            dir_store[*d].contents = contents;
            size
        }
    }
}

fn get_filesystem<I: InputIterator>(mut input: I) -> Result<Vec<Dir>> {
    let mut dir_store = vec![Dir::new()]; // populate with root dir to start
    let mut cur_path = vec![];

    while let Some(l) = input.next() {
        let line = l.as_ref();
        if line == "$ cd /" {
            cur_path = vec![0usize];
        }
        if line == "$ cd .." {
            cur_path.pop().ok_or_err()?;
        } else if let Some(dir_name) = line.strip_prefix("$ cd ") {
            let dir_store_len = dir_store.len();
            let cur_dir = &mut dir_store[*cur_path.last().ok_or_err()?];
            if let Some(e) = cur_dir.contents.get(dir_name) {
                match e {
                    Entry::File { size: _ } => {
                        return Err(InputError::new("Tried to cd into file!").into())
                    }
                    Entry::Dir(d) => cur_path.push(*d),
                }
            } else {
                cur_dir
                    .contents
                    .insert(dir_name.to_string(), Entry::Dir(dir_store_len));
                dir_store.push(Dir::new());
            }
        } else if line == "$ ls" {
            // nothing to do in this case
        } else {
            // result of ls
            let mut split = line.split(' ');
            let (first, name) = (split.next().ok_or_err()?, split.next().ok_or_err()?);
            let dir_store_len = dir_store.len();
            let cur_dir = &mut dir_store[*cur_path.last().ok_or_err()?];
            if first == "dir" {
                cur_dir
                    .contents
                    .insert(name.to_string(), Entry::Dir(dir_store_len));
                dir_store.push(Dir::new());
            } else {
                cur_dir.contents.insert(
                    name.to_string(),
                    Entry::File {
                        size: first.parse()?,
                    },
                );
            }
        }
    }
    update_dir_sizes(&Entry::Dir(0), &mut dir_store);
    Ok(dir_store)
}

fn part1<I: InputIterator>(input: I) -> Result<i64> {
    let dir_store = get_filesystem(input)?;
    Ok(dir_store
        .iter()
        .map(|d| d.size)
        .filter(|s| *s <= 100000)
        .sum())
}

fn part2<I: InputIterator>(input: I) -> Result<i64> {
    let dir_store = get_filesystem(input)?;


    let space_needed = 30000000 - (70000000 - dir_store.first().unwrap().size);
    Ok(dir_store.iter().map(|d| d.size).filter(|s| *s >= space_needed).min().ok_or_err()?)
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    #[test]
    fn dayn_test() {
        let input = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        assert_eq!(part1(input.iter()).unwrap(), 95437);
        assert_eq!(part2(input.iter()).unwrap(), 24933642);
    }
}

fn main() {
    print!("{:?}\n", part1(read_aoc_lines!()));
    print!("{:?}\n", part2(read_aoc_lines!()));
}
