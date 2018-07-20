use *;

#[derive(Fail, Debug)]
pub enum ParsePercentError {
    #[fail(display = "Value should be between 0 and 100")]
    TooBig,
    #[fail(display = "{}", _0)]
    ParseIntError(#[cause] <u8 as FromStr>::Err),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Percent(u8);

impl Percent {
    pub fn new(value: u8) -> Self {
        assert!(value <= 100);
        Percent(value)
    }
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl FromStr for Percent {
    type Err = ParsePercentError;
    fn from_str(s: &str) -> Result<Self, ParsePercentError> {
        match s.parse() {
            Ok(value) if value <= 100 => Ok(Percent(value)),
            Ok(_) => Err(ParsePercentError::TooBig),
            Err(e) => Err(ParsePercentError::ParseIntError(e)),
        }
    }
}

impl PartialOrd for Percent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl rand::distributions::Distribution<Percent> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Percent {
        Percent(rng.gen_range(0, 100))
    }
}
