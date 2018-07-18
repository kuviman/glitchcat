extern crate console;
extern crate rand;

use std::collections::HashMap;

struct Homoglyphs {
    groups: Vec<Vec<char>>,
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

    pub fn random_silimar(&self, c: char) -> char {
        match self.group_map.get(&c) {
            Some(&index) => {
                use rand::Rng;
                let group = &self.groups[index];
                group[rand::thread_rng().gen_range(0, group.len())]
            }
            None => c,
        }
    }
}

fn main() {
    use std::io::Read;
    let homoglyphs = Homoglyphs::new();
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text).unwrap();
    let glitched: String = text.chars().map(|c| homoglyphs.random_silimar(c)).collect();
    println!("{}", glitched);
}
