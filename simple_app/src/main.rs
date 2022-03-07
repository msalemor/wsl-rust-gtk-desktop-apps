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