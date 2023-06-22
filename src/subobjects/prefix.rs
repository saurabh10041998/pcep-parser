use colored::Colorize;
use indoc::writedoc;
use nom::bytes;
use nom::combinator::map_res;
use nom::number;
use nom::IResult;

use std::net::Ipv4Addr;
#[derive(Debug, PartialEq, Eq)]
pub struct Ipv4PrefixSubobject {
    pub ipv4_addr: Ipv4Addr,
    pub pref_len: u8,
    pub reserved: u8,
}

impl Default for Ipv4PrefixSubobject {
    fn default() -> Self {
        Ipv4PrefixSubobject {
            ipv4_addr: Ipv4Addr::new(127, 0, 0, 1),
            pref_len: 32,
            reserved: 0,
        }
    }
}

impl Ipv4PrefixSubobject {
    pub fn parse_ipv4_pref_subobject(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, ipv4_addr_octet): (&[u8], [u8; 4]) =
            map_res(bytes::streaming::take(4usize), |f: &[u8]| f.try_into())(input)?;
        let (remaining, prefix) = number::streaming::be_u8(remaining)?;
        let (remaining, reserved) = number::streaming::be_u8(remaining)?;
        let ipv4_pref_subobject = Ipv4PrefixSubobject {
            ipv4_addr: ipv4_addr_octet.into(),
            pref_len: prefix,
            reserved,
        };
        Ok((remaining, ipv4_pref_subobject))
    }
}

impl std::fmt::Display for Ipv4PrefixSubobject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "--[IPV4 PREFIX Subobject]--".green().bold();
        writedoc!(
            f,
            r#"
                {title}
                     ipv4_address  = {ipv4_addr}
                     prefix_length = {pref_len}
            "#,
            ipv4_addr = self.ipv4_addr,
            pref_len = self.pref_len
        )
    }
}
