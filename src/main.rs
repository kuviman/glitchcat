extern crate console;
extern crate rand;
#[macro_use]
extern crate structopt;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use rand::Rng;
use std::collections::HashMap;
use std::io::Read;

struct Homoglyphs {
    groups: Vec<Vec<char>>,
    group_map: HashMap<char, usize>,
}

impl Homoglyphs {
    pub fn new(data: &str) -> Self {
        let mut groups = Vec::new();
        let mut group_map = HashMap::new();
        for line in data.lines() {
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
                let group = &self.groups[index];
                let i = rand::thread_rng().gen_range(0, group.len() - 1);
                if group[i] == c {
                    group[i + 1]
                } else {
                    group[i]
                }
            }
            None => c,
        }
    }
}

#[derive(Debug, EnumString)]
enum GlyphsMode {
    Default,
    CyrConv,
    HomoGlyphs,
}

impl Homoglyphs {
    pub fn new_with_mode(mode: GlyphsMode) -> Self {
        macro_rules! gen_options {
            ($mode:expr => $($option:ident),*) => {
                match $mode {
                    $(
                        GlyphsMode::$option => Self::new(include_str!(concat!(stringify!($option), ".txt"))),
                    )*
                }
            };
        }
        gen_options!(mode => Default, CyrConv, HomoGlyphs)
    }
}

const GLITCH_RADIUS: usize = 32;
const GLITCH_AMOUNT: usize = 5;
const ANIMATION_STEP: u64 = 16;

#[derive(StructOpt)]
#[structopt(about = "cat-like program with glitch-like animation")]
struct Opt {
    #[structopt(
        short = "g",
        long = "glyphs",
        default_value = "Default",
        help = "Glyphs mode (Default, CyrConv or HomoGlyphs)"
    )]
    glyphs_mode: GlyphsMode,
}

fn main() {
    use structopt::StructOpt;
    let opt = Opt::from_args();
    let stdout = console::Term::stdout();
    let homoglyphs = Homoglyphs::new_with_mode(opt.glyphs_mode);
    let lines: Vec<String> = {
        let mut text = String::new();
        std::io::stdin()
            .read_to_string(&mut text)
            .expect("Failed to read text");
        text.lines().map(|s| s.to_owned()).collect()
    };
    for line in &lines {
        stdout.write_line(line).unwrap();
    }
    if !lines.is_empty() {
        stdout.move_cursor_up(lines.len() - 1).unwrap();
    }
    for line in lines {
        let line: Vec<char> = line.chars().collect();
        let mut glitched_line = line.clone();
        for glitch_center in
            -(GLITCH_RADIUS as isize)..(glitched_line.len() + GLITCH_RADIUS + 1) as isize
        {
            if glitch_center > GLITCH_RADIUS as isize {
                for i in 0..glitch_center as usize - GLITCH_RADIUS - 1 {
                    glitched_line[i] = line[i];
                }
            }
            for _ in 0..GLITCH_AMOUNT {
                let i = rand::thread_rng().gen_range(
                    glitch_center - GLITCH_RADIUS as isize,
                    glitch_center + GLITCH_RADIUS as isize + 1,
                );
                if 0 <= i && i < line.len() as isize {
                    let i = i as usize;
                    glitched_line[i] = if rand::thread_rng().gen::<f32>() < 0.7 {
                        homoglyphs.random_silimar(line[i])
                    } else {
                        line[i]
                    };
                }
            }
            stdout.move_cursor_up(1).unwrap();
            stdout.clear_line().unwrap();
            for (&c, &initial) in glitched_line.iter().zip(line.iter()) {
                if c == initial {
                    print!("{}", c);
                } else {
                    print!("{}", console::style(c).dim());
                }
            }
            println!();
            std::thread::sleep(std::time::Duration::from_millis(ANIMATION_STEP));
        }
        stdout.move_cursor_up(1).unwrap();
        stdout.clear_line().unwrap();
        stdout
            .write_line(&line.iter().map(|&c| c).collect::<String>())
            .unwrap();
        stdout.move_cursor_down(1).unwrap();
    }
}
