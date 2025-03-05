use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let parts =  input_str.split(',');
    let mut set = HashSet::new();
    for ch in parts {
        set.insert(ch);
    }
    set.len()
}
