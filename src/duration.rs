use *;

pub fn parse_duration(s: &str) -> Result<std::time::Duration, <u64 as FromStr>::Err> {
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

pub fn to_millis(duration: std::time::Duration) -> u64 {
    duration.as_secs() * 1000 + duration.subsec_millis() as u64
}
