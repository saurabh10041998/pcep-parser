use colored::Colorize;
use indoc::writedoc;
use nom::bits;
use nom::error::{Error, ErrorKind};
use nom::number;
use nom::sequence::tuple;
use nom::{Err, IResult};

use crate::classes::ObjectClassType;
use crate::common::Version;
use crate::tlvs::{SrPCECapabilityTLV, StatefulPCECapabilityTLV, TLV};
use crate::types::OpenObjectType;

#[derive(Debug, PartialEq, Eq)]
pub struct CommonObject {
    pub object_class_type: ObjectClassType,
    pub reserved: u8,
    pub flag_process: bool,
    pub flag_ignore: bool,
    pub object_length: u16,
}

impl CommonObject {
    fn parse_typ_res_p_i(input: &[u8]) -> IResult<&[u8], (u8, u8, u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(4u8),
            bits::streaming::take(2u8),
            bits::streaming::take(1u8),
            bits::streaming::take(1u8),
        )))(input)
    }

    fn parse_common_object(input: &[u8]) -> IResult<&[u8], CommonObject> {
        let (input, object_class) = number::streaming::be_u8(input)?;
        let (input, typ_res_p_i) = Self::parse_typ_res_p_i(input)?;
        let (input, object_length) = number::streaming::be_u16(input)?;

        let object_class = object_class;
        let object_type = typ_res_p_i.0;

        let object_class_type: ObjectClassType = (object_class, object_type).into();

        let cobj = CommonObject {
            object_class_type,
            reserved: typ_res_p_i.1,
            flag_process: typ_res_p_i.2 & 0b1 == 0b1,
            flag_ignore: typ_res_p_i.3 & 0b1 == 0b1,
            object_length,
        };
        Ok((input, cobj))
    }
}

impl std::fmt::Display for CommonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "[[common object]]".bold();
        writedoc!(
            f,
            r#"
            {title}
                (obj_class, obj_type) = {object_class_type}
                reserved              = {reserved}
                flag_process          = {flag_process}
                flag_ignore           = {flag_ignore}
                object_length         = {object_length}
            "#,
            title = title,
            object_class_type = self.object_class_type,
            reserved = self.reserved,
            flag_process = self.flag_process,
            flag_ignore = self.flag_ignore,
            object_length = self.object_length
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OpenObject {
    pub common_object: CommonObject,
    pub version: Version,
    pub flags: u8,
    pub keepalive: u8,
    pub deadtimer: u8,
    pub sid: u8,
    pub tlvs: Option<Vec<TLV>>,
}

impl OpenObject {
    fn parse_ver_flags(input: &[u8]) -> IResult<&[u8], (u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(3u8),
            bits::streaming::take(5u8),
        )))(input)
    }

    fn parse_open_object_tlv(input: &[u8]) -> IResult<&[u8], TLV> {
        let (remaining, tlv_type) = number::streaming::be_u16(input)?;
        match tlv_type.into() {
            TLV::StatefulPCECapability(_) => {
                // parse StatefulPCETLV
                let (remaining, tlv) = StatefulPCECapabilityTLV::parse_tlv(remaining)?;
                Ok((remaining, TLV::StatefulPCECapability(tlv)))
            }
            TLV::SrPCECapability(_) => {
                //parse SRPCECapabilityTLV
                let (remaining, tlv) = SrPCECapabilityTLV::parse_tlv(remaining)?;
                Ok((remaining, TLV::SrPCECapability(tlv)))
            }
            TLV::Unknown(val) => Ok((remaining, TLV::Unknown(val))),
        }
    }

    fn parse_open_object_tlvs(input: &[u8]) -> IResult<&[u8], Vec<TLV>> {
        let mut left = input;
        let mut tlvs = vec![];
        while let Some(_) = left.first() {
            match Self::parse_open_object_tlv(left) {
                Ok((remaining, tlv)) => {
                    if let TLV::Unknown(_) = tlv {
                        //TODO : skip to the next tlv if this tlv is not parsable.
                        break;
                    }
                    tlvs.push(tlv);
                    left = remaining;
                }
                Err(e) => return Err(e),
            }
        }
        Ok((left, tlvs))
    }

    pub fn parse_open_object(input: &[u8]) -> IResult<&[u8], OpenObject> {
        let (remaining, cobj) = CommonObject::parse_common_object(input)?;
        if let ObjectClassType::Open(OpenObjectType::Open) = cobj.object_class_type {
            let (remaining, ver_flags) = Self::parse_ver_flags(remaining)?;
            let (remaining, keepalive) = number::streaming::be_u8(remaining)?;
            let (remaining, deadtimer) = number::streaming::be_u8(remaining)?;
            let (remaining, sid) = number::streaming::be_u8(remaining)?;
            let mut open_obj = OpenObject {
                common_object: cobj,
                version: ver_flags.0.into(),
                flags: ver_flags.1,
                keepalive,
                deadtimer,
                sid,
                tlvs: None,
            };
            if remaining.len() > 0 {
                // TLV section..
                let (remaining, tlvs) = Self::parse_open_object_tlvs(remaining)?;
                open_obj.tlvs = Some(tlvs);
                return Ok((remaining, open_obj));
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
