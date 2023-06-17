use crate::messages::header::CommonHeader;
use crate::objects::bandwidth::BandwidthObject;
use crate::objects::ero::EroObject;
use crate::objects::lsp::LspObject;
use crate::objects::lspa::LspaObject;
use crate::objects::metric::MetricObject;
use crate::objects::srp::SrpObject;

use colored::Colorize;
use indoc::writedoc;
use nom::IResult;
#[derive(Debug, PartialEq, Eq)]
pub struct PcepUpdate {
    pub common_header: CommonHeader,
    update_request_lst: UpdateRequestList,
}

impl PcepUpdate {
    pub fn parse_update_message(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, update_request_lst) = UpdateRequestList::parse_update_request_list(input)?;
        let pcep_update_msg = PcepUpdate {
            common_header: Default::default(),
            update_request_lst,
        };
        Ok((remaining, pcep_update_msg))
    }
}

impl std::fmt::Display for PcepUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "##[PCUpdate]##".yellow();
        writedoc!(
            f,
            r#"
            {title}
                {common_header}
                {update_request_lst}
            "#,
            title = title,
            common_header = self.common_header,
            update_request_lst = self.update_request_lst
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UpdateRequestList {
    update_request_lst: Vec<UpdateRequest>,
}

impl UpdateRequestList {
    fn parse_update_request_list(input: &[u8]) -> IResult<&[u8], Self> {
        // TODO: for loops
        let (remaining, update_req) = UpdateRequest::parse_update_request(input)?;
        let update_req_lst = UpdateRequestList {
            update_request_lst: vec![update_req],
        };
        Ok((remaining, update_req_lst))
    }
}

impl std::fmt::Display for UpdateRequestList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut update_request_lst_str = String::new();
        for update_request in self.update_request_lst.iter() {
            let update_request_str = format!("{}", update_request);
            update_request_lst_str.push_str(&update_request_str);
        }
        writedoc!(
            f,
            "{update_request_lst_str}",
            update_request_lst_str = update_request_lst_str
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UpdateRequest {
    srp_object: SrpObject,
    lsp_object: LspObject,
    path: Path,
}

impl UpdateRequest {
    fn parse_update_request(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, srp_object) = SrpObject::parse_srp_object(input)?;
        let (remaining, lsp_object) = LspObject::parse_lsp_object(remaining)?;
        let (remaining, path) = Path::parse_path(remaining)?;
        let update_request = UpdateRequest {
            srp_object,
            lsp_object,
            path,
        };
        Ok((remaining, update_request))
    }
}

impl std::fmt::Display for UpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            r#"
                {srp_object}
                {:indent$}{lsp_object}
                {:indent$}{path}
            "#,
            "",
            "",
            srp_object = self.srp_object,
            lsp_object = self.lsp_object,
            path = self.path,
            indent = 4
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    intended_path: EroObject,
    intended_attr_lst: IntendedAttrList,
}

impl Path {
    fn parse_path(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, ero_object) = EroObject::parse_ero_object(input)?;
        let (remaining, intended_attr_lst) = IntendedAttrList::parse_intended_attr_list(remaining)?;
        let path = Path {
            intended_path: ero_object,
            intended_attr_lst,
        };
        Ok((remaining, path))
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            r#"
                {intended_path}
                {:indent$}{intended_attr_lst}
            "#,
            "",
            intended_path = self.intended_path,
            intended_attr_lst = self.intended_attr_lst,
            indent = 4
        )
    }
}

// TODO: Add IRO object to specification
#[derive(Debug, PartialEq, Eq)]
pub struct IntendedAttrList {
    lspa_object: Option<LspaObject>,
    bandwidth_object: Option<BandwidthObject>,
    metric_list: Option<Vec<MetricObject>>,
}
impl IntendedAttrList {
    fn parse_intended_attr_list(input: &[u8]) -> IResult<&[u8], Self> {
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
                let metric_str = format!("{}", metric);
                metric_lst_str.push_str(&metric_str);
            }
            attr_lst.push_str(&metric_lst_str);
        }
        writedoc!(f, "{}", attr_lst)
    }
}
