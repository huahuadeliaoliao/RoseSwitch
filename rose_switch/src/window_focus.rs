use std::process::Command;
use std::str;

pub fn get_focused_application_xorg() -> Result<String, String> {
    // 获取当前焦点窗口的 ID
    let window_id_output = Command::new("xdotool")
        .arg("getwindowfocus")
        .output();

    let window_id = match window_id_output {
        Ok(output) => {
            if output.status.success() {
                str::from_utf8(&output.stdout).unwrap().trim().to_string()
            } else {
                return Err("Failed to get focused window ID in Xorg".to_string());
            }
        }
        Err(_) => return Err("xdotool command failed".to_string()),
    };

    // 使用 xprop 获取窗口类名
    let app_name_output = Command::new("xprop")
        .arg("-id")
        .arg(window_id)
        .arg("WM_CLASS")
        .output();

    match app_name_output {
        Ok(output) => {
            if output.status.success() {
                let output_str = str::from_utf8(&output.stdout).unwrap();
                // 解析 WM_CLASS 输出
                if let Some(start) = output_str.find('"') {
                    let end = output_str[start + 1..].find('"').unwrap_or(output_str.len());
                    Ok(output_str[start + 1..start + 1 + end].to_string())
                } else {
                    Err("Failed to parse WM_CLASS property".to_string())
                }
            } else {
                Err("Failed to get application name in Xorg".to_string())
            }
        }
        Err(_) => Err("xprop command failed".to_string()),
    }
}
