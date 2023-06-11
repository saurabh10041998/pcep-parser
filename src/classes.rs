use crate::types::OpenObjectType;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ObjectClassType {
    Open(OpenObjectType),
    Unknown,
}

impl From<(u8, u8)> for ObjectClassType {
    fn from(value: (u8, u8)) -> Self {
        let object_class = value.0;
        let object_type = value.1;
        match object_class {
            1 => Self::Open(object_type.into()),
            _ => Self::Unknown,
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
            _ => {
                write!(f, "Unknown class and type")
            }
        }
    }
}
