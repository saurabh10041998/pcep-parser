use nom::number;
use nom::IResult;

use crate::objects::header::CommonObject;

#[derive(Debug, PartialEq, Eq)]
pub struct SrpObject {
    pub common_object: CommonObject,
    pub flags: u32,
    pub srp_id: u32,
}

impl SrpObject {
    pub fn _parse_srp_object(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, common_object) = CommonObject::parse_common_object(input)?;
        let (remaining, flags) = number::streaming::be_u32(input)?;
        let (remaining, srp_id) = number::streaming::be_u32(remaining)?;

        let srp_obj = SrpObject {
            common_object,
            flags,
            srp_id,
        };
        Ok((remaining, srp_obj))
    }
}
