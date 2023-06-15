use nom::bytes;
use nom::IResult;

pub struct Utils;

impl Utils {
    pub fn consume_padding(len: usize, input: &[u8]) -> IResult<&[u8], &[u8]> {
        match len % 4 {
            0 => Ok((input, &[])),
            1 => {
                // consume 3 bytes
                let (input, padding) = bytes::streaming::take(3usize)(input)?;
                Ok((input, padding))
            }
            2 => {
                // consume 2 bytes
                let (input, padding) = bytes::streaming::take(2usize)(input)?;
                Ok((input, padding))
            }
            3 => {
                // consume 1 byte
                let (input, padding) = bytes::streaming::take(1usize)(input)?;
                Ok((input, padding))
            }
            _ => unreachable!(),
        }
    }
}

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

#[non_exhaustive]
// Operational Status for LSP
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationalStatus {
    Down,
    Up,
    Active,
    GoingDown,
    GoingUp,
    Reserved,
}

impl From<u8> for OperationalStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Down,
            1 => Self::Up,
            2 => Self::Active,
            3 => Self::GoingDown,
            4 => Self::GoingUp,
            5..=7 => Self::Reserved,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for OperationalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Down => write!(f, "OperationalStatus::DOWN"),
            Self::Up => write!(f, "OperationalStatus::UP"),
            Self::Active => write!(f, "OperationalStatus::ACTIVE"),
            Self::GoingDown => write!(f, "OperationalStatus::GOING-DOWN"),
            Self::GoingUp => write!(f, "OperationalStatus::GOING-UP"),
            Self::Reserved => write!(f, "OperationalStatus::Reserved"),
        }
    }
}
