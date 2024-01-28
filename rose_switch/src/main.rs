mod input_method;
mod window_focus;

fn main() {
    // 获取当前输入法
    match input_method::get_current_input_method() {
        Ok(im) => println!("Current input method: {}", im),
        Err(e) => eprintln!("Error: {}", e),
    }

    // 获取当前焦点所在的应用程序
    match window_focus::get_focused_application_xorg() {
        Ok(app_name) => println!("Focused application in Xorg: {}", app_name),
        Err(e) => eprintln!("Error: {}", e),
    }
}
