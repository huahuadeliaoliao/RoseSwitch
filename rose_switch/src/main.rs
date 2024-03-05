mod input_method;
mod window_focus;
mod switch;
mod config;

use log::{info, error};
use env_logger;
use tokio::runtime::Runtime;
use x11rb_async::protocol::xproto::EventMask;
use x11rb_async::rust_connection::RustConnection;
use x11rb_async::connection::Connection;
use x11rb_async::protocol::xproto::ConnectionExt;

fn main() {
    // 初始化日志
    env_logger::init();

    // 读取配置文件
    let config = config::Config::new("config.toml").unwrap();

    // 创建一个异步运行时
    let mut rt = Runtime::new().unwrap();

    // 在异步运行时中执行主任务
    rt.block_on(async {
        let result = RustConnection::connect(None).await;

        match result {
            Ok((conn, screen_num, _future)) => {
                let screen = &conn.setup().roots[screen_num];

                // 订阅焦点变动事件
                conn.change_window_attributes(
                    screen.root,
                    &x11rb::protocol::xproto::ChangeWindowAttributesAux::new()
                        .event_mask(EventMask::FOCUS_CHANGE),
                )
                    .await.unwrap();

                // 进入事件循环
                loop {
                    // 获取 X11 的事件
                    let event = conn.wait_for_event().await.unwrap();
                    match event {
                        // 如果焦点变动，获取当前的焦点窗口和输入法
                        x11rb::protocol::Event::FocusIn(_) => {
                            let app_name = window_focus::get_focused_application_xorg(&conn).await.unwrap();
                            let im_name = input_method::get_current_input_method().await.unwrap();
                            info!("Focused application in Xorg: {:?}", app_name);
                            info!("Current input method: {}", im_name);

                            // 根据配置文件中的映射，判断是否需要切换输入法
                            let app_name_str = format!("{}, {}", app_name.0, app_name.1);
                            if let Some(target_im_name) = config.mappings.get(&app_name_str) {
                                // 如果当前的输入法和目标的输入法不同，调用 switch 模块来切换输入法
                                if im_name != *target_im_name {
                                    match switch::switch_input_method(target_im_name).await {
                                        Ok(()) => {
                                            info!("Switched to input method: {}", target_im_name);
                                        }
                                        Err(e) => {
                                            error!("Error: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                // 创建一个异步的 X11 连接失败，你可以在这里处理错误
                error!("Failed to create an asynchronous X11 connection: {}", e);
            }
        }
    });
}
