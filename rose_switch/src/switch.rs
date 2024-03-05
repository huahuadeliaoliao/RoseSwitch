use dbus::blocking::Connection;
use std::time::Duration;

pub async fn switch_input_method(im_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个 dbus 的连接
    let conn = Connection::new_session()?;
    // 创建一个代理对象，指定 fcitx5 的服务名、对象路径和接口名
    let proxy = conn.with_proxy("org.fcitx.Fcitx5", "/controller", Duration::from_millis(5000));
    // 调用 SetCurrentIM 方法，切换到指定的输入法
    let (): () = proxy.method_call("org.fcitx.Fcitx.Controller1", "SetCurrentIM", (im_name,))?;
    Ok(())
}