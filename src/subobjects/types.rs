use super::sr::SrSubobject;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum SubObjectTypes {
    Ipv4Prefix,
    Ipv6Prefix,
    Sr(SrSubobject),
    As,
    Unknown(u8),
}

impl From<u8> for SubObjectTypes {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Ipv4Prefix,
            2 => Self::Ipv6Prefix,
            32 => Self::As,
            36 => Self::Sr(Default::default()),
            _ => Self::Unknown(value),
        }
    }
}

impl std::fmt::Display for SubObjectTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ipv4Prefix => write!(f, "SubObjectType::Ipv4Prefix"),
            Self::Ipv6Prefix => write!(f, "SubObjectType::Ipv6Prefix"),
            Self::Sr(x) => write!(f, "{}", x),
            Self::As => write!(f, "SubObjectType::AS"),
            Self::Unknown(x) => write!(f, "[!!] Unknown subobject type: {}", *x),
        }
    }
}
