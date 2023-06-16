use colored::Colorize;
use indoc::writedoc;
use nom::bits;
use nom::bytes;
use nom::error::{Error, ErrorKind};
use nom::number;
use nom::sequence::tuple;
use nom::{Err, IResult};

use crate::objects::classes::ObjectClassType;
use crate::objects::header::CommonObject;
use crate::objects::types::MetricObjectType;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MetricType {
    Igp,
    Te,
    HopCount,
    SidDepth,
    PathDelay,
    Unknown(u8),
}

impl From<u8> for MetricType {
    fn from(value: u8) -> Self {
        match value {
            1 => MetricType::Igp,
            2 => MetricType::Te,
            3 => MetricType::HopCount,
            11 => MetricType::SidDepth,
            12 => MetricType::PathDelay,
            _ => MetricType::Unknown(value),
        }
    }
}

impl std::fmt::Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Igp => write!(f, "MetricType::IGP"),
            Self::Te => write!(f, "MetricType::TE"),
            Self::HopCount => write!(f, "MetricType::HopCount"),
            Self::SidDepth => write!(f, "MetricType::SID-Depth"),
            Self::PathDelay => write!(f, "MetricType::Path-Delay"),
            Self::Unknown(x) => write!(f, "Unknown{}", *x),
        }
    }
}

#[derive(Debug)]
pub struct MetricObject {
    common_object: CommonObject,
    reserved: u16,
    flag_compute: bool,
    flag_bound: bool,
    metric_type: MetricType,
    metric_value: f32,
}

impl Eq for MetricObject {}
impl PartialEq for MetricObject {
    fn eq(&self, other: &Self) -> bool {
        let f1 = self.common_object.eq(&other.common_object);
        let f2 = self.reserved.eq(&other.reserved);
        let f3 = self.flag_compute.eq(&other.flag_compute);
        let f4 = self.flag_bound.eq(&other.flag_bound);
        let f5 = self.metric_type.eq(&other.metric_type);
        // TODO: Use strong float comparsion heuristics..
        let f6 = (other.metric_value - self.metric_value) as u32 == 0;
        f1 && f2 && f3 && f4 && f5 && f6
    }
}

impl MetricObject {
    fn parse_res_flag_c_b(input: &[u8]) -> IResult<&[u8], (u8, u8, u8)> {
        bits::bits::<_, _, Error<_>, _, _>(tuple((
            bits::streaming::take(6u8),
            bits::streaming::take(1u8),
            bits::streaming::take(1u8),
        )))(input)
    }

    pub fn parse_metric_object(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, cobj) = CommonObject::parse_common_object(input)?;
        if let ObjectClassType::Metric(MetricObjectType::Metric) = cobj.object_class_type {
            let object_body_len = cobj.object_length - 4;
            let (remaining, object_body) =
                bytes::streaming::take(object_body_len as usize)(remaining)?;
            let (object_body, reserved) = number::streaming::be_u16(object_body)?;
            let (object_body, res_flag_c_b) = Self::parse_res_flag_c_b(object_body)?;
            let (object_body, metric_type) = number::streaming::be_u8(object_body)?;
            let (_object_body, metric_value) = number::streaming::be_f32(object_body)?;

            let metric_object = MetricObject {
                common_object: cobj,
                reserved,
                flag_compute: res_flag_c_b.0 & 0b1 == 0b1,
                flag_bound: res_flag_c_b.2 & 0b1 == 0b1,
                metric_type: metric_type.into(),
                metric_value,
            };
            return Ok((remaining, metric_object));
        }
        Err(Err::Failure(Error::new(input, ErrorKind::Fail)))
    }
}

impl std::fmt::Display for MetricObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "==[METRIC Object]==".green().bold();
        writedoc!(
            f,
            r#"
            {title}
                {common_object}
                reserved = {reserved}
                flag_compute = {flag_compute}
                flag_bound = {flag_bound}
                metric_type = {metric_type}
                metric_value = {metric_value}
            "#,
            title = title,
            common_object = self.common_object,
            reserved = self.reserved,
            flag_compute = self.flag_compute,
            flag_bound = self.flag_bound,
            metric_type = self.metric_type,
            metric_value = self.metric_value
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_metric_object_parsing() {
        let input: &[u8] = &[
            0x06, 0x10, 0x00, 0x0c, 0x00, 0x00, 0x01, 0x0b, 0x41, 0x20, 0x00, 0x00,
        ];
        let (remaining, metric_object) =
            MetricObject::parse_metric_object(input).expect("[!!] Error parsing the metric object");
        let expected_cobj = CommonObject {
            object_class_type: ObjectClassType::Metric(MetricObjectType::Metric),
            flag_ignore: false,
            flag_process: false,
            reserved: 0,
            object_length: 12,
        };
        let expected_metric_object = MetricObject {
            common_object: expected_cobj,
            reserved: 0,
            flag_bound: true,
            flag_compute: false,
            metric_type: MetricType::SidDepth,
            metric_value: 10 as f32,
        };
        assert!(remaining.is_empty());
        assert_eq!(expected_metric_object, metric_object);
    }
    #[test]
    fn test_path_delay_metric_object_parsing() {
        let input: &[u8] = &[
            0x06, 0x10, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00,
        ];
        let (remaining, metric_object) =
            MetricObject::parse_metric_object(input).expect("[!!] Error parsing the metric object");
        let expected_cobj = CommonObject {
            object_class_type: ObjectClassType::Metric(MetricObjectType::Metric),
            flag_ignore: false,
            flag_process: false,
            reserved: 0,
            object_length: 12,
        };
        let expected_metric_object = MetricObject {
            common_object: expected_cobj,
            reserved: 0,
            flag_bound: false,
            flag_compute: false,
            metric_type: MetricType::PathDelay,
            metric_value: 0 as f32,
        };
        assert!(remaining.is_empty());
        assert_eq!(expected_metric_object, metric_object);
    }
}
