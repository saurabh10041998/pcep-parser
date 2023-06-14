#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Version {
    One,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::One => write!(f, "Version::one"),
        }
    }
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            1 => Version::One,
            _ => unimplemented!("Unknown version: {}", value),
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::One
    }
}
