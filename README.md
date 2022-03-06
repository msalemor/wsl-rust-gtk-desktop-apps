# Development of desktop apps in Windows 11 Ubuntu 20.04 WSL2, Rust and GTK3
Updated: 2022-03-06 11:00 pm

## Requirements

- Windows 11 Build 22000 or higher
- Ubuntu 20.04 WSL2 updated to support Linux GUI apps
  - https://docs.microsoft.com/en-us/windows/wsl/tutorials/gui-apps

> Note: install gedit in WSL2 (```sudo apt install gedit -y```) and <br/>
> make sure you can see it (```gedit ~/.profile```) on the Windoes 11 desktop

## Windows 11 development tools

- VS Code
- Extensions:
  - Rust Analyzer
- Terminal

## Development tools in WSL2 Ubuntu 20.04

From Terminal open Ubuntu and:

- Install the build essentials and git
  - ```sudo apt install build-essentials git -y```
- Install Rust  
  - ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
> Reference: <br/>
> https://www.rust-lang.org/tools/install <br/>
> Note: <br/>
> As of the writing of this repo Rust Version 1.56.0 did not compile this code. I had to update rust to 1.59.0.</br>
> To update rust execute: <br/>
> ```rustup update```
- Install glade (GUI desiner graphical tool)
  - ```sudo apt install glade -y```
- Install the GTK development libraries
  - ```sudo apt instal libgtk-3-dev -y```
  
## Basic app

### Create a cargo package and open it in VS Code

```bash
cargo new first_desktop_app
cd gui_app
code .
```

### Edit the ```cargo.toml```

```toml
[package]
name = "first_desktop_app"
version = "0.1.0"
edition = "2021"

[dependencies]
gtk = "0.15.4"
```

### Edit the ```src/main.rs```
```rust
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("org.example.FirstDesktopApp")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let main_window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Don't forget to make all widgets visible.
        main_window.show_all();
    });

    app.run();
}
```

## Run the app

- From the VS Code Terminal type: ```cargo run```
