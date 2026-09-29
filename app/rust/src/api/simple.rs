#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_local_ip() -> Option<String> {
    let addr = localpost::local_ip_address::local_ip().ok()?;
    Some(addr.to_string())
}
