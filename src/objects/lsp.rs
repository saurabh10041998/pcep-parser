use colored::Colorize;
use indoc::writedoc;
use nom::bits;
use nom::bytes;
use nom::error::{Error, ErrorKind};
use nom::sequence::tuple;
use nom::{Err, IResult};

use crate::common::OperationalStatus;
use crate::objects::header::CommonObject;
use crate::tlvs::tlv_parser::Parser;
use crate::tlvs::types::Tlv;

use super::classes::ObjectClassType;
use super::types::LspObjectType;

#[derive(Debug, PartialEq, Eq)]
pub struct LspObject {
    pub common_object: CommonObject,
    pub plsp_id: u32,
    pub operational_status: OperationalStatus,
    pub flag_administrative: bool,
    pub flag_remove: bool,
    pub flag_sync: bool,
    pub flag_delegate: bool,
    pub tlvs: Option<Vec<Tlv>>,
}

impl LspObject {
    // Parse
    // plsp-id : 20 bits
    // reserved: 5 bits
    // flags : 12 bits as follows
    // operational_status : 3 bits
    // a flag: 1 bit
    // r flag: 1 bit
    // s flag : 1 bit
    // d flag : 1 bit
    fn parse_plsp_id_res_oper_flag_a_r_s_d(
        input: &[u8],
    ) -> IResult<&[u8], (u32, u8, u8, u8, u8, u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(20u32),
            bits::streaming::take(5u8),
            bits::streaming::take(3u8),
            bits::streaming::take(1u8),
            bits::streaming::take(1u8),
            bits::streaming::take(1u8),
            bits::streaming::take(1u8),
        )))(input)
    }

    pub fn parse_lsp_object(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, common_object) = CommonObject::parse_common_object(input)?;
        if let ObjectClassType::Lsp(LspObjectType::Lsp) = common_object.object_class_type {
            let object_body_len = common_object.object_length - 4;
            let (remaining, object_body) =
                bytes::streaming::take(object_body_len as usize)(remaining)?;
            let (object_body, plsp_id_res_oper_flag_a_r_s_d) =
                Self::parse_plsp_id_res_oper_flag_a_r_s_d(object_body)?;
            let mut lsp_object = LspObject {
                common_object,
                plsp_id: plsp_id_res_oper_flag_a_r_s_d.0,
                operational_status: plsp_id_res_oper_flag_a_r_s_d.2.into(),
                flag_administrative: plsp_id_res_oper_flag_a_r_s_d.3 & 0b1 == 0b1,
                flag_remove: plsp_id_res_oper_flag_a_r_s_d.4 & 0b1 == 0b1,
                flag_sync: plsp_id_res_oper_flag_a_r_s_d.5 & 0b1 == 0b1,
                flag_delegate: plsp_id_res_oper_flag_a_r_s_d.6 & 0b1 == 0b1,
                tlvs: None,
            };
            if !object_body.is_empty() {
                let (_object_body, tlvs) = Parser::parse_tlvs(object_body)?;
                lsp_object.tlvs = Some(tlvs);
            }
            return Ok((remaining, lsp_object));
        }
        Err(Err::Failure(Error::new(input, ErrorKind::Fail)))
    }
}

impl std::fmt::Display for LspObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tlvs_str = String::new();
        if let Some(ref tlvs) = self.tlvs {
            for t in tlvs {
                let output = format!("{}", t);
                tlvs_str.push_str(&output)
            }
        }
        let title = "==[LSP Object]==".green().bold();
        writedoc!(
            f,
            r#"
            {title}
                {common_object}
                plsp_id                      = {plsp_id}
                operational_status           = {operational_status}
                flag_administrative          = {flag_administrative}
                flag_remove                  = {flag_remove}
                flag_sync                    = {flag_sync}
                flag_delegate                = {flag_delegate}

            {tlv_str}
            "#,
            title = title,
            common_object = self.common_object,
            plsp_id = self.plsp_id,
            operational_status = self.operational_status,
            flag_administrative = self.flag_administrative,
            flag_remove = self.flag_remove,
            flag_sync = self.flag_sync,
            flag_delegate = self.flag_delegate,
            tlv_str = tlvs_str
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tlvs::tlv_set::{Ipv4LSPIndetifiersTLV, SymbolicPathNameTLV};
    use crate::tlvs::types::Tlv;
    use std::net::Ipv4Addr;
    #[test]
    fn test_lsp_object_parsing() {
        let input: &[u8] = &[
            0x20, 0x10, 0x00, 0x38, 0x00, 0x0e, 0xb0, 0x09, 0x00, 0x11, 0x00, 0x15, 0x63, 0x66,
            0x67, 0x5f, 0x50, 0x53, 0x41, 0x44, 0x45, 0x4c, 0x2d, 0x35, 0x5f, 0x64, 0x69, 0x73,
            0x63, 0x72, 0x5f, 0x31, 0x30, 0x00, 0x00, 0x00, 0x00, 0x12, 0x00, 0x10, 0x0a, 0x64,
            0x00, 0x69, 0x00, 0x00, 0x00, 0xae, 0x0a, 0x64, 0x00, 0x69, 0x0a, 0x68, 0x69, 0x01,
        ];

        let (remaining, lsp_object) =
            LspObject::parse_lsp_object(input).expect("[!!] Error while parsing lsp object");

        let expected_cobj = CommonObject {
            object_class_type: ObjectClassType::Lsp(LspObjectType::Lsp),
            flag_ignore: false,
            flag_process: false,
            reserved: 0,
            object_length: 56,
        };

        let symolic_pathname_tlv = SymbolicPathNameTLV {
            tlv_type: 17,
            tlv_len: 21,
            symbolic_path_name: String::from("cfg_PSADEL-5_discr_10"),
        };

        let ipv4_lsp_identifiers_tlv = Ipv4LSPIndetifiersTLV {
            tlv_type: 18,
            tlv_len: 16,
            tunnel_sender_address: Ipv4Addr::new(10, 100, 0, 105),
            lsp_id: 0,
            tunnel_id: 174,
            extended_tunnel_id: 174325865,
            tunnel_endpoint_address: Ipv4Addr::new(10, 104, 105, 1),
        };

        let expected_lsp_object = LspObject {
            common_object: expected_cobj,
            plsp_id: 235,
            operational_status: OperationalStatus::Down,
            flag_administrative: true,
            flag_remove: false,
            flag_sync: false,
            flag_delegate: true,
            tlvs: Some(vec![
                Tlv::SymbolicPathName(symolic_pathname_tlv),
                Tlv::Ipv4LSPIndetifiers(ipv4_lsp_identifiers_tlv),
            ]),
        };

        assert!(remaining.is_empty());
        assert_eq!(lsp_object, expected_lsp_object);
    }
}
