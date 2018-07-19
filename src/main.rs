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
        help = "Duration of animation in millis (negative means infinite)"
    )]
    duration: i64,
    #[structopt(
        short = "s", long = "step", default_value = "100", help = "Animation step in millis"
    )]
    step: u64,
    #[structopt(
        short = "a",
        long = "amount",
        default_value = "30",
        help = "Percentage of symbols glitched each animation step"
    )]
    amount: usize,
    #[structopt(
        short = "g",
        long = "glitchness",
        default_value = "60",
        help = "Probability of a symbol to be glitched into other symbol"
    )]
    glitchness: usize,
    #[structopt(
        short = "f",
        long = "fade",
        default_value = "400",
        help = "Time to fade back to normal text in millis"
    )]
    fade: u64,
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
    let duration = if opt.duration < 0 {
        None
    } else {
        Some(std::time::Duration::from_millis(opt.duration as u64))
    };
    let mut lines = initial_lines.clone();
    print(&stdout, &initial_lines, &lines, true);
    let start_instant = std::time::Instant::now();
    let mut rng = rand::thread_rng();
    loop {
        let mut glitchness = opt.glitchness;
        if let Some(duration) = duration {
            if start_instant.elapsed() >= duration {
                break;
            }
            let time_left = duration - start_instant.elapsed();
            let time_left = time_left.as_secs() * 1000 + time_left.subsec_millis() as u64;
            if time_left < opt.fade {
                glitchness = glitchness * (opt.fade - time_left) as usize / opt.fade as usize;
            }
        }
        for _ in 0..opt.amount {
            let row = rng.gen_range(0, lines.len());
            let line = &mut lines[row];
            if line.is_empty() {
                continue;
            }
            let col = rng.gen_range(0, line.len());
            let c = &mut line[col];
            *c = if rng.gen_range(0, 100) < glitchness {
                homoglyphs.random_silimar(initial_lines[row][col])
            } else {
                initial_lines[row][col]
            }
        }
        print(&stdout, &initial_lines, &lines, false);
        std::thread::sleep(std::time::Duration::from_millis(opt.step));
    }
    print(&stdout, &initial_lines, &initial_lines, false);
}
