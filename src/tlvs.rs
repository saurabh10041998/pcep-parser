use colored::Colorize;
use indoc::writedoc;
use nom::number;
use nom::IResult;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum Tlv {
    StatefulPCECapability(StatefulPCECapabilityTLV),
    SrPCECapability(SrPCECapabilityTLV),
    Unknown(u16),
}

impl From<u16> for Tlv {
    fn from(value: u16) -> Self {
        match value {
            16 => Self::StatefulPCECapability(Default::default()),
            26 => Self::SrPCECapability(Default::default()),
            _ => Self::Unknown(value),
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
                unimplemented!("[!!] Not sure how to print TLV of type: {}", *x);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct StatefulPCECapabilityTLV {
    pub tlv_type: u16,
    pub tlv_len: u16,
    pub flag_lsp_update_capability: bool,
    pub flag_include_db_version: bool,
    pub flag_lsp_instantiate_capability: bool,
    pub flag_triggered_resync: bool,
    pub flag_delta_lsp_sync_capability: bool,
    pub flag_triggered_initial_sync: bool,
}

impl StatefulPCECapabilityTLV {
    pub fn parse_tlv(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, tlv_len) = number::streaming::be_u16(input)?;
        let (remaining, flags) = number::streaming::be_u32(remaining)?;
        let tlv = StatefulPCECapabilityTLV {
            tlv_type: 16,
            tlv_len,
            flag_lsp_update_capability: flags & 0b00_0001 == 0b00_0001,
            flag_include_db_version: flags & 0b00_0010 == 0b00_0010,
            flag_lsp_instantiate_capability: flags & 0b00_0100 == 0b00_0100,
            flag_triggered_resync: flags & 0b00_1000 == 0b00_1000,
            flag_delta_lsp_sync_capability: flags & 0b01_0000 == 0b01_0000,
            flag_triggered_initial_sync: flags & 0b10_0000 == 0b10_0000,
        };
        Ok((remaining, tlv))
    }
}

impl std::fmt::Display for StatefulPCECapabilityTLV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "[[data]]".bold();
        writedoc!(
            f,
            r#"
               {title}       
                    tlv_type                    = {tlv_type}
                    tlv_len                     = {tlv_len}
                    lsp_update_capability       = {lsp_update_capability}
                    include_db_version          = {include_db_version}
                    lsp_instatiation_capability = {lsp_instatiation_capability}
                    triggered_resync            = {triggered_resync}
                    delta_lsp_sync_capability   = {delta_lsp_sync_capability}
                    triggered_intial_resync     = {triggered_intial_resync}
            "#,
            title = title,
            tlv_type = self.tlv_type,
            tlv_len = self.tlv_len,
            lsp_update_capability = self.flag_lsp_update_capability,
            include_db_version = self.flag_include_db_version,
            lsp_instatiation_capability = self.flag_lsp_instantiate_capability,
            triggered_resync = self.flag_triggered_resync,
            delta_lsp_sync_capability = self.flag_delta_lsp_sync_capability,
            triggered_intial_resync = self.flag_triggered_initial_sync
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SrPCECapabilityTLV {
    pub tlv_type: u16,
    pub tlv_length: u16,
    pub reserved: u16,
    pub flag_limit: bool,
    pub max_sid_depth: u8,
}

impl SrPCECapabilityTLV {
    pub fn parse_tlv(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, tlv_length) = number::streaming::be_u16(input)?;
        let (remaining, reserved) = number::streaming::be_u16(remaining)?;
        let (remaining, flags) = number::streaming::be_u8(remaining)?;
        let (remaining, max_sid_depth) = number::streaming::be_u8(remaining)?;

        let tlv = SrPCECapabilityTLV {
            tlv_type: 26,
            tlv_length,
            reserved,
            flag_limit: flags & 0b1 == 0b1,
            max_sid_depth,
        };
        Ok((remaining, tlv))
    }
}

impl std::fmt::Display for SrPCECapabilityTLV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "[[data]]".bold();
        writedoc!(
            f,
            r#"
                {title}
                     tlv_type        = {tlv_type}
                     tlv_length      = {tlv_length}
                     reserved        = {reserved}
                     limit_flag      = {limit}
                     max-sid-depth   = {max_sid_depth}
            "#,
            title = title,
            tlv_type = self.tlv_type,
            tlv_length = self.tlv_length,
            reserved = self.reserved,
            limit = self.flag_limit,
            max_sid_depth = self.max_sid_depth
        )
    }
}
