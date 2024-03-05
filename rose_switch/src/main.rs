mod input_method;
mod window_focus;
mod switch;
mod config;

use std::sync::mpsc;
use std::thread;
use log::{info, error};
use env_logger;

fn main() {
    // 初始化日志
    env_logger::init();

    // 读取配置文件
    let config = config::Config::new("config.toml").unwrap();

    // 创建一个通道
    let (tx, rx) = mpsc::channel();

    // 在新的线程中运行无限循环来监测X11的信号
    thread::spawn(move || {
        loop {
            match window_focus::get_focused_application_xorg() {
                Ok(app_name) => {
                    info!("Focused application in Xorg: {:?}", app_name);
                    tx.send(app_name).unwrap();
                },
                Err(e) => {
                    error!("Error: {}", e);
                },
            }
            // 暂停一段时间，以减少CPU使用率
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // 在主线程中处理接收到的消息
    loop {
        let app_name = rx.recv().unwrap();
        let app_name_str = format!("{}, {}", app_name.0, app_name.1);
        if let Some(im_name) = config.mappings.get(&app_name_str) {
            match switch::switch_input_method(im_name) {
                Ok(()) => {
                    info!("Switched to input method: {}", im_name);
                },
                Err(e) => {
                    error!("Error: {}", e);
                },
            }
        }
    }
}
