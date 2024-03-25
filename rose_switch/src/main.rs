use std::pin::Pin;
use std::future::Future;
use x11rb_async::connection::Connection;
use x11rb_async::protocol::xproto::{EventMask, ConnectionExt, ChangeWindowAttributesAux};
use x11rb_async::rust_connection::RustConnection;
use log::{info, trace};
use simple_logger::SimpleLogger;
use std::error::Error;

mod window_focus;
mod switch;
mod config;

async fn set_event_mask_for_window(conn: &RustConnection, window: u32) -> Result<(), Box<dyn Error>> {
    let mut attributes = ChangeWindowAttributesAux::default();
    attributes.event_mask = Some(
        EventMask::FOCUS_CHANGE
    );
    conn.change_window_attributes(window, &attributes).await?;
    Ok(())
}

fn set_event_mask_for_all_windows(conn: &'_ RustConnection, window: u32) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + '_>> {
    Box::pin(async move {
        set_event_mask_for_window(conn, window).await?;

        let tree = conn.query_tree(window).await?.reply().await?;
        for child in tree.children {
            set_event_mask_for_all_windows(conn, child).await?;
        }
        Ok(())
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();
    info!("日志初始化完成");

    let config = config::Config::new("config.toml")?;
    info!("配置文件读取完成");

    let (conn, screen_num, driver) = RustConnection::connect(None).await?;
    tokio::spawn(driver);
    info!("与X11服务器的异步连接建立完成");

    let root = conn.setup().roots[screen_num].root;
    set_event_mask_for_all_windows(&conn, root).await?;
    info!("事件订阅完成");

    loop {
        info!("等待事件");
        let event = conn.wait_for_event().await?;
        trace!("收到事件: {:?}", event);

        match event {
            x11rb_async::protocol::Event::FocusIn(_)=> {
                info!("焦点改变事件");
                let (res_name, res_class) = window_focus::get_focused_application_xorg(&conn, screen_num).await?;
                info!("当前焦点应用: res_name = {}, res_class = {}", res_name, res_class);
        
                if let Some(im_name) = config.mappings.get(&res_name) {
                    info!("切换输入法到: {}", im_name);
                    switch::switch_input_method(im_name).await?;
                }                
            }
            _ => {
                info!("其他事件");
            }
        }        
    }
}
