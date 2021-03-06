extern crate console;
extern crate rand;
#[macro_use]
extern crate structopt;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate failure;

mod duration;
mod homoglyph;
mod percent;

use duration::*;
use homoglyph::*;
use percent::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::str::FromStr;

#[derive(StructOpt)]
#[structopt(about = "cat-like program with glitch animation")]
struct Opt {
    #[structopt(help = "File to read input from")]
    file: Option<String>,
    #[structopt(
        short = "m",
        long = "mode",
        default_value = "Default",
        help = "Glyphs mode (Default, CyrConv or HomoGlyphs)"
    )]
    mode: GlyphsMode,
    #[structopt(
        short = "d",
        long = "duration",
        default_value = "1000",
        help = "Duration of animation in millis (or \"infinite\"/\"inf\")"
    )]
    duration: Duration,
    #[structopt(
        short = "s",
        long = "step",
        default_value = "100",
        help = "Animation step in millis",
        parse(try_from_str = "parse_duration")
    )]
    step: std::time::Duration,
    #[structopt(
        short = "a",
        long = "amount",
        default_value = "90",
        help = "Percentage of symbols glitched each animation step"
    )]
    amount: Percent,
    #[structopt(
        short = "g",
        long = "glitchness",
        default_value = "80",
        help = "Probability of a symbol to be glitched into other symbol"
    )]
    glitchness: Percent,
    #[structopt(
        short = "f",
        long = "fade",
        default_value = "400",
        help = "Time to fade back to normal text in millis",
        parse(try_from_str = "parse_duration")
    )]
    fade: std::time::Duration,
    #[structopt(short = "p", long = "plain", help = "Disable animation and colors")]
    plain: bool,
}

struct GlitchCat {
    opt: Opt,
    homoglyphs: Homoglyphs,
    term: console::Term,
    initial_lines: Vec<Vec<char>>,
    lines: Vec<Vec<char>>,
    start_instant: std::time::Instant,
    rng: Box<rand::RngCore>,
}

impl GlitchCat {
    fn new(opt: Opt) -> Self {
        let term = console::Term::buffered_stdout();
        let initial_lines = Self::read_input(&opt, &term);
        let lines = initial_lines.clone();
        Self {
            homoglyphs: Homoglyphs::new_with_mode(opt.mode),
            opt,
            term,
            initial_lines,
            lines,
            start_instant: std::time::Instant::now(),
            rng: Box::new(rand::thread_rng()),
        }
    }

    fn read_input(opt: &Opt, term: &console::Term) -> Vec<Vec<char>> {
        let term_width = term.size().1 as usize;
        let mut text = String::new();
        let mut input: Box<Read> = match opt.file {
            Some(ref path) => match std::fs::File::open(path) {
                Ok(file) => Box::new(file),
                Err(_) => {
                    eprintln!(
                        "{}",
                        console::style(format!("Failed to read {:?}", path)).red()
                    );
                    std::process::exit(1);
                }
            },
            None => Box::new(std::io::stdin()),
        };
        input
            .read_to_string(&mut text)
            .expect("Failed to read text");
        let mut lines = Vec::new();
        for line in text.lines() {
            let line: Vec<char> = line.chars().collect();
            let mut line: &[char] = &line;
            while line.len() > term_width {
                lines.push(line[0..term_width].to_owned());
                line = &line[term_width..]
            }
            lines.push(line.to_owned());
        }
        lines
    }

    fn print(&self) {
        for (initial_line, line) in self.initial_lines.iter().zip(self.lines.iter()) {
            let mut output = String::new();
            for (&initial_c, &c) in initial_line.iter().zip(line.iter()) {
                if self.opt.plain || initial_c == c {
                    output.push(c);
                } else {
                    output += &console::style(c).dim().to_string();
                }
            }
            self.term.write_line(&output).unwrap();
        }
        self.term.flush().unwrap();
    }

    fn update(&mut self) {
        let mut glitchness = self.opt.glitchness;
        if let Duration::Some(duration) = self.opt.duration {
            let time_left = duration - self.start_instant.elapsed();
            if time_left < self.opt.fade {
                glitchness = Percent::new(
                    (glitchness.as_u8() as usize * to_millis(time_left) as usize
                        / to_millis(self.opt.fade) as usize) as u8,
                );
            }
        }
        for (initial_line, line) in self.initial_lines.iter().zip(self.lines.iter_mut()) {
            for (&initial_c, c) in initial_line.iter().zip(line.iter_mut()) {
                if self.rng.gen::<Percent>() < glitchness {
                    if self.rng.gen::<Percent>() < self.opt.amount {
                        *c = self.homoglyphs.random_silimar(initial_c);
                    }
                } else {
                    *c = initial_c;
                }
            }
        }
    }

    fn run(mut self) {
        if self.opt.plain {
            self.update();
            self.print();
            return;
        }
        self.print();
        loop {
            if let Duration::Some(duration) = self.opt.duration {
                if self.start_instant.elapsed() >= duration {
                    break;
                }
            }
            self.update();
            self.term.clear_last_lines(self.lines.len()).unwrap();
            self.print();
            std::thread::sleep(self.opt.step);
        }
        self.term.clear_last_lines(self.lines.len()).unwrap();
        self.lines = self.initial_lines.clone();
        self.print();
    }
}

fn main() {
    use structopt::StructOpt;
    let opt = Opt::from_args();
    let glitchcat = GlitchCat::new(opt);
    glitchcat.run();
}
