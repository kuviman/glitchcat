extern crate console;
extern crate rand;
#[macro_use]
extern crate structopt;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod homoglyph;

use homoglyph::*;
use rand::Rng;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

fn parse_duration(s: &str) -> Result<std::time::Duration, <u64 as FromStr>::Err> {
    Ok(std::time::Duration::from_millis(s.parse()?))
}

pub enum Duration {
    Some(std::time::Duration),
    Infinite,
}

impl FromStr for Duration {
    type Err = <u64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "infinite" || s == "inf" {
            Ok(Duration::Infinite)
        } else {
            Ok(Duration::Some(parse_duration(s)?))
        }
    }
}

fn to_millis(duration: std::time::Duration) -> u64 {
    duration.as_secs() * 1000 + duration.subsec_millis() as u64
}

#[derive(StructOpt)]
#[structopt(about = "cat-like program with glitch-like animation")]
struct Opt {
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
        help = "Duration of animation in millis (of \"infinite\"/\"inf\")"
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
    amount: usize,
    #[structopt(
        short = "g",
        long = "glitchness",
        default_value = "80",
        help = "Probability of a symbol to be glitched into other symbol"
    )]
    glitchness: usize,
    #[structopt(
        short = "f",
        long = "fade",
        default_value = "400",
        help = "Time to fade back to normal text in millis",
        parse(try_from_str = "parse_duration")
    )]
    fade: std::time::Duration,
}

fn print(
    stdout: &console::Term,
    initial_lines: &Vec<Vec<char>>,
    lines: &Vec<Vec<char>>,
    first: bool,
) {
    if !first {
        stdout.clear_last_lines(lines.len()).unwrap();
    }
    for (initial_line, line) in initial_lines.iter().zip(lines.iter()) {
        let mut output = String::new();
        for (&initial_c, &c) in initial_line.iter().zip(line.iter()) {
            if initial_c == c {
                output.push(c);
            } else {
                output += &console::style(c).dim().to_string();
            }
        }
        stdout.write_line(&output).unwrap();
    }
    stdout.flush().unwrap();
}

fn main() {
    use structopt::StructOpt;
    let opt = Opt::from_args();
    let stdout = console::Term::buffered_stdout();
    let stdout_width = stdout.size().1 as usize;
    let homoglyphs = Homoglyphs::new_with_mode(opt.mode);
    let initial_lines: Vec<Vec<char>> = {
        let mut text = String::new();
        std::io::stdin()
            .read_to_string(&mut text)
            .expect("Failed to read text");
        let mut lines = Vec::new();
        for line in text.lines() {
            let line: Vec<char> = line.chars().collect();
            let mut line: &[char] = &line;
            while line.len() > stdout_width {
                lines.push(line[0..stdout_width].to_owned());
                line = &line[stdout_width..]
            }
            lines.push(line.to_owned());
        }
        lines
    };
    let mut lines = initial_lines.clone();
    print(&stdout, &initial_lines, &lines, true);
    let start_instant = std::time::Instant::now();
    let mut rng = rand::thread_rng();
    loop {
        let mut glitchness = opt.glitchness;
        if let Duration::Some(duration) = opt.duration {
            if start_instant.elapsed() >= duration {
                break;
            }
            let time_left = duration - start_instant.elapsed();
            if time_left < opt.fade {
                glitchness =
                    glitchness * to_millis(time_left) as usize / to_millis(opt.fade) as usize;
            }
        }
        for (initial_line, line) in initial_lines.iter().zip(lines.iter_mut()) {
            for (&initial_c, c) in initial_line.iter().zip(line.iter_mut()) {
                if rng.gen_range(0, 100) < glitchness {
                    if rng.gen_range(0, 100) < opt.amount {
                        *c = homoglyphs.random_silimar(initial_c);
                    }
                } else {
                    *c = initial_c;
                }
            }
        }
        print(&stdout, &initial_lines, &lines, false);
        std::thread::sleep(opt.step);
    }
    print(&stdout, &initial_lines, &initial_lines, false);
}
