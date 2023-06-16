use nom::bytes;
use nom::error::{Error, ErrorKind};
use nom::number;
use nom::{Err, IResult};

use crate::objects::classes::ObjectClassType;
use crate::objects::header::CommonObject;
use crate::objects::types::BandwidthObjectType;
#[derive(Debug)]
pub struct BandwidthObject {
    pub common_object: CommonObject,
    pub bandwidth: f32,
}

impl Eq for BandwidthObject {}
impl PartialEq for BandwidthObject {
    fn eq(&self, other: &Self) -> bool {
        let t1 = self.common_object.eq(&other.common_object);
        // TODO: Use strong float comparsion heuristics.
        let t2 = (other.bandwidth - self.bandwidth) as u32 == 0;
        return t1 && t2;
    }
}

impl BandwidthObject {
    pub fn parse_bandwidth_object(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, cobj) = CommonObject::parse_common_object(input)?;
        match cobj.object_class_type {
            ObjectClassType::Bandwidth(obj_type) => match obj_type {
                BandwidthObjectType::Requested | BandwidthObjectType::RequestedOpt => {
                    let object_body_len = cobj.object_length - 4;
                    let (remaining, object_body) =
                        bytes::streaming::take(object_body_len as usize)(remaining)?;
                    let (_object_body, bandwidth) = number::streaming::be_f32(object_body)?;
                    let bandwidth_obj = BandwidthObject {
                        bandwidth,
                        common_object: cobj,
                    };
                    Ok((remaining, bandwidth_obj))
                }
                _ => {
                    unimplemented!("[!!] Unimplementd for other types of bandwidth")
                }
            },
            _ => Err(Err::Error(Error::new(input, ErrorKind::Fail))),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_bandwidth_object_parsing() {
        let input: &[u8] = &[0x05, 0x10, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00];
        let (remaining, bandwidth_object) = BandwidthObject::parse_bandwidth_object(input)
            .expect("[!!] Error while parsing bandwidth object");
        let expected_cobj = CommonObject {
            object_class_type: ObjectClassType::Bandwidth(BandwidthObjectType::Requested),
            flag_ignore: false,
            flag_process: false,
            reserved: 0,
            object_length: 8,
        };
        let expected_bandwidth_object = BandwidthObject {
            common_object: expected_cobj,
            bandwidth: 0 as f32,
        };
        assert!(
            remaining.is_empty(),
            "[!!] Nope, nom did not eat all the object"
        );
        assert_eq!(expected_bandwidth_object, bandwidth_object);
    }
}
