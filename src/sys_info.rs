#[derive(Debug)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub os_type: String,
    pub architecture: String,
    pub hostname: String,
}

pub fn get_system_info() -> SystemInfo {
    use sysinfo::System;
    SystemInfo {
        os_name: System::name().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        os_type: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        hostname: System::host_name().unwrap_or_default(),
    }
}
