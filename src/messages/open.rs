use crate::messages::header::CommonHeader;
use crate::objects::open::OpenObject;
use colored::Colorize;
use indoc::writedoc;

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
        let title = "##[Open]##".yellow();
        writedoc!(
            f,
            r#"
            {title}
                {common_header}
                {open_object}
            "#,
            title = title,
            common_header = self.common_header,
            open_object = self.open_object
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::Version;
    use crate::messages::types::MessageType;
    use crate::objects::classes::ObjectClassType;
    use crate::objects::header::CommonObject;
    use crate::objects::open::OpenObject;
    use crate::objects::types::OpenObjectType;
    use crate::tlvs::tlv_set::{SrPCECapabilityTLV, StatefulPCECapabilityTLV, UnknownTLV};
    use crate::tlvs::types::Tlv;
    #[test]
    fn test_open_message_parsing() {
        let input: &[u8] = &[
            0x20, 0x01, 0x00, 0x24, 0x01, 0x10, 0x00, 0x20, 0x20, 0x1e, 0x78, 0x01, 0x00, 0x10,
            0x00, 0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x1a, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0a,
            0x00, 0x23, 0x00, 0x02, 0x00, 0x14, 0x00, 0x00,
        ];
        let (remaining, common_header) =
            CommonHeader::parse_common_header(input).expect("[!!] Error in parsing common header");
        let (_remaining, open_object) =
            OpenObject::parse_open_object(remaining).expect("[!!] Error while parsing open object");

        let open_message = Open {
            common_header,
            open_object,
        };

        let expected_spc_tlv = StatefulPCECapabilityTLV {
            tlv_type: 16,
            tlv_len: 4,
            flag_lsp_update_capability: true,
            flag_include_db_version: false,
            flag_lsp_instantiate_capability: true,
            flag_triggered_resync: false,
            flag_delta_lsp_sync_capability: false,
            flag_triggered_initial_sync: false,
        };

        let expected_srpc_tlv = SrPCECapabilityTLV {
            tlv_type: 26,
            tlv_len: 4,
            reserved: 0,
            flag_limit: false,
            max_sid_depth: 10,
        };

        // TODO: Code to parse this Tlv
        let unknown_tlv = UnknownTLV {
            tlv_type: 35,
            tlv_len: 2,
            tlv_data: vec![0x00, 0x14],
        };
        let expected_open_object = OpenObject {
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
            tlvs: Some(vec![
                Tlv::StatefulPCECapability(expected_spc_tlv),
                Tlv::SrPCECapability(expected_srpc_tlv),
                Tlv::Unknown(unknown_tlv),
            ]),
        };

        let expected_ch = CommonHeader {
            message_type: MessageType::Open,
            version: Version::One,
            flags: 0,
            message_length: 36,
        };
        let expected_open_message = Open {
            common_header: expected_ch,
            open_object: expected_open_object,
        };
        assert_eq!(open_message, expected_open_message);
    }
}
