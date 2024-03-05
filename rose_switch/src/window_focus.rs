use x11rb_async::rust_connection::RustConnection;
use x11rb_async::protocol::xproto::{AtomEnum, GetInputFocusRequest, GetPropertyRequest, Property};

pub async fn get_focused_application_xorg(conn: &RustConnection) -> Result<(String, String), Box<dyn std::error::Error>> {
    // 获取当前焦点窗口
    let focus = GetInputFocusRequest::default().send(conn).await?.reply().await?;
    // 获取 WM_CLASS 属性
    let prop = GetPropertyRequest {
        delete: false,
        window: focus.focus,
        property: AtomEnum::WM_CLASS.into(),
        type_: AtomEnum::STRING.into(),
        long_offset: 0,
        long_length: u32::MAX,
        ..Default::default()
    }
        .send(conn)
        .await?
        .reply()
        .await?;

    // 如果属性存在，将其转换为字符串并返回
    if let Property::Present(prop) = prop {
        let wm_class = String::from_utf8(prop.value)?;
        let mut parts = wm_class.split('\0');
        let res_name = parts.next().unwrap_or("").to_string();
        let res_class = parts.next().unwrap_or("").to_string();
        Ok((res_name, res_class))
    } else {
        Ok(("".to_string(), "".to_string()))
    }
}