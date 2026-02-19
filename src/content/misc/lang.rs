#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang { En, Ru }

impl Lang {
    pub fn from_str(s: &str) -> Self {
        if s.contains("en") { Lang::En } else { Lang::Ru }
    }
}