use nom::number;
use nom::{Err, IResult};

use crate::tlvs::tlv_set::{SrPCECapabilityTLV, StatefulPCECapabilityTLV, UnknownTlv};
use crate::tlvs::types::Tlv;

pub struct Parser;

impl Parser {
    pub fn parse_tlv(input: &[u8]) -> IResult<&[u8], Tlv> {
        let (remaining, tlv_type) = number::streaming::be_u16(input)?;
        match tlv_type.into() {
            Tlv::StatefulPCECapability(_) => {
                // parse StatefulPCETLV
                let (remaining, tlv) = StatefulPCECapabilityTLV::parse_tlv(remaining)?;
                Ok((remaining, Tlv::StatefulPCECapability(tlv)))
            }
            Tlv::SrPCECapability(_) => {
                //parse SRPCECapabilityTLV
                let (remaining, tlv) = SrPCECapabilityTLV::parse_tlv(remaining)?;
                Ok((remaining, Tlv::SrPCECapability(tlv)))
            }
            Tlv::Unknown(_) => {
                //Parse UnknownTlv
                let (remaining, mut tlv) = UnknownTlv::parse_tlv(remaining)?;
                tlv.tlv_type = tlv_type;
                Ok((remaining, Tlv::Unknown(tlv)))
            }
        }
    }

    pub fn parse_tlvs(input: &[u8]) -> IResult<&[u8], Vec<Tlv>> {
        let mut left = input;
        let mut tlvs = vec![];
        while left.first().is_some() {
            match Self::parse_tlv(left) {
                Ok((remaining, tlv)) => {
                    tlvs.push(tlv);
                    left = remaining;
                }
                Err(Err::Incomplete(_x)) => break,
                Err(e) => return Err(e),
            }
        }
        Ok((&[], tlvs))
    }
}
