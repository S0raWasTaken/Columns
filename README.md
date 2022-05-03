# Columns
Formats text separated in columns just like most fetching programs do

## Example
```rs
use columns::Columns;

println!(
    "{}",
    Columns::from(vec![
        vec!["line1", "line2", "line3"],
        vec!["should", "be", "displayed", "side by side"],
    ])
    .set_tabsize(8) // Reduces distance between columns. Should be used only if last column has big line sizes
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
