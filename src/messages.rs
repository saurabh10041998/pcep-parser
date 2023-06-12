use indoc::writedoc;
use nom::bits;
use nom::error::Error;
use nom::number;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::Version;
use crate::objects::OpenObject;

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
            MessageType::UnKnown(x) => write!(f, "Unknown message type: {}", *x),
        }
    }
}

impl TryFrom<u8> for MessageType {
    type Error = &'static str;
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
            _ => Ok(Self::UnKnown(value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommonHeader {
    pub version: Version,
    pub flags: u8,
    pub message_type: MessageType,
    pub message_length: u16,
}

impl std::fmt::Display for CommonHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            r#"
            ==[Common Header]==
                version: {version},
                flags: {flags},
                message_type: {mtype},
                message_length: {mlength}
            "#,
            version = { self.version },
            flags = { self.flags },
            mtype = { self.message_type },
            mlength = { self.message_length }
        )
    }
}

impl CommonHeader {
    pub fn parse_common_header(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, ver_flags) = Self::parse_version_flags(input)?;
        let (input, message_types) = number::streaming::be_u8(input)?;
        let (input, message_length) = number::streaming::be_u16(input)?;
        let message_type: MessageType = message_types.try_into().unwrap();
        let header = CommonHeader {
            version: ver_flags.0.into(),
            flags: ver_flags.1,
            message_type,
            message_length,
        };
        Ok((input, header))
    }
    fn parse_version_flags(input: &[u8]) -> IResult<&[u8], (u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(3u8),
            bits::streaming::take(5u8),
        )))(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Open {
    common_header: CommonHeader,
    open_object: OpenObject,
}

impl Open {
    pub fn new(common_header: CommonHeader, open_object: OpenObject) -> Self {
        Open {
            common_header,
            open_object,
        }
    }
}

impl std::fmt::Display for Open {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            r#"
            ##[Open]##
                {common_header}
                {open_object}
            "#,
            common_header = self.common_header,
            open_object = self.open_object
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct KeepAlive {
    common_header: CommonHeader,
}

impl From<CommonHeader> for KeepAlive {
    fn from(common_header: CommonHeader) -> Self {
        KeepAlive { common_header }
    }
}

impl std::fmt::Display for KeepAlive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            r#"
            ##[KeepAlive]##
                {common_header}
            "#,
            common_header = self.common_header
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ObjectClassType;
    use crate::objects::{CommonObject, OpenObject};
    use crate::types::OpenObjectType;
    const EMPTY_SLICE: &[u8] = &[];
    #[test]
    // Common Header parsing test
    fn test_ch_message_parsing() {
        let input: &[u8] = &[0x20, 0x01, 0x00, 0x24];
        let (remaing, common_header) = CommonHeader::parse_common_header(input)
            .expect("[!!] Failure while parsing common header");
        let expected = CommonHeader {
            version: Version::One,
            flags: 0,
            message_type: MessageType::Open,
            message_length: 36,
        };
        assert_eq!(common_header, expected);
        assert_eq!(remaing, EMPTY_SLICE);
    }

    #[test]
    fn test_open_message_parsing() {
        let input: &[u8] = &[
            0x20, 0x01, 0x00, 0x24, 0x01, 0x10, 0x00, 0x20, 0x20, 0x1e, 0x78, 0x01, 0x00, 0x10,
            0x00, 0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x1a, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0a,
            0x00, 0x23, 0x00, 0x02, 0x00, 0x14, 0x00, 0x00,
        ];
        let (remaining, _common_header) =
            CommonHeader::parse_common_header(input).expect("[!!] Error in parsing common header");
        let (_remaining, open_object) =
            OpenObject::parse_open_object(remaining).expect("[!!] Error while parsing open object");
        let expected = OpenObject {
            common_object: CommonObject {
                object_class_type: ObjectClassType::Open(OpenObjectType::Open),
                flag_ignore: false,
                flag_process: false,
                reserved: 0,
                object_length: 32,
            },
            version: Version::One,
            flags: 0,
            keepalive: 30,
            deadtimer: 120,
            sid: 1,
            tlvs: None
        };
        assert_eq!(open_object, expected);
    }

    #[test]
    fn test_keepalive_message_parsing() {
        let input: &[u8] = &[0x20, 0x02, 0x00, 0x04];
        let (remaining, common_header) = CommonHeader::parse_common_header(input)
            .expect("[!!] Error while parsing common header");
        let keep_alive_message: KeepAlive = KeepAlive { common_header };
        let expected = KeepAlive {
            common_header: CommonHeader {
                version: Version::One,
                flags: 0,
                message_type: MessageType::Keepalive,
                message_length: 4,
            },
        };
        assert_eq!(keep_alive_message, expected);
        assert_eq!(remaining, EMPTY_SLICE);
    }
}
