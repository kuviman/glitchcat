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
    let homoglyphs = Homoglyphs::new();
    let mut line = String::new();
    loop {
        let line = {
            line.clear();
            std::io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            if line.len() == 0 {
                break;
            }
            line.trim_right_matches('\n').trim_right_matches('\r')
        };
        let glitched: String = line.chars().map(|c| homoglyphs.random_silimar(c)).collect();
        println!("{}", glitched);
    }
}
