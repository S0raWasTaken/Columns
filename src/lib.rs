//! A text manipulation library for displaying separate text in columns.
//!
//! # Quick Start
//! The quickest way to get column-separated text is to use the `From` implementation for
//! `Columns`.
//!  ```
//! use columns::Columns;
//!
//! let column_text = Columns::from(
//!     vec![
//!         vec!["text", "should", "be"],
//!         vec!["in", "different", "columns"],
//!         vec!["even", "in", "more", "than", "two"]
//!     ]
//! );
//!
//! println!("{}", column_text);
//! ```
//! If you're using `&format!`s, you may want to use `make_columns` method from `columns::Columns`.

#![no_std]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
use core::fmt::Display;

extern crate alloc;
use alloc::{format, string::String, vec::Vec};

/// Main crate structure, here goes all the logic.
///
/// Generates a column-separated string.
/// In the future I pretend to make columns mutable, that's why I'm using `Vec`s instead of `&[]`.
#[derive(Debug, Clone)]
pub struct Columns<'a> {
    inner: Vec<Vec<&'a str>>,
    tabsize: usize,
    largest: usize,
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
    /// Transforms a `Columns` struct into a `String`.
    /// Used in Display implementation.
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

    /// Sets tabsize manually.
    /// Can be pretty wonky, so `base_tabsize_in` is recommended instead.
    #[must_use]
    pub fn set_tabsize(self, tabsize: usize) -> Self {
        Self {
            inner: self.inner,
            tabsize,
            largest: self.largest,
        }
    }

    /// Bases tabsize in a single column, instead of checking every column and
    /// finding the largest line.
    ///
    /// Example:
    /// ```
    /// println!(
    ///     "{}",
    ///     columns::Columns::from(
    ///         vec![
    ///             vec!["a", "b", "c"],
    ///             vec!["big", "text", "that makes you annoyed"]
    ///         ]
    ///     ).base_tabsize_in(0) // Column numbers are count from 0
    /// );
    /// ```
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
