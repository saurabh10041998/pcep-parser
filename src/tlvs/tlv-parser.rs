use nom::number;
use nom::{Err, IResult};

use crate::common::Utils;
use crate::tlvs::tlv_set::{
    Ipv4LSPIndetifiersTLV, SrPCECapabilityTLV, StatefulPCECapabilityTLV, UnknownTLV,
};
use crate::tlvs::types::Tlv;

use super::tlv_set::SymbolicPathNameTLV;

pub struct Parser;

impl Parser {
    fn handle_padding(len: usize, input: &[u8]) -> IResult<&[u8], &[u8]> {
        match Utils::consume_padding(len, input) {
            Ok((remaining, padding)) => Ok((remaining, padding)),
            Err(Err::Incomplete(_x)) => Ok((input, &[])),
            Err(e) => Err(e),
        }
    }
    pub fn parse_tlv(input: &[u8]) -> IResult<&[u8], Tlv> {
        let (remaining, tlv_type) = number::streaming::be_u16(input)?;
        match tlv_type.into() {
            Tlv::StatefulPCECapability(_) => {
                // parse StatefulPCETLV
                let (remaining, tlv) = StatefulPCECapabilityTLV::parse_tlv(remaining)?;
                let (remaining, _padding) = Self::handle_padding(tlv.tlv_len as usize, remaining)?;
                Ok((remaining, Tlv::StatefulPCECapability(tlv)))
            }
            Tlv::SrPCECapability(_) => {
                // parse SRPCECapabilityTLV
                let (remaining, tlv) = SrPCECapabilityTLV::parse_tlv(remaining)?;
                let (remaining, _padding) = Self::handle_padding(tlv.tlv_len as usize, remaining)?;
                Ok((remaining, Tlv::SrPCECapability(tlv)))
            }
            Tlv::SymbolicPathName(_) => {
                // parse SymbolicPathNameTLV
                let (remaining, tlv) = SymbolicPathNameTLV::parse_tlv(remaining)?;
                let (remaining, _padding) = Self::handle_padding(tlv.tlv_len as usize, remaining)?;
                Ok((remaining, Tlv::SymbolicPathName(tlv)))
            }
            Tlv::Ipv4LSPIndetifiers(_) => {
                let (remaining, tlv) = Ipv4LSPIndetifiersTLV::parse_tlv(remaining)?;
                let (remaining, _padding) = Self::handle_padding(tlv.tlv_len as usize, remaining)?;
                Ok((remaining, Tlv::Ipv4LSPIndetifiers(tlv)))
            }
            Tlv::Unknown(_) => {
                //Parse UnknownTlv
                let (remaining, mut tlv) = UnknownTLV::parse_tlv(remaining)?;
                tlv.tlv_type = tlv_type;
                let (remaining, _padding) = Self::handle_padding(tlv.tlv_len as usize, remaining)?;
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
