extern crate dbus;

use dbus::blocking::Connection;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到 DBus 会话总线
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy("org.freedesktop.IBus", "/org/freedesktop/IBus", Duration::from_millis(5000));

    // 调用 ListEngines 方法获取输入法列表
    let engines: (Vec<dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>,) = proxy.method_call("org.freedesktop.IBus", "ListEngines", ())?;
    for engine in engines.0 {
        println!("找到输入法: {:?}", engine);
    }

    // 调用 GetCurrentEngine 方法获取当前输入法
    let current_engine: (dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>,) = proxy.method_call("org.freedesktop.IBus", "GetCurrentEngine", ())?;
    println!("当前输入法: {:?}", current_engine.0);

    Ok(())
}