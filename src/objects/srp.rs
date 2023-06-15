use colored::Colorize;
use indoc::writedoc;
use nom::bytes;
use nom::error::{Error, ErrorKind};
use nom::number;
use nom::{Err, IResult};

use crate::objects::header::CommonObject;
use crate::tlvs::tlv_parser::Parser;
use crate::tlvs::types::Tlv;

use super::classes::ObjectClassType;
use super::types::SrpObjectType;

#[derive(Debug, PartialEq, Eq)]
pub struct SrpObject {
    pub common_object: CommonObject,
    pub flags: u32,
    pub srp_id: u32,
    pub tlvs: Option<Vec<Tlv>>,
}

impl SrpObject {
    pub fn parse_srp_object(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, common_object) = CommonObject::parse_common_object(input)?;
        if let ObjectClassType::Srp(SrpObjectType::Srp) = common_object.object_class_type {
            let object_body_len = common_object.object_length - 4;
            let (remaining, object_body) =
                bytes::streaming::take(object_body_len as usize)(remaining)?;
            let (object_body, flags) = number::streaming::be_u32(object_body)?;
            let (object_body, srp_id) = number::streaming::be_u32(object_body)?;
            let mut srp_object = SrpObject {
                common_object,
                flags,
                srp_id,
                tlvs: None,
            };
            if !object_body.is_empty() {
                let (_object_body, tlvs) = Parser::parse_tlvs(object_body)?;
                srp_object.tlvs = Some(tlvs);
            }
            return Ok((remaining, srp_object));
        }
        Err(Err::Failure(Error::new(input, ErrorKind::Fail)))
    }
}

impl std::fmt::Display for SrpObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tlvs_str = String::new();
        if let Some(ref tlvs) = self.tlvs {
            for t in tlvs {
                let output = format!("{}", t);
                tlvs_str.push_str(&output)
            }
        }
        let title = "==[SRP Object]==".green().bold();
        writedoc!(
            f,
            r#"
            {title}
                {common_object}
                flags                  = {flags}
                srp_id                 = {srp_id}
            {tlv_str}
            "#,
            title = title,
            common_object = self.common_object,
            flags = self.flags,
            srp_id = self.srp_id,
            tlv_str = tlvs_str
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tlvs::tlv_set::UnknownTLV;
    use crate::tlvs::types::Tlv;
    #[test]
    fn test_srp_object_parsing() {
        let input: &[u8] = &[
            0x21, 0x10, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x1c,
            0x00, 0x04, 0x00, 0x00, 0x00, 0x01,
        ];
        let (remaining, srp_object) =
            SrpObject::parse_srp_object(input).expect("[!!] Failed to parse srp objects");
        assert!(remaining.is_empty(), "[!!] Nope, object not eaten fully");
        let expected_cobj = CommonObject {
            object_class_type: ObjectClassType::Srp(SrpObjectType::Srp),
            reserved: 0,
            flag_process: false,
            flag_ignore: false,
            object_length: 20,
        };
        // TODO : code to parse this TLVs
        let unknown_tlv = UnknownTLV {
            tlv_type: 28,
            tlv_len: 4,
            tlv_data: vec![0x00, 0x00, 0x00, 0x01],
        };
        let expected_srp_obj = SrpObject {
            common_object: expected_cobj,
            flags: 0,
            srp_id: 1,
            tlvs: Some(vec![Tlv::Unknown(unknown_tlv)]),
        };
        assert_eq!(srp_object, expected_srp_obj);
    }
}
