use nom::bytes;
use nom::IResult;

use super::header::SubObject;
use super::sr::SrSubobject;
use super::types::SubObjectTypes;

pub struct Parser;

impl Parser {
    fn parse_subobject(input: &[u8]) -> IResult<&[u8], SubObject> {
        let (remaining, mut subobject) = SubObject::parse_common_subobject(input)?;
        let subobject_body_len = subobject.subobject_len - 2;
        let (remaining, subobject_body) =
            bytes::streaming::take(subobject_body_len as usize)(remaining)?;
        let subobject_type = match subobject.subobject_type {
            SubObjectTypes::Ipv4Prefix => {
                unimplemented!()
            }
            SubObjectTypes::Ipv6Prefix => {
                unimplemented!()
            }
            SubObjectTypes::As => {
                unimplemented!()
            }
            SubObjectTypes::Sr(_) => {
                let (remaining, sr_subobject) = SrSubobject::parse_sr_subobject(subobject_body)?;
                assert!(
                    remaining.is_empty(),
                    "[!!] Sr subobject is not parsed fully.."
                );
                SubObjectTypes::Sr(sr_subobject)
            }
            SubObjectTypes::Unknown(_x) => {
                unimplemented!()
            }
        };
        subobject.subobject_type = subobject_type;
        Ok((remaining, subobject))
    }

    pub fn parse_subobjects(input: &[u8]) -> IResult<&[u8], Vec<SubObject>> {
        let mut left = input;
        let mut subobjects = vec![];
        while left.first().is_some() {
            match Self::parse_subobject(left) {
                Ok((remaining, subobject)) => {
                    subobjects.push(subobject);
                    left = remaining;
                }
                Err(e) => return Err(e),
            }
        }
        Ok((left, subobjects))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::subobjects::sr::{Ipv4AdjNAI, NaiType};
    use std::net::Ipv4Addr;
    #[test]
    fn test_subobject_parser() {
        let input: &[u8] = &[
            0x24, 0x10, 0x30, 0x01, 0x05, 0xdc, 0x30, 0x00, 0x0a, 0x68, 0x69, 0x02, 0x0a, 0x68,
            0x69, 0x01,
        ];
        let (remaining, subobjects) =
            Parser::parse_subobjects(input).expect("[!!] Error while parsing subobjects");
        let expected_sr_subobject = SrSubobject {
            flag_c: false,
            flag_f: false,
            flag_s: false,
            flag_m: true,
            sid: 98316288,
            nai_type: NaiType::Ipv4Adj(Ipv4AdjNAI {
                remote_ipv4: Ipv4Addr::new(10, 104, 105, 1),
                local_ipv4: Ipv4Addr::new(10, 104, 105, 2),
            }),
        };
        let expected_subobject = SubObject {
            flag_l: false,
            subobject_len: 16,
            subobject_type: SubObjectTypes::Sr(expected_sr_subobject),
        };

        let expected_subobjects = vec![expected_subobject];
        assert!(remaining.is_empty());
        assert_eq!(expected_subobjects, subobjects);
    }
}
