use colored::Colorize;
use indoc::writedoc;
use nom::bits;
use nom::bytes;
use nom::combinator::map_res;
use nom::error::Error;
use nom::number;
use nom::sequence::tuple;
use nom::IResult;
use std::net::Ipv4Addr;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Default)]
pub enum NaiType {
    #[default]
    Absent,
    Ipv4Adj(Ipv4AdjNAI),
}

impl TryFrom<u8> for NaiType {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Absent),
            3 => Ok(Self::Ipv4Adj(Default::default())),
            _ => Err(format!(
                "[!!] unknown nai type..{} please add nai struct def",
                value
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ipv4AdjNAI {
    pub local_ipv4: Ipv4Addr,
    pub remote_ipv4: Ipv4Addr,
}

impl Default for Ipv4AdjNAI {
    fn default() -> Self {
        Ipv4AdjNAI {
            local_ipv4: Ipv4Addr::new(127, 0, 0, 1),
            remote_ipv4: Ipv4Addr::new(127, 0, 0, 1),
        }
    }
}

impl Ipv4AdjNAI {
    fn parse_ipv4_adj_nai(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, local_ipv4): (&[u8], [u8; 4]) =
            map_res(bytes::streaming::take(4usize), |f: &[u8]| f.try_into())(input)?;
        let (remaining, remote_ipv4): (&[u8], [u8; 4]) =
            map_res(bytes::streaming::take(4usize), |f: &[u8]| f.try_into())(remaining)?;
        let ipv4_adj_nai = Ipv4AdjNAI {
            local_ipv4: local_ipv4.into(),
            remote_ipv4: remote_ipv4.into(),
        };
        return Ok((remaining, ipv4_adj_nai));
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SrSubobject {
    pub nai_type: NaiType,
    pub flag_f: bool,
    pub flag_s: bool,
    pub flag_c: bool,
    pub flag_m: bool,
    pub sid: u32,
}

impl SrSubobject {
    fn parse_nt_res_flags(input: &[u8]) -> IResult<&[u8], (u8, u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(4u8),
            bits::streaming::take(8u8),
            bits::streaming::take(4u8),
        )))(input)
    }
    pub fn parse_sr_subobject(subobject_body: &[u8]) -> IResult<&[u8], Self> {
        let (subobject_body, nt_res_flags) = Self::parse_nt_res_flags(subobject_body)?;
        let (subobject_body, sid) = number::streaming::be_u32(subobject_body)?;
        let mut sr_subobject = SrSubobject {
            nai_type: nt_res_flags
                .0
                .try_into()
                .expect("[!!] Error: Unknown NaiType"),
            flag_f: nt_res_flags.2 & 0b0000_1000 == 0b0000_1000,
            flag_s: nt_res_flags.2 & 0b0000_0100 == 0b0000_0100,
            flag_c: nt_res_flags.2 & 0b0000_0010 == 0b0000_0010,
            flag_m: nt_res_flags.2 & 0b0000_0001 == 0b0000_0001,
            sid,
        };
        match sr_subobject.nai_type {
            NaiType::Absent => Ok((subobject_body, sr_subobject)),
            NaiType::Ipv4Adj(_) => {
                let (subobject_body, ipv4_adj_nai) =
                    Ipv4AdjNAI::parse_ipv4_adj_nai(subobject_body)?;
                sr_subobject.nai_type = NaiType::Ipv4Adj(ipv4_adj_nai);
                Ok((subobject_body, sr_subobject))
            }
        }
    }
}

// Display formatter all complex types...
impl std::fmt::Display for Ipv4AdjNAI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "**[Ipv4 Adjacency NAI]**".bold();
        writedoc!(
            f,
            r#"
                {title}
                     remote ipv4 address        = {remove_ipv4}
                     local ipv4  address        = {local_ipv4}
            "#,
            title = title,
            remove_ipv4 = self.remote_ipv4,
            local_ipv4 = self.local_ipv4
        )
    }
}

impl std::fmt::Display for NaiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absent => write!(f, "NaiType::Absent"),
            Self::Ipv4Adj(ipv4_adj_nai) => {
                write!(f, "{}", ipv4_adj_nai)
            }
        }
    }
}

impl std::fmt::Display for SrSubobject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "--[SR Subobject]--".green().bold();
        writedoc!(
            f,
            r#"
                {title}
                     flag_f     = {flag_f}
                     flag_s     = {flag_s}
                     flag_c     = {flag_c}
                     flag_m     = {flag_m}
                     sid        = {sid}
                     
                     {nai_type}
            "#,
            flag_f = self.flag_f,
            flag_s = self.flag_s,
            flag_c = self.flag_c,
            flag_m = self.flag_m,
            sid = self.sid,
            nai_type = self.nai_type
        )
    }
}
