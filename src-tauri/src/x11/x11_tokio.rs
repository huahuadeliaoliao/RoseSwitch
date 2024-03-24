use x11rb_async::rust_connection::RustConnection;
use x11rb_async::protocol::xproto::{EventMask, ConnectionExt};
use x11rb_async::connection::Connection;
use log::{info, trace};
use simple_logger::SimpleLogger;
use std::error::Error;

mod window_focus;
mod input_method;
mod switch;
mod config;

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
    conn.change_window_attributes(root, &{
        let mut cw = x11rb_async::protocol::xproto::ChangeWindowAttributesAux::default();
        cw.event_mask = Some(EventMask::ENTER_WINDOW | EventMask::LEAVE_WINDOW | EventMask::FOCUS_CHANGE | EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY | EventMask::VISIBILITY_CHANGE | EventMask::RESIZE_REDIRECT );
        cw
    }).await?;
    conn.flush().await?;
    info!("事件订阅完成");

    loop {
        info!("等待事件");
        let event = conn.wait_for_event().await?;
        trace!("收到事件: {:?}", event);

        match event {
            x11rb_async::protocol::Event::EnterNotify(_)
            | x11rb_async::protocol::Event::LeaveNotify(_)
            | x11rb_async::protocol::Event::CreateNotify(_)
            | x11rb_async::protocol::Event::MapNotify(_)
            | x11rb_async::protocol::Event::XinputEnter(_)
            | x11rb_async::protocol::Event::XinputFocusIn(_)
            | x11rb_async::protocol::Event::XinputFocusOut(_)
            | x11rb_async::protocol::Event::FocusIn(_)
            | x11rb_async::protocol::Event::FocusOut(_) => {
                info!("鼠标进入窗口事件或焦点改变或窗口创建事件");
                let (res_name, res_class) = window_focus::get_focused_application_xorg(&conn, screen_num).await?;
                info!("当前焦点应用: res_name = {}, res_class = {}", res_name, res_class);
        
                if let Some(im_name) = config.mappings.get(&res_name) {
                    info!("there");
                    let current_im = input_method::get_current_input_method().await?;
                    if &current_im != im_name {
                        info!("切换输入法到: {}", im_name);
                        switch::switch_input_method(im_name).await?;
                    }
                }
            }
            _ => {
                info!("其他事件");
            }
        }        
    }
}
