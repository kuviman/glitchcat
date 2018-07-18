extern crate console;

use std::collections::{HashMap, HashSet};

struct Homoglyphs {
    groups: Vec<HashSet<char>>,
    group_map: HashMap<char, usize>,
}

impl Homoglyphs {
    pub fn new() -> Self {
        let mut groups = Vec::new();
        let mut group_map = HashMap::new();
        for line in include_str!("homoglyphs.txt").lines() {
            if line.starts_with('#') {
                continue;
            }
            let group = line.chars().collect();
            for &c in &group {
                group_map.insert(c, groups.len());
            }
            groups.push(group);
        }
        Self { groups, group_map }
    }
}

fn main() {
    let homoglyphs = Homoglyphs::new();
    for c in "Hello, world!".chars() {
        println!("{:?}: {:?}", c, homoglyphs.groups[homoglyphs.group_map[&c]]);
    }
}
