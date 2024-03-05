use dbus::blocking::Connection;
use std::time::Duration;

pub fn get_current_input_method() -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy("org.fcitx.Fcitx5", "/controller", Duration::from_millis(5000));
    let (current_im,): (String,) = proxy.method_call("org.fcitx.Fcitx.Controller1", "CurrentInputMethod", ())?;
    Ok(current_im)
}
