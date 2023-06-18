use crate::common::IntendedAttrList;
use crate::messages::header::CommonHeader;
use crate::objects::endpoints::EndPointsObject;
use crate::objects::ero::EroObject;
use crate::objects::lsp::LspObject;
use crate::objects::srp::SrpObject;

use colored::Colorize;
use indoc::writedoc;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct PCInitiate {
    pub common_header: CommonHeader,
    pce_initiated_lsp_lst: PceInitiatedLspList,
}

impl PCInitiate {
    pub fn parse_pcinitiate_message(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, pce_initiated_lsp_lst) =
            PceInitiatedLspList::parse_pce_initiated_lsp_list(input)?;
        let pc_initiate_message = PCInitiate {
            common_header: Default::default(),
            pce_initiated_lsp_lst,
        };
        Ok((remaining, pc_initiate_message))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PceInitiatedLspList {
    lsp_requests: Vec<PceInitiatedLspRequest>,
}

impl PceInitiatedLspList {
    pub fn parse_pce_initiated_lsp_list(input: &[u8]) -> IResult<&[u8], Self> {
        let mut lsp_requests = vec![];
        let mut left = input;
        while left.first().is_some() {
            match PceInitiatedLspRequest::parse_pce_initiated_lsp_request(left) {
                Ok((remaining, pce_init_lsp_req)) => {
                    left = remaining;
                    lsp_requests.push(pce_init_lsp_req);
                }
                Err(_e) => {
                    // For now breaking out of loop
                    // As no support for the VENDOROBJECT
                    // TODO: Handle errors gracefully
                    break;
                }
            }
        }
        let pce_init_lsp_list = PceInitiatedLspList { lsp_requests };
        Ok((left, pce_init_lsp_list))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PceInitiatedLspRequest {
    LspInstantiation(PceInitiateLspInstatiation),
    LspDeletion(PceInitiatedLspDeletion),
}

impl PceInitiatedLspRequest {
    pub fn parse_pce_initiated_lsp_request(input: &[u8]) -> IResult<&[u8], Self> {
        let (_input, srp_object) = SrpObject::parse_srp_object(input)?;
        match srp_object.flag_remove {
            true => {
                // Deletion request
                let (remaining, pce_init_lsp_deletion) =
                    PceInitiatedLspDeletion::parse_initiated_deletion(input)?;
                Ok((remaining, Self::LspDeletion(pce_init_lsp_deletion)))
            }
            false => {
                // Instantiation request
                let (remaining, pce_init_lsp_instantiation) =
                    PceInitiateLspInstatiation::parse_pce_initiated_lsp_instantiation(input)?;
                Ok((
                    remaining,
                    Self::LspInstantiation(pce_init_lsp_instantiation),
                ))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PceInitiateLspInstatiation {
    srp_object: SrpObject,
    lsp_object: LspObject,
    endpoints_object: Option<EndPointsObject>,
    ero_object: EroObject,
    attr_lst: Option<IntendedAttrList>,
}

impl PceInitiateLspInstatiation {
    fn parse_pce_initiated_lsp_instantiation(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, srp_object) = SrpObject::parse_srp_object(input)?;
        let (remaining, lsp_object) = LspObject::parse_lsp_object(remaining)?;
        let (remaining, endpoints_object) = match EndPointsObject::parse_endpoints_object(remaining)
        {
            Ok((remaining, endpoints_obj)) => (remaining, Some(endpoints_obj)),
            Err(_e) => (remaining, None),
        };
        let (remaining, ero_object) = EroObject::parse_ero_object(remaining)?;
        let (remaining, attr_lst) = match IntendedAttrList::parse_intended_attr_list(remaining) {
            Ok((remaining, attr_lst)) => (remaining, Some(attr_lst)),
            Err(_e) => (remaining, None),
        };
        let pce_init_lsp_instantiation = PceInitiateLspInstatiation {
            srp_object,
            lsp_object,
            endpoints_object,
            ero_object,
            attr_lst,
        };
        Ok((remaining, pce_init_lsp_instantiation))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PceInitiatedLspDeletion {
    srp_object: SrpObject,
    lsp_object: LspObject,
}

impl PceInitiatedLspDeletion {
    fn parse_initiated_deletion(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, srp_object) = SrpObject::parse_srp_object(input)?;
        let (remaining, lsp_object) = LspObject::parse_lsp_object(remaining)?;
        let pce_init_lsp_deletion = PceInitiatedLspDeletion {
            srp_object,
            lsp_object,
        };
        Ok((remaining, pce_init_lsp_deletion))
    }
}

// Display trait for all complex types
impl std::fmt::Display for PceInitiatedLspDeletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writedoc!(
            f,
            r#"
                {srp_object}
                {:indent$}{lsp_object}
            "#,
            "",
            srp_object = self.srp_object,
            lsp_object = self.lsp_object,
            indent = 4
        )
    }
}

impl std::fmt::Display for PceInitiateLspInstatiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end_points_obj_str = if let Some(ref endpoints_obj) = self.endpoints_object {
            format!("{}", endpoints_obj)
        } else {
            String::new()
        };
        let attr_lst_str = if let Some(ref attr_lst) = self.attr_lst {
            format!("{}", attr_lst)
        } else {
            String::new()
        };
        writedoc!(
            f,
            r#"
                {srp_object}
                {:indent$}{lsp_object}
                {:indent$}{end_points_obj_str}
                {:indent$}{ero_object}
                {:indent$}{attr_lst_str}
            "#,
            "",
            "",
            "",
            "",
            srp_object = self.srp_object,
            lsp_object = self.lsp_object,
            end_points_obj_str = end_points_obj_str,
            ero_object = self.ero_object,
            attr_lst_str = attr_lst_str,
            indent = 4
        )
    }
}

impl std::fmt::Display for PceInitiatedLspRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LspInstantiation(lsp_inst_req) => {
                write!(f, "{}", lsp_inst_req)
            }
            Self::LspDeletion(lsp_del_req) => {
                write!(f, "{}", lsp_del_req)
            }
        }
    }
}

impl std::fmt::Display for PceInitiatedLspList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lsp_requests_str = String::new();
        for lsp_req in self.lsp_requests.iter() {
            let lsp_req_str = format!("{}", lsp_req);
            lsp_requests_str.push_str(&lsp_req_str);
        }
        writedoc!(
            f,
            r#"
                {lsp_requests_str}
            "#,
            lsp_requests_str = lsp_requests_str
        )
    }
}

impl std::fmt::Display for PCInitiate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "##[PCInitiate]##".yellow();
        writedoc!(
            f,
            r#"
            {title}
                {common_header}
                {pce_initiated_lsp_lst}
            "#,
            common_header = self.common_header,
            pce_initiated_lsp_lst = self.pce_initiated_lsp_lst
        )
    }
}
