# Development of desktop apps in Windows 11 Ubuntu 20.04 WSL2, Rust and GTK3
Updated: 2022-03-06 11:00 pm

## Recognition

The content of this repo is based on the following tutorial by Brian Floersch:

https://blog.sb1.io/getting-started-with-rust-and-gtk/

> **Note:**: as of the writing of this document, the intructions in this document were not working for me. I have sinced debugged the code and got it running.

## Objective

In this blog, I am providing specific instructions to setup Windows 11, WSL, Rust, and the development tools to be able to develop desktop apps for Linux.

## Requirements

- Windows 11 Build 22000 or higher
- Ubuntu 20.04 WSL2 updated to support Linux GUI apps
  - https://docs.microsoft.com/en-us/windows/wsl/tutorials/gui-apps

> Note: install gedit in WSL2 (```sudo apt install gedit -y```) and <br/>
> make sure you can open a test file using gedit (i.e. ```gedit ~/.profile```) on the Windoes 11 desktop 

## Windows 11 development tools

- VS Code
- Extensions:
  - Rust Analyzer
- Terminal

## Development tools in WSL2 Ubuntu 20.04

From Terminal open Ubuntu and:

- Install the build essentials and git
  - ```sudo apt install build-essentials git -y```
  - > **Note:** I like installing the latest GIT version following [these instructions](https://www.linuxcapable.com/how-to-install-and-update-latest-git-on-ubuntu-20-04/).
- Install Rust  
  - ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
> Reference: <br/>
> https://www.rust-lang.org/tools/install <br/>
> Note: <br/>
> As of the writing of this repo Rust Version 1.56.0 did not compile this code. I had to update rust to 1.59.0.</br>
> To check the version type: <br/>
> ```rustc --version``` <br/>
> To update rust execute: <br/>
> ```rustup update```
- Install glade (the RAD interface tool)
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

        // Show the window.
        main_window.show_all();
    });

    app.run();
}
```

### Run the app

- From the VS Code Terminal type: ```cargo run```

## Development with Glade

### General Information

Glade is a tool used to generate GTK inferaces. The code is save as XML. More informaciton can be found [here](https://glade.gnome.org/).

## Sample GTK app with Glade

## main.rs

```rust
use gtk::prelude::*;
use gtk::*;
use log::info;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    info!("Starting App");

    let application = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    application.connect_activate(move |app| {
        // Load the compiled resource bundle
        let resources_bytes = include_bytes!("../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&resources_bytes[..]);
        let res = gio::Resource::from_data(&resource_data).unwrap();
        gio::resources_register(&res);

        // Load the CSS
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/org/example/Example/style.css");
        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        info!("{:?}", res);

        // Load the window UI
        let builder = Builder::from_resource("/org/example/Example/main_window.glade");
        //let builder = Builder::from_file("resources/main_window.glade");

        info!("{:?}", builder);

        info!("# of objects: {}", builder.objects().len());
        let mut count = 1;
        for obj in builder.objects() {
            info!(
                "{}: {:?} ",
                count,
                obj
            );
            count += 1;        
        }

        // Get a reference to the window
        let win_name = "main_window";
        let msg = &*format!("Couldn't get window: {}", &win_name);
        let window : ApplicationWindow = builder.object(win_name).expect(msg);
        info!("{:?}", win_name);

        window.set_application(Some(app));

        // Show the UI
        info!("Showing window");
        window.show_all();
    });

    // Run the application and start the event loop
    info!("Launching app and processing event loop");
    application.run();
}
```
