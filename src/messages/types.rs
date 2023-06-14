#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    Open,
    Keepalive,
    PCReq,
    PCRep,
    PCNtf,
    PCErr,
    PCClose,
    PCRpt,
    PCUpd,
    UnKnown(u8),
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Open => write!(f, "Open"),
            MessageType::Keepalive => write!(f, "Keepalive"),
            MessageType::PCReq => write!(f, "Path Computation Request"),
            MessageType::PCRep => write!(f, "Path Computation Reply"),
            MessageType::PCNtf => write!(f, "Notification"),
            MessageType::PCErr => write!(f, "Error"),
            MessageType::PCClose => write!(f, "Close"),
            MessageType::PCRpt => write!(f, "Path Computation LSP State Report"),
            MessageType::PCUpd => write!(f, "Path Computation LSP update request message"),
            MessageType::UnKnown(x) => write!(f, "Unknown message type: {}", *x),
        }
    }
}

// TODO: seperate mod for errors
#[derive(Debug)]
pub enum MessageTypeError {
    UnknownMessageTypeError(u8)
}


impl Default for MessageType {
    fn default() -> Self {
        MessageType::UnKnown(0)
    }
}


impl TryFrom<u8> for MessageType {
    type Error = MessageTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Open),
            2 => Ok(Self::Keepalive),
            3 => Ok(Self::PCReq),
            4 => Ok(Self::PCRep),
            5 => Ok(Self::PCNtf),
            6 => Ok(Self::PCErr),
            7 => Ok(Self::PCClose),
            10 => Ok(Self::PCRpt),
            11 => Ok(Self::PCUpd),
            _ => Err(MessageTypeError::UnknownMessageTypeError(value)),
        }
    }
}