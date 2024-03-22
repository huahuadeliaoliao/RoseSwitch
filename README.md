[中文版](./README_zh.md) | English

# RoseSwitch

## Introduction

RoseSwitch, developed in Rust, is designed for intelligent switching specifically for fcitx5 input methods and the current window focus in Linux environments. This project is currently in the development phase, with plans to implement a user-friendly visual interface using Tauri in the future.

## Features

- **Current Input Method Retrieval**: Retrieves the status of the currently active fcitx5 input method using dbus.
- **Focused Application Retrieval**: Acquires the name of the application in focus of the current window using the X11 protocol, through the asynchronous `x11rb-async` library and the `tokio` runtime for handling asynchronous operations.
- **Smart Switching Between Input Methods and Application Focus**: Utilizes the asynchronous capabilities of `tokio` to intelligently switch between different fcitx5 input methods based on the current window focus, catering to multi-language input needs.

## Future Plans

- To implement more accurate logic for switching between fcitx5 input methods and application focus.
- To build a simple yet efficient user interface using Tauri, enhancing the user experience.
