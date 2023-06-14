use colored::Colorize;
use indoc::writedoc;
use nom::bits;
use nom::error::Error;
use nom::number;
use nom::sequence::tuple;
use nom::IResult;

use crate::objects::classes::ObjectClassType;

#[derive(Debug, PartialEq, Eq)]
pub struct CommonObject {
    pub object_class_type: ObjectClassType,
    pub reserved: u8,
    pub flag_process: bool,
    pub flag_ignore: bool,
    pub object_length: u16,
}

impl CommonObject {
    fn parse_typ_res_p_i(input: &[u8]) -> IResult<&[u8], (u8, u8, u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(4u8),
            bits::streaming::take(2u8),
            bits::streaming::take(1u8),
            bits::streaming::take(1u8),
        )))(input)
    }

    pub fn parse_common_object(input: &[u8]) -> IResult<&[u8], CommonObject> {
        let (input, object_class) = number::streaming::be_u8(input)?;
        let (input, typ_res_p_i) = Self::parse_typ_res_p_i(input)?;
        let (input, object_length) = number::streaming::be_u16(input)?;

        let object_class = object_class;
        let object_type = typ_res_p_i.0;

        let object_class_type: ObjectClassType = (object_class, object_type).into();

        let cobj = CommonObject {
            object_class_type,
            reserved: typ_res_p_i.1,
            flag_process: typ_res_p_i.2 & 0b1 == 0b1,
            flag_ignore: typ_res_p_i.3 & 0b1 == 0b1,
            object_length,
        };
        Ok((input, cobj))
    }
}

impl std::fmt::Display for CommonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "[[common object]]".bold();
        writedoc!(
            f,
            r#"
            {title}
                (obj_class, obj_type) = {object_class_type}
                reserved              = {reserved}
                flag_process          = {flag_process}
                flag_ignore           = {flag_ignore}
                object_length         = {object_length}
            "#,
            title = title,
            object_class_type = self.object_class_type,
            reserved = self.reserved,
            flag_process = self.flag_process,
            flag_ignore = self.flag_ignore,
            object_length = self.object_length
        )
    }
}
