//! Backup (`vzdump`) related request types.

use std::fmt;

use crate::params::PveParams;

#[derive(Debug, Clone, Copy)]
pub enum VzdumpMode {
    Snapshot,
    Suspend,
    Stop,
}

impl VzdumpMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Suspend => "suspend",
            Self::Stop => "stop",
        }
    }
}

impl fmt::Display for VzdumpMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VzdumpCompress {
    None,
    Gzip,
    Lzo,
    Zstd,
}

impl VzdumpCompress {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "0",
            Self::Gzip => "gzip",
            Self::Lzo => "lzo",
            Self::Zstd => "zstd",
        }
    }
}

impl fmt::Display for VzdumpCompress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MailNotification {
    Always,
    Failure,
}

impl MailNotification {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Failure => "failure",
        }
    }
}

impl fmt::Display for MailNotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Default)]
pub struct VzdumpRequest {
    pub all: Option<bool>,
    pub vmid: Option<String>,
    pub mode: Option<VzdumpMode>,
    pub storage: Option<String>,
    pub compress: Option<VzdumpCompress>,
    pub mailnotification: Option<MailNotification>,
    pub mailto: Option<String>,
    pub notes_template: Option<String>,
    pub remove: Option<bool>,
    pub stopwait: Option<u64>,
    pub extra: PveParams,
}

impl VzdumpRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();

        if let Some(all) = self.all {
            params.insert_bool("all", all);
        }

        params.insert_opt("vmid", self.vmid.clone());
        params.insert_opt("mode", self.mode.map(|v| v.to_string()));
        params.insert_opt("storage", self.storage.clone());
        params.insert_opt("compress", self.compress.map(|v| v.to_string()));
        params.insert_opt(
            "mailnotification",
            self.mailnotification.map(|v| v.to_string()),
        );
        params.insert_opt("mailto", self.mailto.clone());
        params.insert_opt("notes-template", self.notes_template.clone());
        if let Some(remove) = self.remove {
            params.insert_bool("remove", remove);
        }
        params.insert_opt("stopwait", self.stopwait.map(|v| v.to_string()));

        params.extend(&self.extra);
        params
    }
}
