use colored::Colorize;
use indoc::writedoc;
use nom::bits;
use nom::bytes;
use nom::error::{Error, ErrorKind};
use nom::number;
use nom::sequence::tuple;
use nom::{Err, IResult};

use crate::objects::classes::ObjectClassType;
use crate::objects::header::CommonObject;
use crate::objects::types::LspaObjectType;
use crate::tlvs::tlv_parser::Parser;
use crate::tlvs::types::Tlv;

#[derive(Debug, PartialEq, Eq)]
pub struct LspaObject {
    common_object: CommonObject,
    exclude_any: u32,
    include_any: u32,
    include_all: u32,
    setup_priority: u8,
    holding_priority: u8,
    flag_local_protection: bool,
    reserved: u8,
    tlvs: Option<Vec<Tlv>>,
}

impl LspaObject {
    fn parse_res_flag_l(input: &[u8]) -> IResult<&[u8], (u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(7u8),
            bits::streaming::take(1u8),
        )))(input)
    }
    pub fn parse_lspa_object(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, cobj) = CommonObject::parse_common_object(input)?;
        if let ObjectClassType::Lspa(LspaObjectType::Lspa) = cobj.object_class_type {
            let object_body_len = cobj.object_length - 4;
            let (remaining, object_body) =
                bytes::streaming::take(object_body_len as usize)(remaining)?;
            let (object_body, exclude_any) = number::streaming::be_u32(object_body)?;
            let (object_body, include_any) = number::streaming::be_u32(object_body)?;
            let (object_body, include_all) = number::streaming::be_u32(object_body)?;
            let (object_body, setup_priority) = number::streaming::be_u8(object_body)?;
            let (object_body, holding_priority) = number::streaming::be_u8(object_body)?;
            let (object_body, res_flag_l) = Self::parse_res_flag_l(object_body)?;
            let (object_body, reserved) = number::streaming::be_u8(object_body)?;

            let mut lspa_object = LspaObject {
                common_object: cobj,
                exclude_any,
                include_any,
                include_all,
                setup_priority,
                holding_priority,
                flag_local_protection: res_flag_l.1 & 0b1 == 0b1,
                reserved,
                tlvs: None,
            };
            if !object_body.is_empty() {
                let (_object_body, tlvs) = Parser::parse_tlvs(object_body)?;
                lspa_object.tlvs = Some(tlvs);
            }
            return Ok((remaining, lspa_object));
        }
        Err(Err::Failure(Error::new(input, ErrorKind::Fail)))
    }
}

impl std::fmt::Display for LspaObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tlvs_str = String::new();
        if let Some(ref tlvs) = self.tlvs {
            for t in tlvs {
                let output = format!("{}", t);
                tlvs_str.push_str(&output)
            }
        }
        let title = "==[LSPA Object]==".green().bold();
        writedoc!(
            f,
            r#"
            {title}
                {common_object}
                exclude_any                 = {exclude_any}
                include_any                 = {include_any}
                include_all                 = {include_all}
                setup_priority              = {setup_priority}
                holding_priority            = {holding_priority}
                flag_local_protection       = {flag_local_protection}
                reserved                    = {reserved}
            {tlv_str}
            "#,
            title = title,
            common_object = self.common_object,
            exclude_any = self.exclude_any,
            include_any = self.include_any,
            include_all = self.include_all,
            setup_priority = self.setup_priority,
            holding_priority = self.holding_priority,
            flag_local_protection = self.flag_local_protection,
            reserved = self.reserved,
            tlv_str = tlvs_str
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_lspa_object_parsing() {
        let input: &[u8] = &[
            0x09, 0x10, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x07, 0x07, 0x01, 0x00,
        ];

        let (remaining, lspa_object) =
            LspaObject::parse_lspa_object(input).expect("[!!] Error while parsing lspa object");
        let expected_cobj = CommonObject {
            object_class_type: ObjectClassType::Lspa(LspaObjectType::Lspa),
            object_length: 20,
            flag_ignore: false,
            flag_process: false,
            reserved: 0,
        };
        let expected_lspa_object = LspaObject {
            common_object: expected_cobj,
            exclude_any: 0x00000000,
            include_any: 0x00000000,
            include_all: 0x00000000,
            setup_priority: 7,
            holding_priority: 7,
            flag_local_protection: true,
            reserved: 0,
            tlvs: None,
        };
        assert!(
            remaining.is_empty(),
            "[!!] Nope, nom did not eat all the object"
        );
        assert_eq!(lspa_object, expected_lspa_object);
    }
}
