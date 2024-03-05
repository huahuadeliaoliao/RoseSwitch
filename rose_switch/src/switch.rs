use dbus::blocking::Connection;
use std::time::Duration;

pub fn switch_input_method(im_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy("org.fcitx.Fcitx5", "/controller", Duration::from_millis(5000));
    let (): () = proxy.method_call("org.fcitx.Fcitx.Controller1", "SetCurrentIM", (im_name,))?;
    Ok(())
}
