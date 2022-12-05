use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use anyhow::Result;
use std::error::Error;
use thiserror::Error;

pub fn read_lines<P: AsRef<Path>>(filename: P) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap())
}

pub fn read_aoc_lines_impl(src_filename: &'static str) -> impl Iterator<Item = String> {
    let mut p = PathBuf::from("/home/nir/Downloads");
    p.push(Path::new(src_filename).file_stem().unwrap());
    p.set_extension("txt");
    read_lines(p)
}

#[macro_export]
macro_rules! read_aoc_lines {
    () => {
        advent_2022::read_aoc_lines_impl(file!())
    };
}


pub struct ChunkedInput<I: InputIterator, F> {
    it: I,
    f: F,
    chunk: Vec<I::Item>,
    done: bool,
}

impl<I: InputIterator, R, F: FnMut(&mut Vec<I::Item>) -> R> Iterator
    for ChunkedInput<I, F>
{
    type Item = R;

    fn next(&mut self) -> Option<R> {
        if self.done {
            return None;
        }
        loop {
            let o = self.it.next();
            match o {
                None => {
                    self.done = true;
                    let r = Some((self.f)(&mut self.chunk));
                    self.chunk.clear();
                    return r;
                }
                Some(s) => {
                    if s.as_ref() == "" {
                        let r = Some((self.f)(&mut self.chunk));
                        self.chunk.clear();
                        return r;
                    }
                    self.chunk.push(s);
                }
            }
        }
    }
}

pub struct FixedChunks<I: Iterator, F> {
    it: I,
    f: F,
    chunk: Vec<I::Item>,
    chunk_size: usize,
    done: bool,
}

impl<I: Iterator, R, F: FnMut(&mut Vec<I::Item>) -> R> Iterator for FixedChunks<I, F> {
    type Item = R;

    fn next(&mut self) -> Option<R> {
        if self.done {
            return None;
        }
        for _ in 0..self.chunk_size {
            let o = self.it.next();
            match o {
                None => {
                    self.done = true;
                    break;
                }
                Some(e) => self.chunk.push(e),
            };
        }
        if self.chunk.is_empty() {
            return None;
        }
        let r = Some((self.f)(&mut self.chunk));
        self.chunk.clear();
        r
    }
}

pub trait InputIterator: Iterator<Item=Self::S> {
    type S: AsRef<str>;

    fn blank_chunks<R, F: FnMut(&mut Vec<Self::Item>) -> R>(self, f: F) -> ChunkedInput<Self, F>
    where
        Self: Sized;
}

impl<I: Iterator<Item = S>, S: AsRef<str>> InputIterator for I {
    type S = S;
    fn blank_chunks<R, F: FnMut(&mut Vec<S>) -> R>(self, f: F) -> ChunkedInput<I, F> {
        ChunkedInput {
            it: self,
            f,
            chunk: Default::default(),
            done: false,
        }
    }
}

#[derive(Debug)]
pub struct OnlyError;
pub trait IteratorUtils : Iterator {

    fn fixed_chunks<R, F: FnMut(&mut Vec<Self::Item>) -> R>(
        self,
        size: usize,
        f: F,
    ) -> FixedChunks<Self, F>
    where
        Self: Sized;

    fn only(self) -> Result<Self::Item, OnlyError>;
}

impl <I: Iterator> IteratorUtils for I {
    fn fixed_chunks<R, F: FnMut(&mut Vec<Self::Item>) -> R>(
        self,
        chunk_size: usize,
        f: F,
    ) -> FixedChunks<Self, F> {
        FixedChunks {
            it: self,
            f,
            chunk: Default::default(),
            chunk_size,
            done: false,
        }
    }

    fn only(mut self) -> Result<Self::Item, OnlyError> {
        let o = self.next();
        match o {
            None => Err(OnlyError{}),
            Some(e) => {
                match self.next() {
                    None => Ok(e),
                    Some(_) => Err(OnlyError{}),
                }
            }
        }
    }
}


pub fn check<R: Error, F: FnOnce() -> R>(b: bool, f: F) -> Result<(), R> {
    if b {
        Ok(())
    }
    else {
        Err(f())
    }
}

#[derive(Error, Debug)]
#[error("{msg}")]
pub struct InputError {
    msg: String,
}

impl InputError {
    pub fn new<S: AsRef<str>>(msg: S) -> InputError {
        InputError { msg: format!("Input error: {}", msg.as_ref()) }
    }
}

impl Default for InputError {
    fn default() -> InputError {
        InputError {msg: "Input error".to_string() }
    }
}

#[derive(Error, Debug)]
#[error("Option Empty")]
pub struct OptionEmptyError;

pub trait OptionUtils {
    type Target;
    fn ok_or_err(self) -> Result<Self::Target, OptionEmptyError>;
}

impl <T> OptionUtils for Option<T> {
    type Target = T;
    fn ok_or_err(self) -> Result<Self::Target, OptionEmptyError> {
        match self {
            Some(e) => Ok(e),
            None => Err(OptionEmptyError{})
        }
    }
}