use crate::tlvs::tlv_set::{
    Ipv4LSPIndetifiersTLV, SrPCECapabilityTLV, StatefulPCECapabilityTLV, SymbolicPathNameTLV,
    UnknownTLV,
};
use colored::Colorize;
use indoc::writedoc;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum Tlv {
    StatefulPCECapability(StatefulPCECapabilityTLV),
    SrPCECapability(SrPCECapabilityTLV),
    SymbolicPathName(SymbolicPathNameTLV),
    Ipv4LSPIndetifiers(Ipv4LSPIndetifiersTLV),
    Unknown(UnknownTLV),
}

impl From<u16> for Tlv {
    fn from(value: u16) -> Self {
        match value {
            16 => Self::StatefulPCECapability(Default::default()),
            17 => Self::SymbolicPathName(Default::default()),
            18 => Self::Ipv4LSPIndetifiers(Default::default()),
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
            Self::SymbolicPathName(spn) => {
                let title = "==[SYMBOLIC-PATH-NAME TLV]==".green().bold();
                writedoc!(
                    f,
                    r#"
                    {:indent$}{title}
                        {tlv}
                    "#,
                    "",
                    title = title,
                    tlv = spn,
                    indent = 4
                )
            }
            Self::Ipv4LSPIndetifiers(ipv4lspi) => {
                let title = "==[IPV4-LSP-IDENTIFIERS TLV]==".green().bold();
                writedoc!(
                    f,
                    r#"
                    {:indent$}{title}
                        {tlv}
                    "#,
                    "",
                    title = title,
                    tlv = ipv4lspi,
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
