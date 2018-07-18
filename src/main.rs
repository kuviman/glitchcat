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

const GLITCH_RADIUS: usize = 15;
const ANIMATION_STEP: u64 = 16;

fn main() {
    let stdout = console::Term::stdout();
    let homoglyphs = Homoglyphs::new();
    let lines: Vec<String> = {
        use std::io::Read;
        let mut text = String::new();
        std::io::stdin()
            .read_to_string(&mut text)
            .expect("Failed to read text");
        text.lines().map(|s| s.to_owned()).collect()
    };
    for line in &lines {
        stdout.write_line(line).unwrap();
    }
    stdout.move_cursor_up(lines.len()).unwrap();
    for line in lines {
        let line: Vec<char> = line.chars().collect();
        let mut first = true;
        for glitch_center in -(GLITCH_RADIUS as isize)..(line.len() + GLITCH_RADIUS + 1) as isize {
            let glitched: String = line.iter()
                .enumerate()
                .map(|(i, &c)| {
                    use rand::Rng;
                    let dist = (i as isize - glitch_center).abs() as usize;
                    if rand::thread_rng().gen_range(0, GLITCH_RADIUS) >= dist {
                        homoglyphs.random_silimar(c)
                    } else {
                        c
                    }
                })
                .collect();
            if first {
                first = false;
            } else {
                stdout.move_cursor_up(1).unwrap();
                stdout.clear_line().unwrap();
            }
            stdout.write_line(&glitched).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(ANIMATION_STEP));
        }
        let glitched: String = line.iter().map(|&c| homoglyphs.random_silimar(c)).collect();
    }
}
