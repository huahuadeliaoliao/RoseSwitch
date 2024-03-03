use dbus::blocking::Connection;
use std::time::Duration;

pub fn switch_input_method(im_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 首先，打开到会话总线的连接
    let conn = Connection::new_session()?;

    // 其次，创建一个包装结构，使得向特定目标和路径发送方法调用变得容易
    let proxy = conn.with_proxy("org.fcitx.Fcitx5", "/controller", Duration::from_millis(5000));

    // 现在进行方法调用。SetCurrentIM 方法调用有一个输入参数，即一个字符串，没有输出参数
    // 因此，输入是一个单元素元组 "(im_name,)"，输出是一个空元组 "()"
    let (): () = proxy.method_call("org.fcitx.Fcitx.Controller1", "SetCurrentIM", (im_name,))?;

    Ok(())
}
