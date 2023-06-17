use colored::Colorize;
use indoc::writedoc;
use nom::bits;
use nom::error::Error;
use nom::number;
use nom::sequence::tuple;
use nom::IResult;

use super::types::SubObjectTypes;

#[derive(Debug, PartialEq, Eq)]
pub struct SubObject {
    pub flag_l: bool,
    pub subobject_type: SubObjectTypes,
    pub subobject_len: u8,
}

impl SubObject {
    fn parse_l_subobj(input: &[u8]) -> IResult<&[u8], (u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(1u8),
            bits::streaming::take(7u8),
        )))(input)
    }

    pub fn parse_common_subobject(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, l_subobj) = Self::parse_l_subobj(input)?;
        let (remaining, subobject_len) = number::streaming::be_u8(remaining)?;
        let common_subobj = SubObject {
            flag_l: l_subobj.0 & 0b1 == 0b1,
            subobject_type: l_subobj.1.into(),
            subobject_len,
        };
        Ok((remaining, common_subobj))
    }
}

impl std::fmt::Display for SubObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "==[Subobject]==".green().bold();
        writedoc!(
            f,
            r#"
            {title}
                flag_l              =  {flag_l}
                subobject_len       = {subobject_len}
                
                {subobject_type}               
            "#,
            title = title,
            flag_l = self.flag_l,
            subobject_type = self.subobject_type,
            subobject_len = self.subobject_len
        )
    }
}
