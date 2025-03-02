use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut set: HashSet<&str> = HashSet::new();
    input_str.split(',').for_each(|line| {
        set.insert(line);
    });
    set.len()
}
