# Columns
A text manipulation library for displaying separate text in columns
## Example
```rs
use columns::Columns;

println!(
    "{}",
    Columns::from(vec![
        vec!["line1", "line2", "line3"],
        vec!["should", "be", "displayed", "side by side"],
    ])
    .base_tabsize_in(0) // Sets the tabsize to be based in the first one. This is to prevent unnecessary spacing
);
```
Result:
```
line1   should
line2   be
line3   displayed
        side by side
```

## TODO list
- Documentation
- Post on crates.io
- Customizable separators
