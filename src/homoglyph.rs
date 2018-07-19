use *;

pub struct Homoglyphs {
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
pub enum GlyphsMode {
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
                        GlyphsMode::$option => Self::new(include_str!(concat!("../modes/", stringify!($option), ".txt"))),
                    )*
                }
            };
        }
        gen_options!(mode => Default, CyrConv, HomoGlyphs)
    }
}
