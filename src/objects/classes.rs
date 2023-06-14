use crate::objects::types::LspObjectType;
use crate::objects::types::OpenObjectType;
use crate::objects::types::SrpObjectType;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ObjectClassType {
    Open(OpenObjectType),
    LSP(LspObjectType),
    SRP(SrpObjectType),
    Unknown((u8, u8)),
}

impl From<(u8, u8)> for ObjectClassType {
    fn from(value: (u8, u8)) -> Self {
        let object_class = value.0;
        let object_type = value.1;
        match object_class {
            1 => Self::Open(object_type.into()),
            32 => Self::LSP(object_type.into()),
            33 => Self::SRP(object_type.into()),
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
            Self::LSP(lsp_obj_type) => match lsp_obj_type {
                LspObjectType::Reserved => {
                    write!(f, "ObjectClassType::LSP, LSPObjectType::Reserved")
                }
                LspObjectType::Lsp => {
                    write!(f, "ObjectClassType::LSP, LSPObjectType::LSP")
                }
                LspObjectType::UnAssigned => {
                    write!(f, "ObjectClassType::LSP, LSPObjectType::UnAssigned")
                }
            },
            Self::SRP(srp_obj_type) => match srp_obj_type {
                SrpObjectType::Reserved => {
                    write!(f, "ObjectClassType::SRP, SRPObjectType::Reserved")
                }
                SrpObjectType::Srp => {
                    write!(f, "ObjectClassType::SRP, SRPObjectType::SRP")
                }
                SrpObjectType::UnAssigned => {
                    write!(f, "ObjectClassType::SRP, SRPObjectType::UnAssigned")
                }
            },
            Self::Unknown(x) => {
                write!(f, "[!!] Unknown class and type: {:?}", *x)
            }
        }
    }
}
