use x11rb_async::rust_connection::RustConnection;
use x11rb_async::protocol::xproto::{Atom, AtomEnum, GetPropertyReply, Window, ConnectionExt};
use std::error::Error;

pub async fn get_focused_application_xorg(conn: &RustConnection, screen_num: usize) -> Result<(String, String), Box<dyn Error>> {
    let focus = conn.get_input_focus().await?.reply().await?;
    let window = focus.focus;

    if window == 0 {
        return Ok(("".to_string(), "".to_string()));
    }

    let wm_class = get_wm_class(conn, window).await?;

    Ok(wm_class)
}

async fn get_wm_class(conn: &RustConnection, window: Window) -> Result<(String, String), Box<dyn Error>> {
    let wm_class_atom = intern_atom(conn, b"WM_CLASS").await?;
    let reply = get_property(conn, false, window, wm_class_atom, AtomEnum::STRING.into(), 0, u32::MAX).await?;

    let wm_class = String::from_utf8(reply.value)?;
    let mut parts = wm_class.split('\0');
    let res_name = parts.next().unwrap_or("").to_string();
    let res_class = parts.next().unwrap_or("").to_string();
    Ok((res_name, res_class))
}

async fn intern_atom(conn: &RustConnection, name: &[u8]) -> Result<Atom, Box<dyn Error>> {
    let reply = conn.intern_atom(false, name).await?.reply().await?;
    Ok(reply.atom)
}

async fn get_property(
    conn: &RustConnection,
    delete: bool,
    window: Window,
    property: Atom,
    type_: Atom,
    long_offset: u32,
    long_length: u32,
) -> Result<GetPropertyReply, Box<dyn Error>> {
    let reply = conn
        .get_property(delete, window, property, type_, long_offset, long_length)
        .await?
        .reply()
        .await?;
    Ok(reply)
}
