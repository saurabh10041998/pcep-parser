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
            _ => panic!("[!!] Invalid Object type value"),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SrpObjectType {
    Reserved,
    Srp,
    UnAssigned,
}

impl From<u8> for SrpObjectType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Reserved,
            1 => Self::Srp,
            2..=15 => Self::UnAssigned,
            _ => panic!("[!!] Invalid Object type value"),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LspObjectType {
    Reserved,
    Lsp,
    UnAssigned,
}

impl From<u8> for LspObjectType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Reserved,
            1 => Self::Lsp,
            2..=15 => Self::UnAssigned,
            _ => panic!("[!!] Invalid Object type value"),
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BandwidthObjectType {
    Reserved,
    Requested,
    RequestedOpt,
    Genric,
    GenericOpt,
    UnAssigned,
}

impl From<u8> for BandwidthObjectType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Reserved,
            1 => Self::Requested,
            2 => Self::RequestedOpt,
            3 => Self::Genric,
            4 => Self::GenericOpt,
            5..=15 => Self::UnAssigned,
            _ => panic!("[!!] Invalid Object type value for Bandwidth object"),
        }
    }
}
