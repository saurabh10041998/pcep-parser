#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpenObjectType {
    Reserved,
    Open,
    UnAssigned,
}

impl From<u8> for OpenObjectType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Reserved,
            1 => Self::Open,
            2..=15 => Self::UnAssigned,
            _ => panic!("[!!] Invalid Object class value"),
        }
    }
}
