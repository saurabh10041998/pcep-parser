use crate::objects::types::BandwidthObjectType;
use crate::objects::types::EndpointsObjectType;
use crate::objects::types::EroObjectType;
use crate::objects::types::LspObjectType;
use crate::objects::types::LspaObjectType;
use crate::objects::types::MetricObjectType;
use crate::objects::types::OpenObjectType;
use crate::objects::types::SrpObjectType;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ObjectClassType {
    Open(OpenObjectType),
    Lsp(LspObjectType),
    Lspa(LspaObjectType),
    Srp(SrpObjectType),
    Bandwidth(BandwidthObjectType),
    Metric(MetricObjectType),
    Ero(EroObjectType),
    EndPoints(EndpointsObjectType),
    Unknown((u8, u8)),
}

impl From<(u8, u8)> for ObjectClassType {
    fn from(value: (u8, u8)) -> Self {
        let object_class = value.0;
        let object_type = value.1;
        match object_class {
            1 => Self::Open(object_type.into()),
            4 => Self::EndPoints(object_type.into()),
            5 => Self::Bandwidth(object_type.into()),
            6 => Self::Metric(object_type.into()),
            7 => Self::Ero(object_type.into()),
            9 => Self::Lspa(object_type.into()),
            32 => Self::Lsp(object_type.into()),
            33 => Self::Srp(object_type.into()),
            _ => Self::Unknown((object_class, object_type)),
        }
    }
}

impl std::fmt::Display for ObjectClassType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open(open_obj_type) => match open_obj_type {
                OpenObjectType::Open => {
                    write!(f, "(ObjectClassType::Open, OpenObjectType::Open)")
                }
                OpenObjectType::Reserved => {
                    write!(f, "(ObjectClassType::Open, OpenObjectType::Reserved)",)
                }
                OpenObjectType::UnAssigned => {
                    write!(f, "(ObjectClassType::Open, OpenObjectType::UnAssigned)")
                }
            },
            Self::EndPoints(endpoints_obj_type) => match endpoints_obj_type {
                EndpointsObjectType::Reserved => {
                    write!(
                        f,
                        "(ObjectClassType::EndPoints, EndpointsObjectType::Reserved)"
                    )
                }
                EndpointsObjectType::Ipv4Addresses => {
                    write!(
                        f,
                        "(ObjectClassType::EndPoints, EndpointsObjectType::Ipv4Addresses)"
                    )
                }
                EndpointsObjectType::Ipv6Addresses => {
                    write!(
                        f,
                        "(ObjectClassType::EndPoints, EndpointsObjectType::Ipv6Addresses)"
                    )
                }
                EndpointsObjectType::Ipv4 => {
                    write!(f, "(ObjectClassType::EndPoints, EndpointsObjectType::Ipv4)")
                }
                EndpointsObjectType::Ipv6 => {
                    write!(f, "(ObjectClassType::EndPoints, EndpointsObjectType::Ipv6)")
                }
                EndpointsObjectType::Generalized => {
                    write!(
                        f,
                        "(ObjectClassType::EndPoints, EndpointsObjectType::Generalized)"
                    )
                }
                EndpointsObjectType::Unassigned => {
                    write!(
                        f,
                        "(ObjectClassType::EndPoints, EndpointsObjectType::Unassigned)"
                    )
                }
            },
            Self::Bandwidth(bandwidth_obj_type) => match bandwidth_obj_type {
                BandwidthObjectType::Reserved => {
                    write!(
                        f,
                        "(ObjectClassType::Bandwidth, BandwidthObjectType::Reserved)"
                    )
                }
                BandwidthObjectType::Requested => {
                    write!(
                        f,
                        "(ObjectClassType::Bandwidth, BandwidthObjectType::REQUESTED)"
                    )
                }
                BandwidthObjectType::RequestedOpt => {
                    write!(
                        f,
                        "(ObjectClassType::Bandwidth, BandwidthObjectType::REQUESTED_OPT)"
                    )
                }
                BandwidthObjectType::Genric => {
                    write!(
                        f,
                        "(ObjectClassType::Bandwidth, BandwidthObjectType::GENERIC)"
                    )
                }
                BandwidthObjectType::GenericOpt => {
                    write!(
                        f,
                        "(ObjectClassType::Bandwidth, BandwidthObjectType::GENERIC_OPT)"
                    )
                }
                BandwidthObjectType::UnAssigned => {
                    write!(
                        f,
                        "(ObjectClassType::Bandwidth, BandwidthObjectType::UnAssigned)"
                    )
                }
            },
            Self::Metric(metric_obj_type) => match metric_obj_type {
                MetricObjectType::Reserved => {
                    write!(f, "(ObjectClassType::Metric, LspaObjectType::Reserved)")
                }
                MetricObjectType::Metric => {
                    write!(f, "(ObjectClassType::Metric, LspaObjectType::Metric)")
                }
                MetricObjectType::Unassigned => {
                    write!(f, "(ObjectClassType::Metric, LspaObjectType::Unassigned)")
                }
            },
            Self::Ero(ero_obj_type) => match ero_obj_type {
                EroObjectType::Reserved => {
                    write!(f, "(ObjectClassType::Metric, LspaObjectType::Reserved)")
                }
                EroObjectType::Ero => {
                    write!(f, "(ObjectClassType::Metric, LspaObjectType::ERO)")
                }
                EroObjectType::Unassigned => {
                    write!(f, "(ObjectClassType::Metric, LspaObjectType::Unassigned)")
                }
            },
            Self::Lspa(lspa_obj_type) => match lspa_obj_type {
                LspaObjectType::Reserved => {
                    write!(f, "(ObjectClassType::Lspa, LspaObjectType::Reserved)")
                }
                LspaObjectType::Lspa => {
                    write!(f, "(ObjectClassType::Lspa, LspaObjectType::LSPA)")
                }
                LspaObjectType::Unassigned => {
                    write!(f, "(ObjectClassType::Lspa, LspaObjectType::Unassigned)")
                }
            },
            Self::Lsp(lsp_obj_type) => match lsp_obj_type {
                LspObjectType::Reserved => {
                    write!(f, "(ObjectClassType::LSP, LSPObjectType::Reserved)")
                }
                LspObjectType::Lsp => {
                    write!(f, "(ObjectClassType::LSP, LSPObjectType::LSP)")
                }
                LspObjectType::UnAssigned => {
                    write!(f, "(ObjectClassType::LSP, LSPObjectType::UnAssigned)")
                }
            },
            Self::Srp(srp_obj_type) => match srp_obj_type {
                SrpObjectType::Reserved => {
                    write!(f, "(ObjectClassType::SRP, SRPObjectType::Reserved")
                }
                SrpObjectType::Srp => {
                    write!(f, "(ObjectClassType::SRP, SRPObjectType::SRP)")
                }
                SrpObjectType::UnAssigned => {
                    write!(f, "(ObjectClassType::SRP, SRPObjectType::UnAssigned)")
                }
            },
            Self::Unknown(x) => {
                write!(f, "[!!] Unknown class and type: {:?}", *x)
            }
        }
    }
}
