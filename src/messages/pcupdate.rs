use crate::common::IntendedAttrList;
use crate::messages::header::CommonHeader;
use crate::objects::ero::EroObject;
use crate::objects::lsp::LspObject;
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
