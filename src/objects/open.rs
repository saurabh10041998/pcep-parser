use nom::bits;
use nom::bytes;
use nom::error::{Error, ErrorKind};
use nom::number;
use nom::sequence::tuple;
use nom::{Err, IResult};

use colored::Colorize;
use indoc::writedoc;

use crate::common::Version;
use crate::objects::classes::ObjectClassType;
use crate::objects::header::CommonObject;
use crate::objects::types::OpenObjectType;
use crate::tlvs::tlv_parser::Parser;
use crate::tlvs::types::Tlv;

#[derive(Debug, PartialEq, Eq)]
pub struct OpenObject {
    pub common_object: CommonObject,
    pub version: Version,
    pub flags: u8,
    pub keepalive: u8,
    pub deadtimer: u8,
    pub sid: u8,
    pub tlvs: Option<Vec<Tlv>>,
}

impl OpenObject {
    fn parse_ver_flags(input: &[u8]) -> IResult<&[u8], (u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(3u8),
            bits::streaming::take(5u8),
        )))(input)
    }

    pub fn parse_open_object(input: &[u8]) -> IResult<&[u8], OpenObject> {
        let (remaining, cobj) = CommonObject::parse_common_object(input)?;
        if let ObjectClassType::Open(OpenObjectType::Open) = cobj.object_class_type {
            let object_body_len = cobj.object_length - 4;
            let (remaining, object_body) =
                bytes::streaming::take(object_body_len as usize)(remaining)?;
            let (object_body, ver_flags) = Self::parse_ver_flags(object_body)?;
            let (object_body, keepalive) = number::streaming::be_u8(object_body)?;
            let (object_body, deadtimer) = number::streaming::be_u8(object_body)?;
            let (object_body, sid) = number::streaming::be_u8(object_body)?;
            let mut open_obj = OpenObject {
                common_object: cobj,
                version: ver_flags.0.into(),
                flags: ver_flags.1,
                keepalive,
                deadtimer,
                sid,
                tlvs: None,
            };
            if !object_body.is_empty() {
                // TLV section..
                let (object_body, tlvs) = Parser::parse_tlvs(object_body)?;
                // Did nom eat all the object ??
                assert!(object_body.is_empty());
                open_obj.tlvs = Some(tlvs);
            }
            return Ok((remaining, open_obj));
        }
        Err(Err::Failure(Error::new(input, ErrorKind::Fail)))
    }
}

impl std::fmt::Display for OpenObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tlvs_str = String::new();
        if let Some(ref tlvs) = self.tlvs {
            for t in tlvs {
                let output = format!("{}", t);
                tlvs_str.push_str(&output)
            }
        }
        let title = "==[Open Object]==".green().bold();
        writedoc!(
            f,
            r#"
            {title}
                {common_object}
                version                 = {version}
                flags                   = {flags}
                keepalive               = {keepalive}
                deadtimer               = {deadtimer}
                sid                     = {sid}

            {tlv_str}
            "#,
            title = title,
            common_object = self.common_object,
            version = self.version,
            flags = self.flags,
            keepalive = self.keepalive,
            deadtimer = self.deadtimer,
            sid = self.sid,
            tlv_str = tlvs_str
        )
    }
}
