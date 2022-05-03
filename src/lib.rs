use std::fmt::Display;

#[derive(Debug)]
pub struct Columns<'a> {
    inner: Vec<Vec<&'a str>>,
    tabsize: usize,
    largest: usize,
    separator: char,
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
            separator: ' ',
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
            separator: ' ',
        }
    }
}

impl<'a> Display for Columns<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.make_columns())
    }
}

impl<'a> Columns<'a> {
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
                    line += &spaces(self.tabsize - wall_item.len(), self.separator);
                }
            }
            i += 1;
            f += &format!("{}\n", line.trim_end());
        }
        f
    }

    pub fn set_tabsize(&self, tabsize: usize) -> Self {
        Self {
            inner: self.inner.clone(),
            tabsize,
            largest: self.largest,
            separator: self.separator,
        }
    }

    pub fn set_separator(&self, separator: char) -> Self {
        Self {
            inner: self.inner.clone(),
            tabsize: self.tabsize,
            largest: self.largest,
            separator,
        }
    }
}

fn spaces(size: usize, separator: char) -> String {
    let mut spaces = String::new();
    let mut i = 0;
    while i < size {
        spaces += " ";
        i += 1;
    }
    spaces.insert(spaces.len() / 2, separator);
    spaces.remove(spaces.len() / 2 + 1);
    spaces
}
