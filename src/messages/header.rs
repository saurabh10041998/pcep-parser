use colored::Colorize;
use indoc::writedoc;
use nom::bits;
use nom::combinator::map_res;
use nom::error::Error;
use nom::number;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::Version;
use crate::messages::types::MessageType;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct CommonHeader {
    pub version: Version,
    pub flags: u8,
    pub message_type: MessageType,
    pub message_length: u16,
}

impl std::fmt::Display for CommonHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "==[Common Header]==".green().bold();
        writedoc!(
            f,
            r#"
            {title}
                version: {version},
                flags: {flags},
                message_type: {mtype},
                message_length: {mlength}
            "#,
            title = title,
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
        let (input, message_type): (&[u8], MessageType) =
            map_res(number::streaming::be_u8, |val| val.try_into())(input)?;
        let (input, message_length) = number::streaming::be_u16(input)?;
        //let message_type: MessageType = message_types.try_into().unwrap();
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

#[cfg(test)]
pub mod tests {
    use super::*;
    const EMPTY_SLICE: &[u8] = &[];
    #[test]
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
}
