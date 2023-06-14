use crate::tlvs::tlv_set::{SrPCECapabilityTLV, StatefulPCECapabilityTLV, UnknownTlv};
use colored::Colorize;
use indoc::writedoc;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum Tlv {
    StatefulPCECapability(StatefulPCECapabilityTLV),
    SrPCECapability(SrPCECapabilityTLV),
    Unknown(UnknownTlv),
}

impl From<u16> for Tlv {
    fn from(value: u16) -> Self {
        match value {
            16 => Self::StatefulPCECapability(Default::default()),
            26 => Self::SrPCECapability(Default::default()),
            _ => Self::Unknown(Default::default()),
        }
    }
}

impl std::fmt::Display for Tlv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StatefulPCECapability(spc) => {
                let title = "==[STATEFUL-PCE-CAPABILITY TLV]==".green().bold();
                writedoc!(
                    f,
                    r#"
                    {:indent$}{title}
                        {tlv}
                    "#,
                    "",
                    title = title,
                    tlv = spc,
                    indent = 4
                )
            }
            Self::SrPCECapability(srpc) => {
                let title = "==[SR-PCE-CAPABILITY TLV]==".green().bold();
                writedoc!(
                    f,
                    r#"
                    {:indent$}{title}
                        {tlv}
                    "#,
                    "",
                    title = title,
                    tlv = srpc,
                    indent = 4
                )
            }
            Self::Unknown(x) => {
                let title = "==[UNKNOWN TLV]==".green().bold();
                writedoc!(
                    f,
                    r#"
                    {:indent$}{title}
                        {tlv}
                    "#,
                    "",
                    title = title,
                    tlv = x,
                    indent = 4
                )
            }
        }
    }
}
