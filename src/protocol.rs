use crate::Result;
use lipi::{Decode, Encode};
use std::{env::consts as SYS, ops::RangeBounds};

#[repr(u8)]
#[derive(Debug, Encode, Decode)]
pub enum Message {
    Handshake(Hello) = 1,
    Read = 2,
    Write = 3,
}

#[derive(Debug, Encode, Decode)]
pub struct Hello {
    #[key = 0]
    pub protocol: String,
    #[key = 1]
    pub version: u8,
    #[key = 2]
    pub info: SystemInfo,
}

impl Hello {
    pub fn message() -> Self {
        Self {
            protocol: "localpost".into(),
            version: 10,
            info: SystemInfo::get(),
        }
    }

    pub fn check_protocol_version(&self, target: impl RangeBounds<u8>) -> Result<()> {
        if self.protocol != "localpost" {
            return Err(format!("Invalid protocol: {}", self.protocol).into());
        }
        if !target.contains(&self.version) {
            let version = format_protocol_version(self.version);
            return Err(format!("Invalid protocol version: {version}").into());
        }
        Ok(())
    }
}

#[derive(Debug, Encode, Decode)]
pub struct SystemInfo {
    #[key = 0]
    pub os_name: String,

    #[key = 1]
    pub os_version: String,

    #[key = 2]
    pub os_type: String,

    #[key = 3]
    pub architecture: String,

    #[key = 4]
    pub hostname: String,
}

impl SystemInfo {
    pub fn get() -> Self {
        use sysinfo::System;
        SystemInfo {
            os_name: System::name().unwrap_or_default(),
            os_version: System::os_version().unwrap_or_default(),
            os_type: SYS::OS.to_string(),
            architecture: SYS::ARCH.to_string(),
            hostname: System::host_name().unwrap_or_default(),
        }
    }
}

fn format_protocol_version(num: u8) -> String {
    let major = num / 10;
    let minor = num % 10;
    format!("{}.{}", major, minor)
}
