use crate::messages::header::CommonHeader;
use colored::Colorize;
use indoc::writedoc;

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
        let title = "##[KeepAlive]##".yellow();
        writedoc!(
            f,
            r#"
            {title}
                {common_header}
            "#,
            title = title,
            common_header = self.common_header
        )
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::common::Version;
    use crate::messages::types::MessageType;
    const EMPTY_SLICE: &[u8] = &[];
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
