#![no_std]
#![warn(clippy::pedantic)]
use core::fmt::Display;

extern crate alloc;
use alloc::{format, string::String, vec, vec::Vec};

#[derive(Debug, Clone)]
pub struct Columns<'a> {
    inner: Vec<Vec<&'a str>>,
    tabsize: usize,
    largest: usize,
}

impl<'a> From<Vec<&'a str>> for Columns<'a> {
    fn from(f: Vec<&'a str>) -> Self {
        Self {
            inner: vec![f.clone()],
            tabsize: {
                let mut size = 0;
                for item in &f {
                    if size < item.len() {
                        size = item.len();
                    }
                }
                size + 3
            },
            largest: {
                let mut largest_line_count = 0;
                if f.len() > largest_line_count {
                    largest_line_count = f.len();
                }
                largest_line_count
            },
        }
    }
}

impl<'a> From<Vec<Vec<&'a str>>> for Columns<'a> {
    fn from(f: Vec<Vec<&'a str>>) -> Self {
        Self {
            inner: f.clone(),
            tabsize: {
                let mut size = 0;
                for walls in &f {
                    for item in walls {
                        if size < item.len() {
                            size = item.len();
                        }
                    }
                }
                size + 3
            },
            largest: {
                let mut largest_line_count = 0;
                for wall in f {
                    if wall.len() > largest_line_count {
                        largest_line_count = wall.len();
                    }
                }
                largest_line_count
            },
        }
    }
}

impl<'a> Display for Columns<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.make_columns())
    }
}

impl<'a> Columns<'a> {
    #[must_use]
    pub fn make_columns(&self) -> String {
        let mut i = 0;
        let mut f = String::new();
        while i < self.largest {
            let mut line = String::new();
            for item in &self.inner {
                let wall_item = item.get(i).unwrap_or(&"");
                line += wall_item;
                if wall_item.len() < self.tabsize
                    && self.inner.iter().position(|f| f == item) != Some(self.inner.len() - 1)
                {
                    line += &spaces(self.tabsize - wall_item.len());
                }
            }
            i += 1;
            f += &format!("{}\n", line);
        }
        f
    }

    #[must_use]
    pub fn set_tabsize(self, tabsize: usize) -> Self {
        Self {
            inner: self.inner,
            tabsize,
            largest: self.largest,
        }
    }

    #[must_use]
    pub fn base_tabsize_in(self, column_number: usize) -> Self {
        Self {
            inner: self.inner.clone(),
            tabsize: {
                let mut size = 0;
                self.inner[column_number].iter().for_each(|line| {
                    if line.len() > size {
                        size = line.len();
                    }
                });
                size + 3
            },
            largest: self.largest,
        }
    }
}

fn spaces(size: usize) -> String {
    let mut spaces = String::new();
    let mut i = 0;
    while i < size {
        spaces += " ";
        i += 1;
    }
    spaces
}
