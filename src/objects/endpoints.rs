use crate::objects::classes::ObjectClassType;
use crate::objects::header::CommonObject;
use crate::objects::types::EndpointsObjectType;
use colored::Colorize;
use indoc::writedoc;
use nom::bytes;
use nom::combinator::map_res;
use nom::error::{Error, ErrorKind};
use nom::{Err, IResult};

use std::net::Ipv4Addr;

#[derive(Debug, PartialEq, Eq)]
pub struct EndPointsObject {
    common_object: CommonObject,
    end_points: EndPoints,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EndPoints {
    Ipv4Addresses(Ipv4AddressesEndPoint),
}

impl std::fmt::Display for EndPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ipv4Addresses(ipv4_addresses_ep) => {
                writedoc!(
                    f,
                    r#"
                    Endpoints Type           = Ipv4 Addresses(1)
                    {ipv4_addresses_ep} 
                    "#,
                    ipv4_addresses_ep = ipv4_addresses_ep
                )
            }
        }
    }
}

impl EndPointsObject {
    pub fn parse_endpoints_object(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, common_obj) = CommonObject::parse_common_object(input)?;
        if let ObjectClassType::EndPoints(EndpointsObjectType::Ipv4Addresses) =
            common_obj.object_class_type
        {
            let object_body_len = common_obj.object_length - 4;
            let (remaining, object_body) =
                bytes::streaming::take(object_body_len as usize)(remaining)?;
            let (_object_body, ipv4_address_endpoints) =
                Ipv4AddressesEndPoint::parse_ipv4_addresses_endpoint(object_body)?;
            let endpoints_object = EndPointsObject {
                common_object: common_obj,
                end_points: EndPoints::Ipv4Addresses(ipv4_address_endpoints),
            };
            return Ok((remaining, endpoints_object));
        }
        Err(Err::Failure(Error::new(input, ErrorKind::Fail)))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ipv4AddressesEndPoint {
    source_ipv4: Ipv4Addr,
    destination_ipv4: Ipv4Addr,
}

impl Ipv4AddressesEndPoint {
    pub fn parse_ipv4_addresses_endpoint(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, source_ipv4_octets): (&[u8], [u8; 4]) =
            map_res(bytes::streaming::take(4usize), |f: &[u8]| f.try_into())(input)?;

        let (input, destination_ipv4_octet): (&[u8], [u8; 4]) =
            map_res(bytes::streaming::take(4usize), |f: &[u8]| f.try_into())(input)?;

        let ipv4_addresses_endpoints = Ipv4AddressesEndPoint {
            source_ipv4: source_ipv4_octets.into(),
            destination_ipv4: destination_ipv4_octet.into(),
        };
        Ok((input, ipv4_addresses_endpoints))
    }
}

impl std::fmt::Display for Ipv4AddressesEndPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            r#"
                {:indent$}source Ipv4 address      = {source_ipv4}
                {:indent$}destination Ipv4 address = {destination_ipv4}
            "#,
            "",
            "",
            source_ipv4 = self.source_ipv4,
            destination_ipv4 = self.destination_ipv4,
            indent = 4
        )
    }
}

impl std::fmt::Display for EndPointsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "==[ENDPOINTS Object]==".green().bold();
        writedoc!(
            f,
            r#"
                {title}
                    {common_object}
                    {endpoints}
            "#,
            title = title,
            common_object = self.common_object,
            endpoints = self.end_points
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_endpoints_object_parsing() {
        let input: &[u8] = &[
            0x04, 0x10, 0x00, 0x0c, 0x0a, 0x64, 0x00, 0x69, 0x0a, 0x64, 0x00, 0x68,
        ];
        let (remaining, endpoints_object) = EndPointsObject::parse_endpoints_object(input)
            .expect("[!!] Error occured while parsing endpoints object");
        let expected_common_obj = CommonObject {
            object_class_type: ObjectClassType::EndPoints(EndpointsObjectType::Ipv4Addresses),
            flag_ignore: false,
            flag_process: false,
            reserved: 0,
            object_length: 12,
        };
        let expected_ipv4_addr_endpoints = Ipv4AddressesEndPoint {
            source_ipv4: Ipv4Addr::new(10, 100, 0, 105),
            destination_ipv4: Ipv4Addr::new(10, 100, 0, 104),
        };
        let expected_endpoint_object = EndPointsObject {
            common_object: expected_common_obj,
            end_points: EndPoints::Ipv4Addresses(expected_ipv4_addr_endpoints),
        };
        assert!(
            remaining.is_empty(),
            "[!!] Nope, Nom did not eat all ENDPOINTS OBJECT"
        );
        assert_eq!(endpoints_object, expected_endpoint_object);
    }
}
