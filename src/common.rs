use nom::bytes;
use nom::IResult;

pub struct Utils;

impl Utils {
    pub fn consume_padding(len: usize, input: &[u8]) -> IResult<&[u8], &[u8]> {
        match len % 4 {
            0 => Ok((input, &[])),
            1 => {
                // consume 3 bytes
                let (input, padding) = bytes::streaming::take(3usize)(input)?;
                Ok((input, padding))
            }
            2 => {
                // consume 2 bytes
                let (input, padding) = bytes::streaming::take(2usize)(input)?;
                Ok((input, padding))
            }
            3 => {
                // consume 1 byte
                let (input, padding) = bytes::streaming::take(1usize)(input)?;
                Ok((input, padding))
            }
            _ => unreachable!(),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Version {
    One,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::One => write!(f, "Version::one"),
        }
    }
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            1 => Version::One,
            _ => unimplemented!("Unknown version: {}", value),
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::One
    }
}

#[non_exhaustive]
// Operational Status for LSP
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationalStatus {
    Down,
    Up,
    Active,
    GoingDown,
    GoingUp,
    Reserved,
}

impl From<u8> for OperationalStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Down,
            1 => Self::Up,
            2 => Self::Active,
            3 => Self::GoingDown,
            4 => Self::GoingUp,
            5..=7 => Self::Reserved,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for OperationalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Down => write!(f, "OperationalStatus::DOWN"),
            Self::Up => write!(f, "OperationalStatus::UP"),
            Self::Active => write!(f, "OperationalStatus::ACTIVE"),
            Self::GoingDown => write!(f, "OperationalStatus::GOING-DOWN"),
            Self::GoingUp => write!(f, "OperationalStatus::GOING-UP"),
            Self::Reserved => write!(f, "OperationalStatus::Reserved"),
        }
    }
}

use crate::objects::bandwidth::BandwidthObject;
use crate::objects::lspa::LspaObject;
use crate::objects::metric::MetricObject;
use indoc::writedoc;

// Attribute List Entity
#[derive(Debug, PartialEq, Eq)]
pub struct IntendedAttrList {
    lspa_object: Option<LspaObject>,
    bandwidth_object: Option<BandwidthObject>,
    metric_list: Option<Vec<MetricObject>>,
}
impl IntendedAttrList {
    pub fn parse_intended_attr_list(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, lspa_object) = match LspaObject::parse_lspa_object(input) {
            Ok((remaining, lspa_object)) => (remaining, Some(lspa_object)),
            Err(_e) => (input, None),
        };
        let (input, bandwidth_object) = match BandwidthObject::parse_bandwidth_object(input) {
            Ok((remaining, bandwidth_object)) => (remaining, Some(bandwidth_object)),
            Err(_e) => (input, None),
        };
        let mut left = input;
        let mut metric_objects = vec![];
        while left.first().is_some() {
            match MetricObject::parse_metric_object(left) {
                Ok((remaining, metric_object)) => {
                    left = remaining;
                    metric_objects.push(metric_object);
                }
                Err(_e) => {
                    break;
                }
            }
        }
        let attr_lst = IntendedAttrList {
            lspa_object,
            bandwidth_object,
            metric_list: if metric_objects.is_empty() {
                None
            } else {
                Some(metric_objects)
            },
        };
        Ok((left, attr_lst))
    }
}

impl std::fmt::Display for IntendedAttrList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut attr_lst = String::new();
        if let Some(ref lspa_object) = self.lspa_object {
            let lspa_obj_str = format!("{}", lspa_object);
            attr_lst.push_str(&lspa_obj_str);
        }
        if let Some(ref bandwidth_obj) = self.bandwidth_object {
            let bandwidth_obj_str = format!("{}", bandwidth_obj);
            attr_lst.push_str(&bandwidth_obj_str);
        }
        if let Some(ref metric_lst) = self.metric_list {
            let mut metric_lst_str = String::new();
            for metric in metric_lst {
                let metric_str = format!("{:indent$}{}", "", metric, indent = 4);
                metric_lst_str.push_str(&metric_str);
            }
            attr_lst.push_str(&metric_lst_str);
        }
        writedoc!(f, "{}", attr_lst)
    }
}
