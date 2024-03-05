use dbus::blocking::Connection;
use std::time::Duration;

pub async fn get_current_input_method() -> Result<String, Box<dyn std::error::Error>> {
    // 创建一个 dbus 的连接
    let conn = Connection::new_session()?;
    // 创建一个代理对象，指定 fcitx5 的服务名、对象路径和接口名
    let proxy = conn.with_proxy("org.fcitx.Fcitx5", "/controller", Duration::from_millis(5000));
    // 调用 CurrentInputMethod 方法，获取当前的输入法
    let (current_im,): (String,) = proxy.method_call("org.fcitx.Fcitx.Controller1", "CurrentInputMethod", ())?;
    Ok(current_im)
}
