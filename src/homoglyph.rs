use *;

pub struct Homoglyphs {
    groups: HashMap<char, Vec<char>>,
}

impl Homoglyphs {
    pub fn new(data: &str) -> Self {
        let mut groups = HashMap::new();
        for line in data.lines() {
            if line.starts_with('#') {
                continue;
            }
            let group: HashSet<char> = line.chars().collect();
            for &c1 in &group {
                for &c2 in &group {
                    let mut insert = |c_in, c_out| {
                        if !groups.contains_key(&c_in) {
                            groups.insert(c_in, HashSet::new());
                        }
                        groups.get_mut(&c_in).unwrap().insert(c_out);
                    };
                    insert(c1, c2);
                    insert(c2, c2);
                }
            }
        }
        Self {
            groups: groups
                .into_iter()
                .map(|(c, mut group)| {
                    group.remove(&c);
                    assert!(!group.is_empty());
                    (c, group.into_iter().collect())
                })
                .collect(),
        }
    }

    pub fn random_silimar(&self, c: char) -> char {
        match self.groups.get(&c) {
            Some(group) => group[rand::thread_rng().gen_range(0, group.len())],
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
