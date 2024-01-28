[中文版](./README_zh.md) | English

# RoseSwitch

## Introduction

RoseSwitch, developed in Rust, is designed for intelligent switching between input method frameworks and the current window focus in Linux environments. This project is currently in the testing phase, with plans to implement a user-friendly visual interface using Tauri in the future.

## Features

- **Current Input Method Retrieval**: Utilizes commands like `fcitx5-remote` to obtain the status of the currently active input method.
- **Focused Application Retrieval**: Acquires the name of the application in focus of the current window using `xdotool` and `xprop` commands.
- **Smart Switching Between Input Methods and Application Focus**: Aims to intelligently switch between different input methods based on the current window focus, catering to multi-language input needs.

## Future Plans

- To implement more accurate logic for switching between input methods and application focus.
- To build a simple yet efficient user interface using Tauri, enhancing the user experience.
