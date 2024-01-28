use std::process::Command;
use std::str;

pub fn get_current_input_method() -> Result<String, String> {
    let output = Command::new("fcitx5-remote")
        .arg("-n")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        Ok(stdout.trim().to_string())
    } else {
        Err("Failed to get current input method".to_string())
    }
}
