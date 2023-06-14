use nom::IResult;

use crate::messages::header::CommonHeader;
use crate::objects::srp::SrpObject;
#[derive(Debug, PartialEq, Eq)]
pub struct PcepUpdate {
    common_header: CommonHeader,
    update_request: Vec<UpdateRequest>,
}

impl PcepUpdate {
    fn parse_update_requests(input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }

    pub fn parse_update_message(input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UpdateRequest {
    Srp: SrpObject,
}
