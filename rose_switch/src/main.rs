extern crate dbus;

use dbus::{blocking::Connection};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy("org.freedesktop.IBus", "/org/freedesktop/IBus", Duration::from_millis(5000));

    let current_engine: (String,) = proxy.method_call("org.freedesktop.IBus", "CurrentEngine", ())?;
    println!("当前输入法引擎: {}", current_engine.0);

    Ok(())
}

