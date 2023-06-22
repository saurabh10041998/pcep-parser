use colored::Colorize;
use indoc::writedoc;
use nom::bytes;
use nom::error::{Error, ErrorKind};
use nom::{Err, IResult};

use crate::objects::classes::ObjectClassType;
use crate::objects::header::CommonObject;
use crate::objects::types::EroObjectType;
use crate::subobjects::header::SubObject;
use crate::subobjects::parser::Parser;

#[derive(Debug, PartialEq, Eq)]
pub struct EroObject {
    pub common_object: CommonObject,
    pub subobjects: Option<Vec<SubObject>>,
}

impl EroObject {
    pub fn parse_ero_object(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, common_object) = CommonObject::parse_common_object(input)?;
        if let ObjectClassType::Ero(EroObjectType::Ero) = common_object.object_class_type {
            let object_body_len = common_object.object_length - 4;
            let (remaining, object_body) =
                bytes::streaming::take(object_body_len as usize)(remaining)?;
            let mut ero_object = EroObject {
                common_object,
                subobjects: None,
            };
            if !object_body.is_empty() {
                let (_remaining, subobjects) = Parser::parse_subobjects(object_body)?;
                ero_object.subobjects = Some(subobjects);
            }
            return Ok((remaining, ero_object));
        }
        Err(Err::Failure(Error::new(input, ErrorKind::Fail)))
    }
}

impl std::fmt::Display for EroObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut subobjects_str = String::new();
        if let Some(ref subobjects) = self.subobjects {
            for subobject in subobjects {
                let subobject_str = format!("{:indent$}{}", "", subobject, indent = 4);
                subobjects_str.push_str(&subobject_str);
            }
        }

        let title = "==[ERO Object]==".green().bold();
        writedoc!(
            f,
            r#"
                {title}
                     {common_object}
                {subobjects_str}
            "#,
            common_object = self.common_object,
            subobjects_str = subobjects_str
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::subobjects::sr::{Ipv4AdjNAI, NaiType, SrSubobject};
    use crate::subobjects::types::SubObjectTypes;
    use std::net::Ipv4Addr;
    #[test]
    fn test_ero_object_parsing() {
        let input: &[u8] = &[
            0x07, 0x10, 0x00, 0x14, 0x24, 0x10, 0x30, 0x01, 0x05, 0xdc, 0x30, 0x00, 0x0a, 0x68,
            0x69, 0x02, 0x0a, 0x68, 0x69, 0x01,
        ];
        let (remaining, ero_object) =
            EroObject::parse_ero_object(input).expect("[!!] Error while parsing ero object");
        let expected_common_object = CommonObject {
            object_class_type: ObjectClassType::Ero(EroObjectType::Ero),
            reserved: 0,
            flag_ignore: false,
            flag_process: false,
            object_length: 20,
        };

        let expected_ero_object = EroObject {
            common_object: expected_common_object,
            subobjects: Some(vec![SubObject {
                flag_l: false,
                subobject_len: 16,
                subobject_type: SubObjectTypes::Sr(SrSubobject {
                    flag_c: false,
                    flag_f: false,
                    flag_s: false,
                    flag_m: true,
                    sid: 98316288,
                    nai_type: NaiType::Ipv4Adj(Ipv4AdjNAI {
                        remote_ipv4: Ipv4Addr::new(10, 104, 105, 1),
                        local_ipv4: Ipv4Addr::new(10, 104, 105, 2),
                    }),
                }),
            }]),
        };
        assert!(remaining.is_empty());
        assert_eq!(expected_ero_object, ero_object);
    }
}
